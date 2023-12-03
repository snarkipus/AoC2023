use color_eyre::eyre::Result;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
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
    let mut total = 0;
    data.iter().for_each(|number| {
        total += number;
    });

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

fn parse_data(data: Vec<String>) -> Result<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();
    data.iter().for_each(|line| {
        let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        // concatenate first and last digits as a usize
        let number = format!("{}{}", first_digit, last_digit).parse::<usize>();
        result.push(number.unwrap());
    });

    Ok(result)
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
}
