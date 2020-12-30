//
// file: program.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//

// This is how we "import" a module from the standard library. A module is a group of functions and
// types. "std" stands for "standard library" and "io" stands for "input/output". We will use this
// module to read input from the user of our application.
// The import "self" imports the name "io" itself, and "Write" imports the "Write trait" which we
// need to flush stdout below.
use std::io::{self, Write};
// We use the process::exit function to quit the program when we need to.
use std::process;

// This constant can be used to set the board size
// Since Rust's arrays are fat pointers, you won't see this constant referred to again after the
// we declare the type of Game. I mention this because if you were writing in a language like C,
// you would either need to pass the size to every function with the board or rely on this global
// constant. In Rust, that information is stored directly in the array so you always have the
// correct value.
const BOARD_SIZE: usize = 3;

// We want to use an enum for piece because we can either have one piece or the other on a tile,
// but never both at the same time
// `derive` automatically derives certain useful traits. These make this custom type that we've
// defined copyable, comparable for equality, and more without any additional work!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    // Access these variants using `Piece::X` or `Piece::O`
    X,
    O,
}

impl Piece {
    // This method is used to return the opposite piece and is used to quickly determine the next
    // piece after each move
    // By putting `self` as the first parameter, we are copying the piece that this method is
    // called on. This happens because this type derives `Copy` in its declaration. Without that,
    // using `self` alone would "move" the value into this function. Rust would ensure that no
    // other code could access it afterwards. Copy gives us complete control over which values we
    // want Rust to copy and which values we want Rust to move and only copy when we explicitly
    // ask for it.
    // For more information, see: https://doc.rust-lang.org/beta/std/marker/trait.Copy.html
    pub fn other(self) -> Piece {
        // The last expression in a function is returned from that function, so without writing
        // `return` anywhere, we can return the correct Piece from this function.
        // We could have also used multiple if statements, but this is a little simpler to read
        // once you understand the syntax.

        // Rust will tell us if Piece ever changes and this match doesn't cover every case
        match self {
            // match lets us conveniently express both cases without too much additional syntax
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

// By using an Option type, we can represent the possibility of having one of the valid piece
// types, or no piece at all. Notice that we chose not to just add an "Empty" piece type because
// this allows us to use Piece for other things like representing the choices for the current
// piece. The current piece can never be "empty", so it doesn't make sense to have an Empty variant
// in the Piece enum.
pub type Tile = Option<Piece>;
// We represent the tiles of the board using a 2D array
// Each element of the first array is a row of the board.
// tiles[1][2] accesses the second row and third column of the board.
pub type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];

// There are three possibilities for the winner at the end of the game. We represent them as an
// enum because only one of them can ever occur at a given time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}

// This type represents the possible errors that can occur when making a move
#[derive(Debug, Clone)]
pub enum MoveError {
    // Putting /// instead of // means that Rust's documentation tool will automatically pickup
    // that comment and use it when generating beautiful documentation for this module.

    /// The game was already over when a move was attempted
    GameAlreadyOver,

    // Fields allow us to provide more information about what happened

    /// The position provided was invalid
    InvalidPosition { row: usize, col: usize },

    /// The tile already contained another piece
    TileNotEmpty { other_piece: Piece, row: usize, col: usize },
}

#[derive(Debug, Clone)]
pub struct Game {
    tiles: Tiles,
    // There is always a current piece, so we don't need to wrap it in an Option type.
    current_piece: Piece,
    // There is only a winner at the end of the game, and once there is, it never changes. If we
    // wanted to, we could use the Rust type system to enforce this invariant and make sure the
    // program can't even be written in a way that would violate that. I decided to keep it simple
    // and not do that, but it's a great exercise to try out!
    // Hint: Replace the Winner enum declaration with a `struct Winner(...)` and make the type of
    // this field `Winner`. If you make that type so that the winner can only be set to something
    // other than None once, it will no longer be possible to write a program that violates the
    // invariant stated above.
    winner: Option<Winner>,
}

impl Game {
    // Using Self inside of an impl allows us to refer to its type (i.e. `Game`) without using the
    // type name explicitly. This is useful for renaming!
    pub fn new() -> Self {
        // Here we construct and return a new instance of Game
        Self {
            // Here, we take advantage of the Default trait to make it so that this code doesn't
            // have to know the type we defined for tiles in order to initialize it. Rust has
            // already defined the trait for arrays and the Option type, so we don't need to
            // implement it ourself!
            // More info: https://doc.rust-lang.org/std/default/trait.Default.html
            tiles: Default::default(),
            // We want to start with X
            current_piece: Piece::X,
            // There is no winner at the start of the game. We cleanly represent this with `None`.
            // Rust will warn us before our program even tries to run if we forget that this value
            // might be None.
            winner: None,
        }
    }

    // `&mut self` reflects that we plan to modify this struct in this method. Rust will ensure
    // that no other thread can access this object while we are modifying it. Thus eliminating any
    // possible data races.
    // Both row and col must be values from 0 to (BOARD_SIZE-1)
    // In the return type, () indicates the "unit type". That means that on success, this function
    // returns nothing.
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), MoveError> {
        if self.is_finished() {
            // Here, we use `return` to indicate that we want to leave this function early if this
            // case occurs. We could have written it without return by using `else` and indenting
            // the remaining function.
            return Err(MoveError::GameAlreadyOver);
        }
        // The usize type is "unsigned", meaning it is always positive. That means that this
        // potential error case is unrepresentable. We don't need to check for it if it can't
        // happen!
        // Notice that we use `.len()` instead of the BOARD_SIZE constant we defined because Rust
        // arrays provide their length.
        else if row >= self.tiles.len() || col >= self.tiles[0].len() {
            // Rust supports a "field shorthand" syntax which allows us to write {row, col} instead
            // of {row: row, col: col}
            return Err(MoveError::InvalidPosition {row, col});
        }
        // Rust allows us to conditionally test a pattern match without using `match` directly.
        // This makes it super convenient to check if the tile is empty or not
        else if let Some(other_piece) = self.tiles[row][col] {
            // The pattern match allows us to check if there is a potential value and extract it
            // in one quick sweep. This makes writing the next line very easy!
            return Err(MoveError::TileNotEmpty {other_piece, row, col});
        }

        // Now that we've done all of the error checking, we can proceed with making the move and
        // modifying the tiles and current piece

        // Here we store the current piece at the correct location in self.tiles
        self.tiles[row][col] = Some(self.current_piece);

        // Notice that since we don't publically expose a way to set the current piece, we can
        // always be sure that it will be updated correctly and according the rules we expect.
        self.current_piece = self.current_piece.other();

        // After making a move, it may be that someone won the game. We'll use another method for
        // that since this one is getting quite long.
        self.update_winner(row, col);

        // Now that everything is complete, we can go ahead and return our "nothing" value `()`
        // called "unit" to indicate that this operation was a success. We construct a Result type
        // using its `Ok` variant as the constructor.
        Ok(())
    }

    // We use a private method to separate code that shouldn't be accessed publically
    fn update_winner(&mut self, row: usize, col: usize) {
        // To find a potential winner, we only need to check the row, column and (maybe) diagonal
        // that the last move was made in.

        // Let's make some convenience variables for the number of rows and columns
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();

        // We can extract the row pretty easily because of how we stored tiles
        let tiles_row = self.tiles[row];

        // To get the correct column, we could do something very fancy that would work for every
        // size of board, but in this case we'll just do the simplest thing and get the column
        // directly using indexing.
        let tiles_col = [self.tiles[0][col], self.tiles[1][col], self.tiles[2][col]];

        // This relies on the assumption that the board has size 3, so let's assert that so that if
        // someone ever changes this code there are no weird bugs
        // This will produce an error at runtime if this assumption is broken.
        assert!(rows == 3 && cols == 3,
            "This code was written with the assumption that there are three rows and columns");

        // There are two diagonals on the board. Their positions are as follows:
        // 1. (0, 0), (1, 1), (2, 2)
        // 2. (0, 2), (1, 1), (2, 0)
        // Due to the possibility of being on (1, 1), we might be on both diagonals. We will check
        // both diagonals separately.
        // Notice that on a 3x3 board, if row == col, we are on the first diagonal
        // and if (rows - row - 1) == col, we are on the second diagonal.
        // If we are on neither diagonal, we can just use an array of None's so that it definitely
        // won't find a match.

        // Here, we see that if statements can be used as expressions just like match statements.
        // That means that we can assign this variable to the result of the if statement.
        let tiles_diagonal_1 = if row == col {
            // Once again, we'll do the simplest thing and just use an array.

            // Diagonal 1
            [self.tiles[0][0], self.tiles[1][1], self.tiles[2][2]]
        }
        else {
            // This will never produce a winner, so it is suitable to use for the case where the
            // last move isn't on diagonal 1 anyway.
            [None, None, None]
        };

        let tiles_diagonal_2 = if (rows - row - 1) == col {
            // Diagonal 2
            [self.tiles[0][2], self.tiles[1][1], self.tiles[2][0]]
        }
        else {
            // Our last move isn't on diagonal 2.
            [None, None, None]
        };

        // Now that we have the row, column and diagonal of the last move, let's check if we have
        // a winner. To do that, we'll use a check_winner function that either returns a new
        // Winner or None. This is useful because we can chain together the methods of the Option
        // type to produce a result. This is an alternative to multiple if statements that works
        // just as well.
        fn check_winner(row: &[Tile]) -> Option<Winner> {
            // This is an "inner function". It is only visible to this update_winner method. We
            // could have defined this as a method or defined it as a function separate from this
            // impl too.
            // The type `&[Tile]` is known as a slice. This is how we pass an array by reference.
            // We don't have to pass the size with the array because the array pointer also stores
            // its length.
            // By returning an option type, we signal that this function may return some value or
            // no value (i.e. None).

            // Here, we once again do the simplest thing possible and just use indexes to check
            // if the entire row is the same. We could potentially do something more general using
            // iterators, but why do that if this simpler way works?
            if row[0] == row[1] && row[1] == row[2] {
                // We use a match to retrieve the correct winner based on the piece that has filled
                // this row.
                match row[0] {
                    Some(Piece::X) => Some(Winner::X),
                    Some(Piece::O) => Some(Winner::O),
                    None => None,
                }
            }
            else {
                // All the tiles are not the same, there is no winner yet, so let's signal that
                // with None
                None
            }
        }
        // Now that we can determine if there is a winner or not, we can use the option type's
        // methods to chain together the results. See the Option type documentation for more info:
        // https://doc.rust-lang.org/std/option/enum.Option.html
        self.winner = self.winner
            // The || syntax is actually defining a special function called a "closure" (or
            // "lambda" in some languages). That allows us to delay calling the check_winner
            // function until we actually need it.
            // By using or_else over and over again, we never overwrite a previously found winner
            // and the code is only run in case a previous winner was *not* found.
            .or_else(|| check_winner(&tiles_row))
            .or_else(|| check_winner(&tiles_col))
            .or_else(|| check_winner(&tiles_diagonal_1))
            .or_else(|| check_winner(&tiles_diagonal_2));

        // The final case is when the board has filled up. Here, for the first time, we'll be a
        // bit fancy and use the Iterator trait. For more info, see the book:
        // https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html
        // This is also the first time we see a multiline closure using curly braces. Just like
        // any other function, this returns the final (and only) value between the curly braces.
        self.winner = self.winner.or_else(|| {
            // You can read this code as follows:
            // if in each of the rows, all tiles have *something* in them,
            //     return that the winner is a tie.
            // otherwise, return that there is no winner yet
            // For more information on `all`, see:
            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
            if self.tiles.iter().all(|row| row.iter().all(|tile| tile.is_some())) {
                Some(Winner::Tie)
            }
            else {
                None
            }
        });
    }

    // We can define helpful accessor functions for common questions that will be asked about this
    // type. This makes it so that people using this type won't have to rely on how the type is
    // represented.
    // `&self` tells the Rust compiler that we won't be modifying this type
    pub fn is_finished(&self) -> bool {
        // The last line of a function is its return value, so we don't need to write return for
        // simple one line functions.

        // The game is finished if there is a winner.
        // Since we used an Option type, we can use the convenient method it provides for checking
        // if it is Some or None instead of having to match on the type itself.
        self.winner.is_some()
    }

    // This method returns the winner of the game (if any). Since Winner derives the Copy trait, we
    // can return it directly from this function without moving its value. Rust will copy the value
    // (including the Option type that wraps it). For small types, this can make writing the code
    // much easier without introducing any additional performance penalty.
    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    // This method is similar to the winner method above. It returns a copy of the current piece.
    // Just like Winner, Piece also implements the Copy trait.
    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    // This function gives public, read-only access to the tiles of the board. Rust will enforce
    // at compile-time that no outside entity is able to modify the tiles from this reference.
    pub fn tiles(&self) -> &Tiles {
        // The `&` at the front creates a read-only reference. `self.tiles` accesses the tiles
        // field of this struct.
        &self.tiles
    }
}

// This type is used to provide an error when the user provides an invalid move string. If we
// wanted to avoid copying the invalid string, we could use &str instead and Rust would enforce at
// compile time that the reference remained valid until any instance of InvalidPiece containing it
// goes out of scope. String is used for the same of simplicity. By marking the type stored in this
// struct as `pub`, its value can be freely accessed even in patterns (for example, match
// statements).
#[derive(Debug, Clone)]
pub struct InvalidMove(pub String);

//
// foundation of the program and related
// application logic must be implemented
// in the foundation.
//
pub fn foundation()
{
    // The constructor for Game creates a new, empty Tic-Tac-Toe board. `mut` signals that we plan
    // to modify the value of the game variable. Rust will tell us if we forget to use this and
    // warn us if we use it but it isn't needed.
    let mut game = Game::new();

    // Let's continuously prompt the user for input using a loop until the game is finished
    while !game.is_finished() {
        // First, print out the current board
        print_tiles(game.tiles());

        // Inform the user of who's turn it currently is
        // match will enforce that we do not forget any case and the string that it produces will
        // replace `{}` in the printed string.
        println!("Current piece: {}", match game.current_piece() {
            Piece::X => "x",
            Piece::O => "o",
        });

        // prompt_move continuously prompts for a valid move from the user, determines exactly
        // which position on the board that move is referring to, and then returns that move
        let (row, col) = prompt_move();

        // Now that we have a move, let's attempt to make it
        // We use match to account for every case of the result
        match game.make_move(row, col) {
            // If the move is made successfully, we can just move on. You can think of empty
            // curly braces as an "empty expression". We could have also used the unit value `()`.
            Ok(()) => {},
            // Match allows us to conveniently match even nested types like Result and pull out the
            // fields as variables

            // Since we are using is_finished(), it should never be possible for this error to
            // occur. If it does, that means that we (the programmer) did something wrong, not the
            // user. `unreachable!()` works a lot like `println!();` except it exits the program
            // with an error using the message that we provided it. Use `unreachable!()` whenever
            // you encounter a case that you think should never be reached.
            Err(MoveError::GameAlreadyOver) => unreachable!("Game was already over when it should not have been"),
            // Since prompt_move limits the range of what can be returned, it should never allow
            // the user to enter a move that is out of range. Thus, this case is unreachable as
            // well.
            Err(MoveError::InvalidPosition {row, col}) => {
                unreachable!("Should not be able to enter an invalid move, but still got ({}, {})", row, col)
            },

            // Notice that we have already eliminated two possible errors just by structuring our
            // code in a certain way!

            // This is the only case that prompt_move does not account for, so if this happens, we
            // print an error message.
            // The `eprintln!` macro is exactly the same as `println!` except it prints to stderr
            // instead of stdout.
            Err(MoveError::TileNotEmpty {other_piece, row, col}) => eprintln!(
                // Each {} will be replaced with one of the arguments following this string
                "The tile at position {}{} already has piece {} in it!",
                // The row number that is displayed starts at 1, not zero, so we add 1 to get the
                // correct value
                row + 1,
                // `b'A'` produces the ASCII character code for the letter A (i.e. 65)
                // Adding col to it will produce either 65 (A), 66 (B), or 67 (C).
                // `as u8` is necessary because b'A' has type u8 and we can't add u8 to usize
                // without performing a conversion first.
                // Converting it to char using `as char` will get Rust to format this as a
                // character rather than printing the number out
                (b'A' + col as u8) as char,
                // match allows us to print something for each case and will tell us if something
                // ever changes such that this is no longer complete
                match other_piece {
                    Piece::X => "x",
                    Piece::O => "o",
                },
            ),
        }
    }

    // Once the loop is over, the game is finished. Let's output the results

    // First, we'll print the board again
    print_tiles(game.tiles());

    // Then print out which piece won the game
    // We use expect() to express that there should definitely be a winner now and if the winner
    // method returns None, the program should exit with this error
    match game.winner().expect("finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
} // end of function foundation

// Functions do not need to be ordered in any particular way in the file. That means that Rust
// doesn't suffer from any forward declaration issues where those declarations can get out of sync
// with the actual function implementation.

// This function returns a "tuple" of two values, the row and column of the selected move. Tuples
// are very useful for when you have a function that needs to return two values because it saves
// you from having to define a custom struct just for that purpose.
fn prompt_move() -> (usize, usize) {
    // We'll use `loop` to continuously prompt for input until the user provides what we want. When
    // we get the answer we want, the loop will return the value and it will be used as the return
    // value of this function
    loop {
        // Rust supports convenient `print!` and `println!` macros which support easy and
        // customizable formatting of values from your program. Here we are just using them to
        // prompt for some values that we want the user of our program to provide.
        print!("Enter move (e.g. 1A): ");

        // Line-buffering is when something waits until it sees a new line character before
        // actually writing to its designated destination. Rust's stdout is line-buffered by
        // default, so `print!` does not produce any output unless we "flush" the contents of
        // stdout's buffer in the line below.
        // expect() is how we "ignore" any error that could occur during this process. If an error
        // does occur, the program will exit with the message we provided.
        io::stdout().flush().expect("Failed to flush stdout");

        // The read_line() function is something we defined below to make reading input quick and
        // easy.
        let line = read_line();

        // We delegate reading the line as a move to the parse_move function. That function takes a
        // string and converts it to a "tuple" of two values (row, col). The read_line function
        // returns the type String, but parse_move expects a &str. We use `&` here to convert
        // String to &String. Rust then automatically converts &String to &str. This isn't a
        // special case for just strings, Rust supports a feature called "deref conversions" and
        // this is just a consequence of that. For more information, see:
        // http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
        match parse_move(&line) {
            // The benefit of parse_move returning a Result is that we can't forget to handle the
            // case where the input might be invalid. match gives us a convenient syntax for
            // handling each case.

            // Rust allows us to "return" a value from a loop by providing it to break. When
            // the loop exits, this will be the return value of the function too because the loop
            // is the last statement in this function.
            Ok((row, col)) => break (row, col),
            // Instead of defining methods to extract the value from InvalidMove, we can use
            // pattern matching to extract its value and print a helpful error message. The
            // `eprintln!` macro is exactly the same as `println!` except it prints to stderr
            // instead of stdout.
            Err(InvalidMove(invalid_str)) => eprintln!(
                // The `{}` is replaced with the next argument passed to eprintln. We can pass an
                // arbitrary amount of arguments and Rust can even tell us at compile time if there
                // is a mismatch between the number of {} and the number of additional arguments
                // passed.
                "Invalid move: '{}'. Please try again.",
                invalid_str,
            ),
        }
    }
}

// This function gets the row and column of the move the user entered. If the string doesn't
// represent a valid move, we return Result::Err to indicate failure.
// We pretty much always want to use &str instead of String in function arguments.
// For learn why, see:
// http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// NOTE: There are various ways that we could make this more "idiomatic" using some of the advanced
// features of Rust. However, notice though that we don't really lose anything or make anything
// worse for ourselves by keeping it simple. Rust lets you write nice code even if you haven't
// mastered all of its features just yet.
fn parse_move(input: &str) -> Result<(usize, usize), InvalidMove> {
    // The move will be in the format 1A, 2C, 3B, etc.
    // Let's start by rejecting any input that isn't of size 2
    if input.len() != 2 {
        // We use `return` to exit early from this function in case the size of the input is
        // incorrect.
        return Err(InvalidMove(input.to_string()));
    }

    // Let's start by getting the row number
    // Using match allows us to easily accept the cases we want to support and reject everything
    // else. If none of the cases match, an error will be returned.
    let row = match &input[0..1] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        _ => return Err(InvalidMove(input.to_string())),
    };

    let col = match &input[1..2] {
        // Rust lets us match against multiple patterns using | to separate them. This
        // lets us accept either lowercase or uppercase versions of the letters.
        "A" | "a" => 0,
        "B" | "b" => 1,
        "C" | "c" => 2,

        // We didn't find a match so far, so the string must be invalid. We use the `Err`
        // variant of Result to express that.
        // We can convert a &str to a String using `to_string()`. InvalidMove expects a String,
        // so we need to do this for this code to work.
        invalid => return Err(InvalidMove(invalid.to_string())),
    };

    // The last line of the function is the return value, so we construct the tuple that we want
    // to return with the move that the user selected
    Ok((row, col))
}

// This function is something we've defined to make reading a line of input convenient. Rust gives
// us a lot of control over our program so we could do many fancy things like buffer the input as
// we read it or properly handle error conditions. However, since this is a simple application, we
// have chosen to just exit the program when an error occurs and do no extra buffering of the
// input. Since we're just reading a line at a time and we expect the lines to be short, this
// should not cause problems in the majority of cases. Rust gives us the power to make that choice
// explicitly and know that we are making it in the code.
fn read_line() -> String {
    // This creates a new growable/heap-allocated string. The `mut` after `let` declares that we
    // plan to modify the string. Saying this explicitly lets the compiler automatically check that
    // we don't modify any variables that we don't intend to. Many languages encourage you to use
    // `const` or `final` on pretty much everything until you don't need to. In Rust, that
    // behaviour is by default.
    let mut input = String::new();
    // Here, we read a line of input from the standard input stream stdin. `&mut input` passes a
    // mutable reference to the String in the input variable. This allows the function to modify
    // input without taking ownership of its value. That way we can return it from this function
    // afterwards.
    // expect() is a function that takes a Result value and exits the program with an error message
    // if the Result value is anything other than Ok(...). This in a way is "ignoring" any error
    // that can occur while reading input. However, instead of ignoring it implicitly, we explciitly
    // call out that we intend to just exit the program with an error if this operation fails. This
    // is one of the ways that Rust gives you control. Don't want to deal with a potential failure?
    // You don't have to! But it's really nice to know where the error came from if something ever
    // does go wrong and you want to figure out why.
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // An empty string will only be returned if we reach the end of input (otherwise we always
    // receive at least a newline character).
    if input.is_empty() {
        // We print a final newline because otherwise the cursor may still be at the end of one
        // of our `print!` calls earlier.
        println!();

        // process::exit(0) indicates that the program exited successfully. This will end the
        // program right here, and none of the rest of our code will run.
        process::exit(0);
    }

    // read_line leaves the trailing newline on the string, so we remove it using truncate. By
    // modifying the string in place, we avoid copying its contents after it was just allocated.
    let len_without_newline = input.trim_end().len();
    input.truncate(len_without_newline);

    // The last expression in a function is returned from that function. We want to return the
    // line that was read, so we put that variable on its own at the end of the function in order
    // to provide it as the result of this function.
    input
}

// This function is used to print out the board in a human readable way
fn print_tiles(tiles: &Tiles) {
    // The result of this function will be something like the following:
    //   A B C
    // 1 x ▢ ▢
    // 2 ▢ ▢ o
    // 3 ▢ ▢ ▢
    //
    // The boxes represent empty tiles, and x and o are placed wherever a tile is filled.

    // First we print the space before the column letters
    print!("  ");
    // Then we look from the numbers 0 to 2.
    // `a..b` creates a "range" of numbers from a to one less than b.
    // `tiles[0].len()` gets the number of columns (i.e. 2)
    // `as u8` converts the length from the type `usize` to the type `u8` so that it works in the
    // body of the loop
    for j in 0..tiles[0].len() as u8 {
        // `b'A'` produces the ASCII character code for the letter A (i.e. 65)
        // By adding j to it, we get 'A', then 'B', and then 'C'.
        // We don't just want to print the ASCII character code, so we convert that number into
        // a character using `as char`. That way Rust will print it correctly.
        print!(" {}", (b'A' + j) as char);
    }
    // This prints the final newline after the row of column letters
    println!();

    // Now we print each row preceeded by its row number
    // .iter().enumerate() goes through each row and provides a row number with each element using
    // a tuple.
    for (i, row) in tiles.iter().enumerate() {
        // We print the row number with a space in front of it
        print!(" {}", i + 1);
        // Now we go through each tile in the row and print it out
        for tile in row {
            // Here, we match on the value of the tile. We use `*` to "dereference" the tile and
            // match on its value of type Option<Piece>. This is just for convenience and is
            // actually something that future versions of Rust might not even require in order to
            // match on something as simple as this.
            print!(" {}", match *tile {
                // The string produced by this match will be printed in `print!`. This match works
                // because we return the same type, &str, in each branch. Rust still requires that
                // if a match statement produces a value, it produces a value of the same type in
                // every branch.
                // Notice that we don't need to create another match for the piece produced in
                // Some(...). Rust allows us to match arbitrarily nested structures with no
                // additional syntax.
                Some(Piece::X) => "x",
                Some(Piece::O) => "o",
                None => "\u{25A2}",
            });
        }
        // We finish each row by printing a final new line
        println!();
    }

    // Add an extra line at the end of the board to space it out from the prompts that follow
    println!();
}
