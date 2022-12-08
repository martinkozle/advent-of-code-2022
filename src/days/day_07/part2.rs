use std::collections::HashMap;

use itertools::Itertools;

struct File {
    size: u32,
}

struct Directory {
    children_paths: Vec<String>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
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
    fn new() -> FileSystem {
        let mut file_system = FileSystem {
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

    fn ls(&self, dir: &str) -> Option<&Vec<String>> {
        let abs_dir = if dir.starts_with('/') {
            dir.to_string()
        } else {
            format!("{}{}/", self.current_working_directory, dir)
        };
        match self.file_table.get(&abs_dir)? {
            FileType::D(directory) => Some(&directory.children_paths),
            _ => None,
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
        match self
            .file_table
            .get_mut(&self.current_working_directory)
            .unwrap()
        {
            FileType::D(ref mut directory) => directory.children_paths.push(new_path),
            _ => unreachable!(),
        };
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
        match self
            .file_table
            .get_mut(&self.current_working_directory)
            .unwrap()
        {
            FileType::D(ref mut directory) => directory.children_paths.push(new_path),
            _ => unreachable!(),
        };
    }

    fn size(&self, path: &str) -> u32 {
        match self.file_table.get(path) {
            Some(FileType::F(file)) => file.size,
            Some(FileType::D(directory)) => directory
                .children_paths
                .iter()
                .map(|path| self.size(path))
                .sum(),
            None => panic!("Invalid path: {}", path),
        }
    }

    fn get_sizes_filtered<P>(&self, predicate: P) -> Vec<u32>
    where
        P: Fn(u32) -> bool,
    {
        let mut stack = vec!["/"];
        let mut total = Vec::<u32>::new();

        while !stack.is_empty() {
            let dir = stack.pop().unwrap();
            let size = self.size(dir);
            if predicate(size) {
                total.push(size);
                for path in self.ls(dir).expect("dir should be a valid dir path").iter() {
                    if let FileType::D(_) = self.file_table.get(path).unwrap() {
                        stack.push(path)
                    }
                }
            }
        }
        total
    }
}

pub fn solve(input: String) -> String {
    let mut file_system = FileSystem::new();
    for block in input.split("$ ").skip(1) {
        let mut lines = block.lines();
        let command = lines.next().unwrap().trim();
        let output = lines.collect::<Vec<_>>();
        match command.split(' ').collect::<Vec<_>>()[..] {
            ["cd", dir] => {
                file_system.cd_mkdir(dir);
            }
            ["ls"] => {
                for (a, b) in output.iter().map(|line| {
                    line.split_once(' ')
                        .expect("Each line of ls output should have 2 words")
                }) {
                    match a {
                        "dir" => file_system.mkdir(b),
                        size if size.parse::<u32>().is_ok() => {
                            file_system.touch(b, size.parse().unwrap())
                        }
                        _ => panic!("Invalid ls output: {} {}", a, b),
                    };
                }
            }
            _ => panic!("Invalid command: {}", command),
        };
    }
    let total_free_space = 70000000 - file_system.size("/");
    file_system
        .get_sizes_filtered(|size| total_free_space + size >= 30000000)
        .iter()
        .min()
        .unwrap()
        .to_string()
}
