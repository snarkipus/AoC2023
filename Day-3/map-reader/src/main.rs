use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Symbol {
    position: Position,
    symbol: char,
}

#[derive(Debug, Clone)]
struct Numeral {
    position: Position,
    value: u32,
}

#[derive(Debug)]
struct Number(Vec<Numeral>);

impl Number {
    fn border(&self) -> HashSet<Position> {
        let mut border = HashSet::new();
        self.0.iter().enumerate().for_each(|(i, numeral)| {
            // first numeral: 5 neighbors
            if i == 0 {
                border.insert(Position { row: numeral.position.row - 1, col: numeral.position.col }); // above
                border.insert(Position { row: numeral.position.row + 1, col: numeral.position.col }); // below
                border.insert(Position { row: numeral.position.row - 1, col: numeral.position.col - 1 }); // diagonally up
                border.insert(Position { row: numeral.position.row + 1, col: numeral.position.col - 1 }); // diagonally down
                border.insert(Position { row: numeral.position.row, col: numeral.position.col - 1 }); // to the left
            } 
            // last numeral: 5 neighbors
            else if i == self.0.len() - 1 {
                border.insert(Position { row: numeral.position.row - 1, col: numeral.position.col }); // above
                border.insert(Position { row: numeral.position.row + 1, col: numeral.position.col }); // below
                border.insert(Position { row: numeral.position.row - 1, col: numeral.position.col + 1 }); // diagonally up and right
                border.insert(Position { row: numeral.position.row + 1, col: numeral.position.col + 1 }); // diagonally down and right
                border.insert(Position { row: numeral.position.row, col: numeral.position.col + 1 }); // to the right
            }
            // middle numeral: 2 neighbors
            else {
                border.insert(Position { row: numeral.position.row - 1, col: numeral.position.col });
                border.insert(Position { row: numeral.position.row + 1, col: numeral.position.col });
            }
        });
        border
    }

    fn value(&self) -> u32 {
        // based on length of vector, calculate value
        let mut value = 0;
        self.0.iter().enumerate().for_each(|(i, numeral)| {
            value += numeral.value * 10u32.pow((self.0.len() - i - 1) as u32);
        });

        value
    }
}

fn main() {
    color_eyre::install().unwrap();
    install_tracing("info");
    info!("Starting up...");

    // 1) Read input file
    let input = read_input("../input.txt").unwrap();

    // 2) Parse input file
    let data = parse_symbols(input).unwrap();

    // 3) Process data


    // 4) Print result


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
fn parse_symbols(input: Vec<String>) -> Result<Vec<Position>> {
    let mut symbols = Vec::<Position>::new();
    
    input.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            match ch {
                '*' => symbols.push(Position { row, col }),
                '$' => symbols.push(Position { row, col }),
                '+' => symbols.push(Position { row, col }),
                '#' => symbols.push(Position { row, col }),
                _ => (),
            }
        })
    });
    
    Ok(symbols)
}

#[tracing::instrument]
fn parse_numbers(input: Vec<String>) -> Result<Vec<Number>> {
    let mut numbers = Vec::<Number>::new();
    let mut current_number = Vec::<Numeral>::new();

    input.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if let Some(digit) = ch.to_digit(10) {
                let numeral = Numeral { position: Position { row, col }, value: digit };
                current_number.push(numeral);
            } else if !current_number.is_empty() {
                numbers.push(Number(current_number.clone()));
                current_number.clear();
            }
        });

        if !current_number.is_empty() {
            numbers.push(Number(current_number.clone()));
            current_number.clear();
        };
    });

    Ok(numbers)
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

    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    #[test]
    fn test_read_input() {
        let input = read_input("../test-1.txt").unwrap();
        assert_eq!(input.len(), 10);
        assert_eq!(input[0], "467..114..");
        assert_eq!(input[9], ".664.598..");
    }

    #[test]
    fn test_parse_symbols() {
        let input = read_input("../test-1.txt").unwrap();
        let symbols = parse_symbols(input).unwrap();
        assert_eq!(symbols.len(), 6);
        assert_eq!(symbols[0].row, 1);
        assert_eq!(symbols[0].col, 3);
        assert_eq!(symbols[1].row, 3);
        assert_eq!(symbols[1].col, 6);
        assert_eq!(symbols[2].row, 4);
        assert_eq!(symbols[2].col, 3);
    }

    #[test]
    fn test_border() {
        let number = Number(vec![
            Numeral { position: Position { row: 2, col: 3 }, value: 4 },
            Numeral { position: Position { row: 2, col: 4 }, value: 6 },
            Numeral { position: Position { row: 2, col: 5 }, value: 7 },
            Numeral { position: Position { row: 2, col: 6 }, value: 8 },
            Numeral { position: Position { row: 2, col: 7 }, value: 9 },
        ]);
        let border = number.border();
        assert_eq!(border.len(), 16);
    }

    #[test]
    fn test_value() {
        let number = Number(vec![
            Numeral { position: Position { row: 2, col: 3 }, value: 4 },
            Numeral { position: Position { row: 2, col: 4 }, value: 6 },
            Numeral { position: Position { row: 2, col: 5 }, value: 7 },
            Numeral { position: Position { row: 2, col: 6 }, value: 8 },
            Numeral { position: Position { row: 2, col: 7 }, value: 9 },
        ]);
        assert_eq!(number.value(), 46789);
    }

    #[test]
    fn test_parse_numbers() {
        let input = read_input("../test-1.txt").unwrap();
        let result = parse_numbers(input).unwrap();
        
        assert_eq!(result.len(), 10); // Check if the number of numbers parsed is correct

        // Check the first number
        assert_eq!(result[0].0.len(), 3); // Check if the number of numerals in the first number is correct
        assert_eq!(result[0].0[0].value, 4); // Check the value of the first numeral of the first number
        assert_eq!(result[0].0[0].position, Position { row: 0, col: 0 }); // Check the position of the first numeral of the first number

        // Check the sixth number
        assert_eq!(result[5].0.len(), 2); // Check if the number of numerals in the sixth number is correct
        assert_eq!(result[5].0[0].value, 5); // Check the value of the first numeral of the sixth number
        assert_eq!(result[5].0[0].position, Position { row: 5, col: 7 }); // Check the position of the first numeral of the sixth number

        // Add more assertions as needed to check the other numbers and numerals
        assert_eq!(result[0].value(), 467);
        assert_eq!(result[1].value(), 114);
        assert_eq!(result[2].value(), 35);
        assert_eq!(result[3].value(), 633);
        assert_eq!(result[4].value(), 617);
        assert_eq!(result[5].value(), 58);
        assert_eq!(result[6].value(), 592);
        assert_eq!(result[7].value(), 755);
        assert_eq!(result[8].value(), 664);
        assert_eq!(result[9].value(), 598);

    }
} 