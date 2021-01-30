use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Seat {
    id: i32,
    row: i32,
    col: i32,
}

impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_value(size: i32, input: &str, next: char) -> i32 {
            let mut pos = 0;
            let mut step = size / 2;
            for val in input.chars() {
                if val == next {
                    pos += step;
                }
                step /= 2;
            }
            pos
        }

        // Old way
        // fn get_value2(size: i32, input: &str, next: char) -> i32 {
        //     let mut range = (0, size - 1);
        //     for val in input[..input.len() - 1].chars() {
        //         let step = (range.1 - range.0) / 2 + 1;
        //         if val == next {
        //             range.0 += step;
        //         } else {
        //             range.1 -= step;
        //         }
        //     }
        //     let step = (range.1 - range.0) / 2;
        //     if input.chars().nth(input.len() - 1).unwrap() == next {
        //         range.1
        //     } else {
        //         range.0
        //     }
        // }

        let row = get_value(128, &s[..7], 'B');
        let col = get_value(8, &s[7..], 'R');

        Ok(Seat {
            id: row * 8 + col,
            row,
            col,
        })
    }
}

fn parse_input(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    contents.split("\n").map(|x| x.to_string()).collect()
}

fn main() {
    let seats = parse_input("input.txt");
    let mut listed_seats = HashSet::new();
    let max_id = seats
        .iter()
        .map(|x| {
            let seat = Seat::from_str(x).unwrap();
            listed_seats.insert(seat.id);
            seat
        })
        .max_by_key(|s| s.id);
    println!("{:?}", max_id.unwrap());

    for id in 8..(8 * 127) {
        if !listed_seats.contains(&id)
            && listed_seats.contains(&(id - 1))
            && listed_seats.contains(&(id + 1))
        {
            println!("{:?}", id);
            return;
        }
    }
}
