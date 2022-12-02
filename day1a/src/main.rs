
/* 
Input is values grouped per elf, blank line = new elf. Find elf carrying most calories (values added together)
*/
fn main() {
    let file = include_str!("../input.txt");
    let values: Vec<i32> = file.split("\n\n")
        .map(|group| 
            group.lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>()
        ).collect();
    let max = values.iter().max().unwrap();
    println!("max: {}", max);
}

