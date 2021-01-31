use std::collections::HashSet;
use std::fs;

fn parse_input(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    contents.split("\n\n").map(|x| x.to_string()).collect()
}

fn count_group(group: &str, operation: fn(HashSet<char>, &str) -> HashSet<char>) -> i32 {
    let persons: Vec<&str> = group.split('\n').filter(|x| x.len() != 0).collect();
    let res = persons[1..]
        .iter()
        .fold(persons[0].chars().collect(), |acc: HashSet<_>, a| {
            operation(acc, a)
        })
        .len() as i32;
    res
}

fn main() {
    let answers = parse_input("input.txt");
    let res = answers.iter().fold(0, |acc, g| {
        acc + count_group(g, |x, y| x.union(&y.chars().collect()).cloned().collect())
    });
    println!("{}", res);

    let res = answers.iter().fold(0, |acc, g| {
        acc + count_group(g, |x, y| {
            x.intersection(&y.chars().collect()).cloned().collect()
        })
    });
    println!("{}", res);
}
