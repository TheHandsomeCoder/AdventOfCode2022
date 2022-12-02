/* 
Input is values grouped per elf, blank line = new elf. Find top three elves carrying most calories
*/
fn main() {
    let file = include_str!("../input.txt");
    let groups = file.split("\n\n");
    let mut values: Vec<i32> = groups.map(|group| 
            group.lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>()
        ).collect();
   
    values.sort();
    values.reverse();
    values.truncate(3);
  

    let max = values.iter().sum::<i32>();
    println!("max: {:#?}", max);
}

