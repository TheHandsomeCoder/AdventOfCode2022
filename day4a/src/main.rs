use std::f32::consts::E;

fn main() {
    let input = include_str!("../input.txt").lines();
    let count: i32 = input
        .map(|line| line.split_once(",").unwrap())
        .map(|(left, right)| return (left.split_once("-").unwrap(), right.split_once("-").unwrap()))
        .map(|((a, b), (c, d))| {
            let e = a.parse::<i32>().unwrap();
            let f = b.parse::<i32>().unwrap();
            let g = c.parse::<i32>().unwrap();
            let h = d.parse::<i32>().unwrap();

            if e <= g && f >= h {
                return 1;
            } else if g <= e && h >= f {
                return 1;
            }
            return 0;
        })
        .sum();

    print!("{}", count)
}
