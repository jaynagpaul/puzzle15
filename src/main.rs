mod puzzle;
use std::io;

use puzzle::Puzzle;

fn main() {
    println!("Welcome to Puzzle15");
    let mut puzzle = Puzzle::new_random_board();

    println!("{puzzle}");

    while !puzzle.has_won() {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "w" => puzzle.move_up(),
            "a" => puzzle.move_left(),
            "s" => puzzle.move_down(),
            "d" => puzzle.move_right(),

            s => println!("Invalid input '{s}'"),
        }

        println!("{puzzle}");
    }

    println!("You won!");
}
