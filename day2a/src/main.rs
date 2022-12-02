// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
// 1 for Rock, 2 for Paper, and 3 for Scissors
// 0 for lose, 3 for draw,  and 6 for win


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
                ("A","X") | ("B","Y") | ("C","Z") => 3, //Player Victory = 3
                _ => 0 // Player loss = 0
            };

            return play + result;
        }).sum();

    println!("{}", points);
        
}
