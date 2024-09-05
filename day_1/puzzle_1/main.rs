use std::fs::File;
use std::io::Read;

fn main() 
{
    let input = get_input_file("input.txt").expect("input.txt file should be in the same folder as the executable");
    let words = input.split('\n');
    let mut sum: i64 = 0;

    for word in words 
    {
        let numbers: Vec<i8> = get_numbers_from_word(word);

        if numbers.len() == 0
        {
            continue;
        }

        let first_number: i8 = numbers[0];
        let last_number: i8 = numbers.last().copied().expect("Word should have at least a number");

        let two_digit_number = 10 * first_number + last_number;
        sum += two_digit_number as i64;
    }

    println!("total calibraton : {}", sum);
}

fn get_input_file(file_name: &str) -> std::io::Result<String>
{
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_numbers_from_word(word: &str) -> Vec<i8>
{
    let mut numbers: Vec<i8> = Vec::new();
    let mut index: usize = 0;

    // For every character of the word
    while index < word.len()
    {
        let character: char = word.chars().nth(index).expect("Index out of range");

        // If character is a digit
        if character.is_ascii_digit()
        {
            let number = character
                .to_string()
                .as_str()
                .parse::<i8>()
                .expect("Cannot parse number");

            numbers.push(number);
        }

        index += 1;
    }

    numbers
}
