use std::collections::HashMap;

use anyhow::{anyhow, bail, Context};
use itertools::Itertools;

struct File {
    size: u32,
}

struct Directory {
    children_paths: Vec<String>,
}

impl Directory {
    fn new() -> Self {
        Self {
            children_paths: vec![],
        }
    }
}

enum FileType {
    F(File),
    D(Directory),
}

struct FileSystem {
    file_table: HashMap<String, FileType>,
    current_working_directory: String,
}

impl FileSystem {
    fn new() -> Self {
        let mut file_system = Self {
            file_table: HashMap::new(),
            current_working_directory: "/".to_string(),
        };
        file_system
            .file_table
            .insert("/".to_string(), FileType::D(Directory::new()));
        file_system
    }

    fn cd(&mut self, dir: &str) -> bool {
        let new_working_directory = if dir.starts_with('/') {
            dir.to_string()
        } else if dir == ".." {
            let mut split = self.current_working_directory.split('/');
            split.next_back();
            split.next_back();
            format!("{}/", split.join("/"))
        } else {
            format!("{}{}/", self.current_working_directory, dir)
        };
        if self.file_table.contains_key(&new_working_directory) {
            self.current_working_directory = new_working_directory;
            true
        } else {
            false
        }
    }

    fn ls(&self, dir: &str) -> anyhow::Result<&Vec<String>> {
        let abs_dir = if dir.starts_with('/') {
            dir.to_string()
        } else {
            format!("{}{}/", self.current_working_directory, dir)
        };
        match self
            .file_table
            .get(&abs_dir)
            .ok_or_else(|| anyhow!("invalid dir path: {}", dir))?
        {
            FileType::D(directory) => Ok(&directory.children_paths),
            FileType::F(_) => Err(anyhow!("tried to ls into a file: `{}`", abs_dir)),
        }
    }

    fn get_current_working_directory_mut(&mut self) -> &mut Directory {
        match self
            .file_table
            .get_mut(&self.current_working_directory)
            .expect("valid filesystem table and current working directory")
        {
            FileType::D(ref mut directory) => directory,
            FileType::F(_) => panic!("current working directory was a file"),
        }
    }

    fn mkdir(&mut self, dir: &str) {
        assert!(
            !dir.starts_with('/'),
            "Making absolute directories isn't supported"
        );
        let new_path = format!("{}{}/", self.current_working_directory, dir);
        if self.file_table.contains_key(&new_path) {
            return;
        }
        self.file_table
            .insert(new_path.clone(), FileType::D(Directory::new()));
        self.get_current_working_directory_mut()
            .children_paths
            .push(new_path);
    }

    fn cd_mkdir(&mut self, dir: &str) {
        if !self.cd(dir) {
            self.mkdir(dir);
            self.cd(dir);
        }
    }

    fn touch(&mut self, file: &str, size: u32) {
        let new_path = format!("{}{}", self.current_working_directory, file);
        self.file_table
            .insert(new_path.clone(), FileType::F(File { size }));
        self.get_current_working_directory_mut()
            .children_paths
            .push(new_path);
    }

    fn size(&self, path: &str) -> anyhow::Result<u32> {
        match self.file_table.get(path) {
            Some(FileType::F(file)) => Ok(file.size),
            Some(FileType::D(directory)) => Ok(directory
                .children_paths
                .iter()
                .map(|path| self.size(path))
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .sum()),
            None => Err(anyhow!("Invalid path: `{}`", path)),
        }
    }

    fn get_total_size_filtered<P>(&self, predicate: P) -> anyhow::Result<u32>
    where
        P: Fn(u32) -> bool,
    {
        let mut stack = vec!["/"];
        let mut total: u32 = 0;

        while let Some(dir) = stack.pop() {
            let size = self.size(dir)?;
            if predicate(size) {
                total += size;
            }
            for path in self.ls(dir)? {
                if let Some(FileType::D(_)) = self.file_table.get(path) {
                    stack.push(path)
                }
            }
        }
        Ok(total)
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let mut file_system = FileSystem::new();
    for block in input.split("$ ").skip(1) {
        let mut lines = block.lines();
        let command = lines
            .next()
            .context("block should contain at least one line")?
            .trim();
        let output = lines.collect::<Vec<_>>();
        match command.split(' ').collect_vec()[..] {
            ["cd", dir] => {
                file_system.cd_mkdir(dir);
            }
            ["ls"] => {
                for (a, b) in output
                    .iter()
                    .map(|line| {
                        line.split_once(' ').ok_or_else(|| {
                            anyhow!("invalid ls line, should have 2 words: `{}`", line)
                        })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?
                {
                    match a {
                        "dir" => file_system.mkdir(b),
                        size if size.parse::<u32>().is_ok() => {
                            file_system.touch(b, size.parse().unwrap())
                        }
                        _ => bail!("invalid ls output: `{}` `{}`", a, b),
                    };
                }
            }
            _ => bail!("invalid command: `{}`", command),
        };
    }
    Ok(file_system
        .get_total_size_filtered(|size| size <= 100000)?
        .to_string())
}
