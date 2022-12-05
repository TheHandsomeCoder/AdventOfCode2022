#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("../input.txt");
    let sum_of_intersections: i32 = input
    .lines()
       .array_chunks::<3>()
       .map(|[first, second, third]| {           
          return first
            .chars()
            .into_iter()
            .filter(|item| second.contains(*item) && third.contains(*item))
            .collect::<Vec<char>>()[0];

        })
        .map(|char| priority(char))
        .sum();

    print!("{}", sum_of_intersections)
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        (c as i32) - 96
    } else {
        (c as i32) - 38
    }
}
