/**
 * This is a simple implementation of the Caesar cipher
 */

fn caesar(text: &str, shift: u32) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            // only shift letters not numbers or other characters
            let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            result.push((((c as u8 - offset + shift as u8) % 26) + offset) as char);
        } else {
            result.push(c);
        }
    }
    return result;
}

fn main() {
    // prompt for input
    let mut input = String::new();
    println!("Enter text to encrypt:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut shift = String::new();
    println!("Enter shift:");
    std::io::stdin().read_line(&mut shift).expect("Failed to read line");
    let parse = shift.trim().parse();
    let shift: u32 = match parse {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid shift, it must be an positive integer");
            return;
        }
    };

    // encrypt and print
    let encrypted = caesar(&input, shift % 26);
    println!("Encrypted text: {}", encrypted);
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar() {
        assert_eq!(caesar("abc", 0), "abc");
        assert_eq!(caesar("abc", 1), "bcd");
        assert_eq!(caesar("abc", 26), "abc");
        assert_eq!(caesar("abc", 27), "bcd");
        assert_eq!(caesar("abc", 52), "abc");
    }
}

