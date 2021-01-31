use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
struct Bag {
    bags: HashMap<String, (usize, Rc<RefCell<Bag>>)>,
}

impl Bag {
    pub fn get_or_new(
        color: String,
        current_bags: &mut HashMap<String, Rc<RefCell<Bag>>>,
    ) -> Rc<RefCell<Bag>> {
        if current_bags.get(&color).is_some() {
            Rc::clone(current_bags.get(&color).unwrap())
        } else {
            let new_bag = Bag {
                bags: HashMap::new(),
            };
            let new_bag = Rc::new(RefCell::new(new_bag));
            current_bags.insert(color, new_bag.clone());
            new_bag
        }
    }

    pub fn build_and_add(description: &str, current_bags: &mut HashMap<String, Rc<RefCell<Bag>>>) {
        lazy_static! {
            static ref FIRST: Regex = Regex::new(r"^(.*) bags contain").unwrap();
            static ref CONTAINS: Regex = Regex::new(r"([0-9]+) (\w* \w*) bags?").unwrap();
        };

        let color = &FIRST.captures(description).unwrap()[1];
        let bag = Bag::get_or_new(color.to_string(), current_bags);

        CONTAINS.captures_iter(description).for_each(|caps| {
            let other_qty = caps.get(1).unwrap().as_str().parse().unwrap();
            let other_color = caps.get(2).unwrap().as_str().to_string();
            let other = Bag::get_or_new(other_color.clone(), current_bags);
            bag.borrow_mut()
                .bags
                .insert(other_color, (other_qty, other));
        });
    }
}

fn parse_input(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .split("\n")
        .map(|x| x.to_string())
        .filter(|x| x.len() != 0)
        .collect()
}

fn is_possible(bag: Rc<RefCell<Bag>>, target: &str) -> bool {
    let bags = &bag.borrow().bags;
    if bags.get(target).is_some() {
        true
    } else {
        bags.values().any(|bag| is_possible(bag.1.clone(), target))
    }
}

fn count_inside(bag: Rc<RefCell<Bag>>) -> usize {
    bag.borrow()
        .bags
        .values()
        .fold(1, |acc, bag| acc + bag.0 * count_inside(bag.1.clone()))
}

fn main() {
    let bags_description = parse_input("input.txt");
    let mut bags = HashMap::new();
    bags_description
        .iter()
        .for_each(|bag| Bag::build_and_add(bag, &mut bags));

    let res = bags
        .values()
        .filter(|bag| is_possible((*bag).to_owned(), "shiny gold"))
        .count();
    dbg!(res);

    let res = count_inside(bags.get("shiny gold").unwrap().clone());
    dbg!(res - 1);
}
