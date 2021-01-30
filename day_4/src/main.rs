use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn parse_input(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    contents.split("\n\n").map(|x| x.to_string()).collect()
}

fn has_all_fields(passport: &String) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields
        .iter()
        .all(|field| passport.matches(field).count() == 1)
}

fn check_validity(passport: &String) -> bool {
    lazy_static! {
        static ref FIELD_MATCHERS: Vec<Regex> = vec![
            Regex::new(r"byr:(19[2-9][0-9]|200[0-2])(\s|$)").unwrap(),
            Regex::new(r"iyr:(201[0-9]|2020)(\s|$)").unwrap(),
            Regex::new(r"eyr:(202[0-9]|2030)(\s|$)").unwrap(),
            Regex::new(r"hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)(\s|$)").unwrap(),
            Regex::new(r"hcl:(#[0-9a-f]{6})(\s|$)").unwrap(),
            Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)(\s|$)").unwrap(),
            Regex::new(r"pid:([0-9]{9})(\s|$)").unwrap(),
        ];
    }

    FIELD_MATCHERS
        .iter()
        .all(|field_matcher| field_matcher.is_match(passport))
}

fn main() {
    let passports = parse_input("input.txt");
    let methods: [fn(&String) -> bool; 2] = [has_all_fields, check_validity];

    methods.iter().for_each(|fun| {
        let res = passports
            .iter()
            .fold(0, |acc, pass| if fun(pass) { acc + 1 } else { acc });
        println!("{:?}", res);
    })
}
