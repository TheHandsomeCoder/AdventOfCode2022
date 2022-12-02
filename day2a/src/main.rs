//Opponent - A for Rock, B for Paper, and C for Scissors
//Player - X for Rock, Y for Paper, and Z for Scissors
//Player 1 point for Rock, 2 points for Paper, and 3 points for Scissors
//Player 0 points for loss, 3 points for draw,  and 6 points for victory


fn main() {
    let input = include_str!("../input.txt");
    let points: i32 = input.lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|moves| {
            let play= match moves {
                (_, "X") => 1,
                (_, "Y") => 2,
                (_, "Z") => 3,
                _ => unreachable!()
            };

            let result = match moves {
                ("A","Y") | ("B","Z") | ("C","X") => 6, //Player Victory = 6
                ("A","X") | ("B","Y") | ("C","Z") => 3, //Player Draw = 3
                _ => 0 // Player loss = 0
            };

            return play + result;
        })
        .sum();

    println!("{}", points);
        
}
