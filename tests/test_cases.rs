//
// file: test_cases.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
// USE CASE:
//
// The use case of this file is to contain the test cases needed by this
// project since its important to test once implementation against a set
// of common test cases
//
extern crate program;
use program::{Game, Piece, Winner};

// These are tests! Rust has testing built-in so you get a streamlined experience that encourages
// you to write tests more often.
// To run these tests, run `cargo test`
// More information: https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html

// #[cfg(test)] tells the compiler to only include the rest of this code if we are running
// `cargo test`. This speeds up compile times since Rust doesn't need to process a bunch of code
// which won't be run otherwise.
#[cfg(test)]
mod tests {
    //TODO: Writing more tests. These are not even close to
    // exhaustive, but they are a good start!

    // This is a test! Just add #[test] to a regular function and Rust will find that function
    // and run it during `cargo test`
    #[test]
    fn col_3_o_wins() {
        // To test the game, we just have to create a game, play on it a bit, and then check
        // what happened
        let mut game = Game::new();
        // In tests, it is okay to "unwrap" and ignore errors. If something goes wrong, the test
        // will fail because unwrap will exit with an error
        // It can be helpful to use "expect" instead since with that you can provide a message
        // for more context.
        game.make_move(0, 0).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 2).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(0, 2).unwrap();
        // assert_eq! is a special macro (like `println!`) which checks if two things are equal
        // and then exits with an error if they are not. We use this to make sure game behaves
        // as we expect
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn diag_1_x_wins() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn diag_2_x_wins() {
        let mut game = Game::new();
        game.make_move(0, 2).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(2, 0).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn row_2_o_wins() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(1, 0).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 1).unwrap();
        game.make_move(0, 2).unwrap();
        game.make_move(1, 2).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn tie() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(0, 2).unwrap();
        game.make_move(2, 0).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(1, 0).unwrap();
        game.make_move(1, 2).unwrap();
        game.make_move(1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }
}
