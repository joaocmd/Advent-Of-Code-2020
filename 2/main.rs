use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::*, BufReader};

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    // part 1
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        let only_whitespace = line.unwrap().replace("-", " ").replace(":", "");
        let (lower, upper, chr, pass) = scan!(
            &only_whitespace,
            |x| char::is_whitespace(x),
            u32,
            u32,
            String,
            String
        );

        let count: u32 = pass
            .unwrap()
            .matches(&chr.unwrap())
            .count()
            .try_into()
            .unwrap();
        if lower.unwrap() <= count && count <= upper.unwrap() {
            res += 1;
        }
    }
    println!("{:?}", res);

    // part 2
let file = File::open("input.txt").unwrap();
let reader = BufReader::new(file);

let mut res = 0;
for line in reader.lines() {
    let only_whitespace = line.unwrap().replace("-", " ").replace(":", "");
    let (i, j, chr, pass) = scan!(
        &only_whitespace,
        |x| char::is_whitespace(x),
        usize,
        usize,
        char,
        String
    );

    let (i, j, chr, pass) = (i.unwrap(), j.unwrap(), chr.unwrap() as u8, pass.unwrap());
    let pass_bytes = pass.as_bytes();
    if (pass_bytes[i - 1] == chr && pass_bytes[j - 1] != chr)
        || (pass_bytes[i - 1] != chr && pass_bytes[j - 1] == chr)
    {
        res += 1
    }
}
println!("{:?}", res);
}
