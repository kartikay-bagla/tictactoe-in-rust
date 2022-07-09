pub struct Grid {
    grid: [[u8; 3]; 3],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: [[0u8; 3]; 3],
        }
    }

    pub fn print_grid(&self) {
        println!("┌---┬---┬---┐");
        println!(
            "│ {} │ {} │ {} │",
            self.grid[0][0], self.grid[0][1], self.grid[0][2]
        );
        println!("├---┼---┼---┤");
        println!(
            "│ {} │ {} │ {} │",
            self.grid[1][0], self.grid[1][1], self.grid[1][2]
        );
        println!("├---┼---┼---┤");
        println!(
            "│ {} │ {} │ {} │",
            self.grid[2][0], self.grid[2][1], self.grid[2][2]
        );
        println!("└---┴---┴---┘");

        // for row in self.grid.iter() {
        //     for col in row.iter() {
        //         print!("{} ", col);
        //     }
        //     println!();
        // }
    }

    pub fn check_win(&self) -> u8 {
        let mut win = 0u8;

        for row in self.grid.iter() {
            if row[0] == row[1] && row[1] == row[2] && row[0] != 0 {
                win = row[0];
            }
        }

        for col in 0..3 {
            if self.grid[0][col] == self.grid[1][col]
                && self.grid[1][col] == self.grid[2][col]
                && self.grid[0][col] != 0
            {
                win = self.grid[0][col];
                break;
            }
        }

        if self.grid[0][0] == self.grid[1][1]
            && self.grid[1][1] == self.grid[2][2]
            && self.grid[0][0] != 0
        {
            win = self.grid[0][0];
        }

        if self.grid[0][2] == self.grid[1][1]
            && self.grid[1][1] == self.grid[2][0]
            && self.grid[0][2] != 0
        {
            win = self.grid[0][2];
        }

        return win;
    }

    pub fn place_marker(&mut self, row: u8, col: u8, player: u8) -> Result<(), &'static str> {
        if row > 2 || col > 2 {
            return Err("Invalid row or column.");
        }

        if self.grid[row as usize][col as usize] != 0 {
            return Err("That space is already taken.");
        }

        self.grid[row as usize][col as usize] = player;

        return Ok(());
    }
}
