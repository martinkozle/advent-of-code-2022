pub fn solve(input: String) -> String {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| range.split('-').map(|num| num.parse::<u32>().unwrap()))
        })
        .map(|mut pair| (pair.next().unwrap(), pair.next().unwrap()))
        .map(|(mut range1, mut range2)| {
            (
                (range1.next().unwrap(), range1.next().unwrap()),
                (range2.next().unwrap(), range2.next().unwrap()),
            )
        })
        .filter(|((fr1, to1), (fr2, to2))| fr1 <= to2 && to1 >= fr2)
        .count()
        .to_string()
}
