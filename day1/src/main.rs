use std::{
    env, fs,
    io::{BufRead, BufReader},
};

fn str_to_digit(buf: &str) -> Option<u32> {
    let simple_digit = match buf.chars().next() {
        Some('0') => Some(0),
        Some('1') => Some(1),
        Some('2') => Some(2),
        Some('3') => Some(3),
        Some('4') => Some(4),
        Some('5') => Some(5),
        Some('6') => Some(6),
        Some('7') => Some(7),
        Some('8') => Some(8),
        Some('9') => Some(9),
        Some(_) => None,
        None => None,
    };

    if simple_digit.is_some() {
        return simple_digit;
    }

    match buf {
        s if s.starts_with("zero") => Some(0),
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn main() -> Result<(), anyhow::Error> {
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

                for i in 0..buffer.len() {
                    if let Some(val) = str_to_digit(&buffer[i..]) {
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
            }
        }

        buffer.clear();
    }

    println!("Sum found: {}", sum);

    Ok(())
}
