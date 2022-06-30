use std::io;

fn print_grid(grid: [[u8; 3]; 3]) {
    for row in grid.iter() {
        let (c1, c2, c3) = (row[0], row[1], row[2]);
        println!("{}|{}|{}", c1, c2, c3);
    }
}

fn parse_input(inp_str: &str) -> Result<u8, String> {
    let out: u8 = match inp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(String::from("Unable to parse number.")),
    };

    if out > 3 {
        return Err(String::from(out.to_string() + " is not between 0 and 2."));
    }

    return Ok(out);
}

fn get_input() -> [u8; 2] {
    loop {
        let mut out = String::new();
        io::stdin()
            .read_line(&mut out)
            .expect("Failed to read line");

        let split: Vec<&str> = out.split(" ").collect();

        if split.len() != 2 {
            println!("Too many arguments.");
            continue;
        }

        let (row, col) = (split[0], split[1]);

        let row = match parse_input(row) {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let col = match parse_input(col) {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        return [row, col];
    }
}

fn check_win(grid: [[u8; 3]; 3]) -> bool {
    let mut win = false;

    for row in grid.iter() {
        if row[0] == row[1] && row[1] == row[2] && row[0] != 0 {
            win = true;
        }
    }

    for col in 0..3 {
        if grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col] && grid[0][col] != 0 {
            win = true;
        }
    }

    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[0][0] != 0 {
        win = true;
    }

    if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[0][2] != 0 {
        win = true;
    }

    return win;
}

fn main() {
    let mut grid = [[0u8; 3]; 3];
    print_grid(grid);

    let mut player = 1u8;

    loop {
        let [row, col] = get_input();
        let (row, col) = (row as usize, col as usize);

        if grid[row][col] != 0 {
            println!("That space is already taken.");
            continue;
        }

        grid[row][col] = player;

        print_grid(grid);

        if check_win(grid) {
            println!("Player {} wins!", player);
            break;
        }

        player = if player == 2 { 1 } else { 2 };
    }
}
