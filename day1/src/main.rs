use std::{fs, io::{BufReader, BufRead}, env};
fn main() -> Result<(), anyhow::Error>{

    if env::args().len() != 2 {
        anyhow::bail!("You must pass the input file as a parameter! Example:\nday1 ./input.txt")
    }

    let filename = env::args().nth(1).unwrap();

    let file = fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut sum = 0;

    loop {
        match reader.read_line(&mut buffer)? {
            0 => break,
            _ => {
                let mut first_digit: Option<u32> = None;
                let mut last_digit: Option<u32> = None;

                for c in buffer.chars() {
                    if let Some(val) = c.to_digit(10) {
                        if first_digit == None {
                            first_digit = Some(val);
                        } else {
                            last_digit = Some(val);
                        }
                    }
                }

                if last_digit == None {
                    last_digit = first_digit
                }

                if first_digit == None {
                    anyhow::bail!("Couldn't find a digit in line {}", buffer);
                }

                // println!("{first_digit:?} {last_digit:?}");

                sum += first_digit.unwrap() * 10 + last_digit.unwrap();
            },
        }
        
        buffer.clear();
    }
    
    println!("Sum found: {}", sum);

    Ok(())
}
