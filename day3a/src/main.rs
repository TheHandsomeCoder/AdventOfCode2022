use std::{collections::HashSet, iter::FromIterator};

fn main() {
    let input = include_str!("../input.txt");
    let sum_of_intersections: i32 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            let a: HashSet<char> = HashSet::from_iter(first.chars().into_iter());
            let b: HashSet<char> = HashSet::from_iter(second.chars().into_iter());

            return a
                .intersection(&b)
                .into_iter()
                .copied()
                .collect::<Vec<char>>();
        })
        .map(|chars| chars[0])
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
