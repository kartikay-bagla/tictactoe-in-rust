use crate::grid::Grid;
use crate::input;

pub struct Game {
    grid: Grid,
    player: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            player: 1,
        }
    }

    pub fn play(&mut self) {
        println!("Welcome to Tic Tac Toe!");
        println!("Each player will take turns placing a mark on the grid.");
        println!("Marks will be represented as the row and column.");
        println!("They must separated be by a space and are 0 indexed.");
        println!("The first player to get 3 in a row wins.");
        loop {
            println!("Current Grid:");
            self.grid.print_grid();
            loop {
                println!("Player {}'s turn. Enter row column: ", self.player);
                let [row, col] = input::get_input();

                match self.grid.place_marker(row, col, self.player) {
                    Ok(_) => break,
                    Err(e) => println!("Could not place marker: {}", e),
                }
            }

            if self.grid.check_win() == self.player {
                println!("Player {} wins!", self.player);
                self.grid.print_grid();
                break;
            }

            self.player = if self.player == 2 { 1 } else { 2 };
        }
    }
}
