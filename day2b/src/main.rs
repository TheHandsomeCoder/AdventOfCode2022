//Opponent - A for Rock, B for Paper, and C for Scissors
//Player - X lose, Y draw, and Z win. !"
//Player 1 point for Rock, 2 points for Paper, and 3 points for Scissors
//Player 0 points for loss, 3 points for draw,  and 6 points for victory

fn main() {
    let input = include_str!("../input.txt");
    let points: i32 = input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|moves| {
            let play= match moves {
                (_, "X") => 0,
                (_, "Y") => 3,
                (_, "Z") => 6,
                _ => unreachable!()
            };

            let result = match moves {
                ("A","Y") | ("B","X") | ("C","Z") => 1,
                ("A","X") | ("B","Z") | ("C","Y") => 3,                 
                ("A","Z") | ("B","Y") | ("C","X") => 2,
                _ => unreachable!()
                
            };

            return play + result;
        })
        .sum();

    println!("{}", points);
}
