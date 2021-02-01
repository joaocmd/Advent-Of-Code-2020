use std::collections::VecDeque;
use std::fs;

fn parse_input(file: &str) -> Vec<i64> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .split('\n')
        .map(|x: &str| x.parse().unwrap())
        .collect()
}

fn in_deque(el: i64, deque: &VecDeque<i64>) -> bool {
    for x in deque {
        for y in deque {
            if x + y == el {
                return true;
            }
        }
    }
    return false;
}

fn part2(total: i64, numbers: &Vec<i64>) -> Option<i64> {
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut j = i + 1;
        while sum < total && j < numbers.len() {
            sum += numbers[j];
            if sum == total {
                return Some(numbers[i] + numbers[j]);
            }
            j += 1;
        }
    }
    None 
}

fn part1(numbers: &Vec<i64>) -> Option<i64> {
    let mut deque: VecDeque<i64> = VecDeque::new();
    for i in 0..25 {
        deque.push_back(numbers[i]);
    }

    for i in 25.. {
        let el = numbers[i];
        if !in_deque(el, &deque) {
            return Some(el)
        }
        deque.push_back(el);
        deque.pop_front();
    }
    None
}

fn main() {
    let numbers = parse_input("input.txt");
    let part1 = part1(&numbers).unwrap();
    dbg!(part1);
    let part2 = part2(part1, &numbers).unwrap();
    dbg!(part2);
}
