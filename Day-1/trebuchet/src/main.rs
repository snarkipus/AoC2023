use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use tracing::debug;
use tracing::info;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    color_eyre::install().unwrap();
    install_tracing("info");
    info!("Starting up...");

    // 1) Read input file
    let input = read_input("../input.txt").unwrap();

    // 2) Parse input file
    let data = parse_data(input).unwrap();

    // 3) Process data
    let total = data.iter().sum::<usize>();

    // 4) Print result
    println!("Total: {}", total);

    info!("Winding Down...");
}

#[tracing::instrument]
fn read_input(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }

    Ok(result)
}

#[tracing::instrument]
fn parse_data(data: Vec<String>) -> Result<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();
    data.iter().for_each(|line| {
        let new_line = replace_strings(line);
        let first_digit = new_line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = new_line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        // concatenate first and last digits as a usize
        let number = format!("{}{}", first_digit, last_digit).parse::<usize>();
        result.push(number.unwrap());
    });

    Ok(result)
}

#[tracing::instrument]
fn replace_strings(line: &str) -> String {
    let mut result = String::new();

    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut i = 0;

    while i < line.len() {
        let _foo = &line[i..];

        // check if the current character is a digit - if so, add it to the result
        if line.chars().nth(i).unwrap().is_ascii_digit() {
            result.push(line.chars().nth(i).unwrap());
            i += 1;
            continue;
        }

        // check if the current character is the beginning of a string to be replaced
        // if so, replace it and move the index forward to the last character of the replacement
        let mut replaced = false;
        for (key, &value) in map.iter() {
            if line[i..].starts_with(key) {
                result.push_str(value);
                i += key.len() - 1;
                replaced = true;
                break;
            }
        }

        // if the current character is not a digit and not the beginning of a string to be replaced
        // move the index forward by one
        if !replaced {
            i += 1;
        }
    }

    result
}

fn install_tracing(level: &str) {
    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or(level)).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    // test file reader:
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    fn test_read_input() {
        let result = read_input("../test-1.txt").unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "1abc2");
        assert_eq!(result[1], "pqr3stu8vwx");
        assert_eq!(result[2], "a1b2c3d4e5f");
        assert_eq!(result[3], "treb7uchet");
    }

    #[test]
    // test parse data
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    fn test_parse_data() {
        let data = read_input("../test-1.txt").unwrap();
        let result = parse_data(data).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 12);
        assert_eq!(result[1], 38);
        assert_eq!(result[2], 15);
        assert_eq!(result[3], 77);
    }

    #[test]
    // test main
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    fn test_main() {
        main();
    }

    #[test_case("two1nine", "219" ; "two1nine")]
    #[test_case("eightwothree", "823" ; "eightwothree")]
    #[test_case("abcone2threexyz", "123" ; "abcone2threexyz")]
    #[test_case("xtwone3four", "2134" ; "xtwone3four")]
    #[test_case("4nineeightseven2", "49872" ; "4nineeightseven2")]
    #[test_case("zoneight234", "18234" ; "zoneight234")]
    #[test_case("7pqrstsixteen", "76" ; "7pqrstsixteen")]
    // f47ninexfqsbdrseventwo7twonep - overlapping string case from the data - super sucked
    #[test_case("f47ninexfqsbdrseventwo7twonep", "47972721" ; "f47ninexfqsbdrseventwo7twonep")]
    fn test_replace_strings(input: &str, expected: &str) {
        let mut result = Vec::<String>::new();
        let line = String::from(input);
        result.push(replace_strings(&line));
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_parse_data_2() {
        let data = read_input("../test-2.txt").unwrap();
        let result = parse_data(data).unwrap();
        assert_eq!(result.len(), 7);
        assert_eq!(result[0], 29);
        assert_eq!(result[1], 83);
        assert_eq!(result[2], 13);
        assert_eq!(result[3], 24);
        assert_eq!(result[4], 42);
        assert_eq!(result[5], 14);
        assert_eq!(result[6], 76);
        assert_eq!(result.iter().sum::<usize>(), 281);
    }
}
