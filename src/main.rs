use std::io::{self, stdout, Write};

fn encode_decode(text: &str, op: u8) -> String {
    let mut res: String = String::from("");

    match op {
        1 => {
            for letter in text.chars() {
                if letter >= 'A' && letter <= 'Z' {
                    let aux = ((letter as u8 - 'A' as u8 + 13) % 26) + 'A' as u8;
                    res.push(aux as char);
                } else if letter >= 'a' && letter <= 'z' {
                    let aux = ((letter as u8 - 'a' as u8 + 13) % 26) + 'a' as u8;
                    res.push(aux as char);
                } else {
                    res.push(letter);
                }
            }
        }
        2 => {
            for letter in text.chars() {
                if letter >= 'A' && letter <= 'Z' {
                    let aux = (((letter as i16 - 'A' as i16 - 13 + 26) % 26) + 'A' as i16) as u8;
                    res.push(aux as char);
                } else if letter >= 'a' && letter <= 'z' {
                    let aux = (((letter as i16 - 'a' as i16 - 13 + 26) % 26) + 'a' as i16) as u8;
                    res.push(aux as char);
                } else {
                    res.push(letter);
                }
            }
        }
        _ => println!("[-] Invalid option"),
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_uppercase() {
        assert_eq!(encode_decode("HELLO", 1), "URYYB");
    }

    #[test]
    fn test_decode_uppercase() {
        assert_eq!(encode_decode("URYYB", 2), "HELLO");
    }

    #[test]
    fn test_encode_decode_empty_string() {
        assert_eq!(encode_decode("", 1), "");
    }
}

fn main() {
    loop {
        let mut to_encode: String = String::new();
        let mut input: String = String::new();

        print!("[+] 1: encode, 2: decode, 99: exit -> ");
        stdout().flush().expect("[-] Error flushing stdout");
        io::stdin().read_line(&mut input).expect("[-] Error while reading input");

        match input.trim().parse::<i32>() {
            Ok(opt) => {
                match opt {
                    1 => {
                        print!("[?] Text to encode: ");
                        stdout().flush().expect("[-] Error flushing stdout");
                        io::stdin().read_line(&mut to_encode).expect("[-] Error while reading input");
                        println!("[+] Encoded text: {}", encode_decode(to_encode.trim(), 1));
                    }
                    2 => {
                        print!("[?] Text to decode: ");
                        stdout().flush().expect("[-] Error flushing stdout");
                        io::stdin().read_line(&mut to_encode).expect("[-] Error while reading input");
                        println!("[+] Decoded text: {}", encode_decode(to_encode.trim(), 2));
                    }
                    99 => {
                        println!("[+] Exiting...");
                        break;
                    }
                    _ => println!("[-] ERROR: Invalid option"),
                }
            }
            Err(_err) => println!("[-] Error: Entered value isn´t a  number"),
        }
    }
}
