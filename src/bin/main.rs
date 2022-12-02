fn main() {
    let input = include_str!("./input.txt");
    
    let lines = input.split("\n\n");
    
    let mut lines_parsed: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
        .collect();

        lines_parsed.sort_by(|a, b| b.cmp(a));

    println!("{:?}", lines_parsed[0]);

    println!("{:?}", 
    lines_parsed.iter().take(3).sum::<u32>()
    );

}
