use std::io;

fn parse_number(inp_str: &str) -> Result<u8, &'static str> {
    let out: u8 = match inp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Unable to parse number."),
    };

    if out > 3 {
        return Err("Number is not between 0 and 2.");
    }

    return Ok(out);
}

pub fn get_input() -> [u8; 2] {
    loop {
        let mut out = String::new();
        io::stdin()
            .read_line(&mut out)
            .expect("Failed to read line");

        let split: Vec<&str> = out.split(" ").collect();

        if split.len() > 2 {
            println!("Too many arguments.");
            continue;
        } else if split.len() < 2 {
            println!("Too few arguments.");
            continue;
        }

        let (row, col) = (split[0], split[1]);

        let row = match parse_number(row) {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let col = match parse_number(col) {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        return [row, col];
    }
}
