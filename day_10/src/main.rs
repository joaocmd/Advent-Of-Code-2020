use maplit::hashmap;
use std::collections::HashMap;
use std::fs;

fn parse_input(file: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file).unwrap();
    let mut vec = contents
        .split('\n')
        .map(|x: &str| x.trim_end().parse().unwrap())
        .collect::<Vec<i32>>();
    vec.sort_unstable();
    vec.push(vec[vec.len() - 1] + 3);
    vec
}

fn part1(adapters: &Vec<i32>) -> HashMap<i32, usize> {
    let mut res = hashmap! {
        1 => 0,
        3 => 0
    };

    let mut last = 0;
    for adapter in adapters {
        match adapter - last {
            x if x == 1 || x == 3 => {
                res.entry(x).and_modify(|c| *c += 1);
            }
            _ => (),
        }
        last = *adapter;
    }
    res
}

fn part2(input: &Vec<i32>) -> usize {
    let mut the_ways = HashMap::new();
    the_ways.insert(0, 1);
    for &v in &input[1..] {
        let val = the_ways.get(&(v - 1)).unwrap_or(&0)
            + the_ways.get(&(v - 2)).unwrap_or(&0)
            + the_ways.get(&(v - 3)).unwrap_or(&0);
        the_ways.insert(v, val);
        dbg!(&the_ways);
    }
    *the_ways.get(input.last().unwrap()).unwrap()
}

fn main() {
    let adapters = parse_input("input.txt");
    let res1 = part1(&adapters);
    dbg!(&res1);
    dbg!(res1[&1] * res1[&3]);

    let mut adapters = adapters.clone();
    adapters.insert(0, 0);
    dbg!(fraud(&adapters));
}
