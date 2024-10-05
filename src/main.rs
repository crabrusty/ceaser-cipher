use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Alphabet {
    letters: Vec<char>,
    index_map: HashMap<char, usize>,
}

impl Alphabet {
    fn new(letters: &str) -> Self {
        let letters_vec: Vec<char> = letters.chars().collect();
        let index_map = letters_vec.iter().enumerate()
            .map(|(i, &c)| (c, i))
            .collect();

        Alphabet { 
            letters: letters_vec,
            index_map,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Operation {
    Cipher,
    Decipher,
}

impl Operation {
    fn apply(&self, message: &str, shifting:i32, direction:Direction, alphabet: &Alphabet)->Result<String,String>{
        match self {
            Operation::Cipher => Ok(shift(message.to_string(), shifting, direction, alphabet)),
            Operation::Decipher => Ok(shift(message.to_string(), -shifting, direction, alphabet)),
        }
    }
}

fn get_word_input(prompt: &str, alphabet: &Alphabet) -> String {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid word!");

        let input = input.trim().to_lowercase();

        if input.is_empty() || input.chars().all(|c| c.is_whitespace()) {
            println!("Invalid input! Please enter a non-empty word without spaces or newline characters.");
            continue;
        }

        if input.chars().all(|c| alphabet.index_map.contains_key(&c)) {
            return input;
        } else {
            println!("Invalid input! Please enter a word containing only the characters from the selected alphabet.");
        }
    }
}

fn get_int_input(prompt: &str) -> i32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn leaning() -> bool {
    loop {
        let choice = get_int_input("Would you like to continue to cipher & decipher or quit?\n1.) Continue\n2.) Quit");
        match choice {
            1 => return true,
            2 => return false,
            _ => println!("Invalid input, please enter 1 for Continue or 2 for Quit."),
        }
    }
}

fn shift(message: String, shifting: i32, direction: Direction, alphabet: &Alphabet) -> String {
    let mut result = String::with_capacity(message.len());
    let len = alphabet.letters.len() as i32;
    let normalized_shift = shifting.rem_euclid(len);

    for c in message.chars() {
        if let Some(&index) = alphabet.index_map.get(&c) {
            let shift_amount = match direction {
                Direction::Left => ((index as i32 - normalized_shift).rem_euclid(len)) % len,
                Direction::Right => ((index as i32 + normalized_shift).rem_euclid(len)) % len,
            };
            result.push(alphabet.letters[shift_amount as usize]);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let english_alphabet = Alphabet::new("abcdefghijklmnopqrstuvwxyz");
    let turkish_alphabet = Alphabet::new("abcçdefgğhıijklmnoöprsştuüvyz");
    
    let mut lean = leaning();

    while lean {
        let transaction = loop {
            let choice = get_int_input("Please choose the desired operation:\n1.) Cipher\n2.) Decipher");
            if choice == 1 || choice == 2 {
                break choice;
            } else {
                println!("Please choose a valid operation (1 for Cipher or 2 for Decipher).");
            }
        };

        let language_choice = loop {
            let choice = get_int_input("Please choose the desired language:\n1.) Turkish\n2.) English");
            if choice == 1 || choice == 2 {
                break choice;
            } else {
                println!("Please choose a valid language (1 for Turkish or 2 for English).");
            }
        };

        let alphabet = match language_choice {
            1 => &turkish_alphabet,
            2 => &english_alphabet,
            _ => unreachable!(),
        };

        let message: String;
        let direction: Direction;
        let shifting: i32;

        match transaction {
            1 => {
                message = get_word_input("Please enter the word to cipher:", alphabet);

                direction = loop {
                    let choice = get_int_input("Please choose the desired direction to shift:\n1.) Left\n2.) Right");
                    match choice {
                        1 => break Direction::Left,
                        2 => break Direction::Right,
                        _ => println!("Please choose a valid direction (1 for Left or 2 for Right)."),
                    }
                };

                shifting = get_int_input("Enter the number of positions to shift:");

                match Operation::Cipher.apply(&message, shifting, direction, alphabet) {
                    Ok(result) => println!("Original {} -> Ciphered {}", message, result),
                    Err(e) => println!("{}", e),
                }
            }
            2 => {
                message = get_word_input("Please enter the word to decipher:", alphabet);

                direction = loop {
                    let choice = get_int_input("Please choose the desired direction to shift:\n1.) Left\n2.) Right");
                    match choice {
                        1 => break Direction::Left,
                        2 => break Direction::Right,
                        _ => println!("Please choose a valid direction (1 for Left or 2 for Right)."),
                    }
                };

                shifting = get_int_input("Enter the number of positions to shift:");

                match Operation::Decipher.apply(&message, shifting, direction, alphabet) {
                    Ok(result) => println!("Ciphered {} -> Deciphered {}", message, result),
                    Err(e) => println!("{}", e),
                }
            }
            _ => println!("This should not happen as input is validated."),
        }

        lean = leaning();
    }
}