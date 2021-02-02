use lazy_static::lazy_static;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Cell {
    Floor,
    Seat,
    Occupied,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone)]
struct Plane {
    cells: Vec<Vec<Cell>>,
    empty: HashSet<Point>,
    occupied: HashSet<Point>,
}

impl Plane {
    pub fn part_1(&self, pos: &Point) -> usize {
        lazy_static! {
            static ref DIRECTIONS: [i32; 3] = [-1, 0, 1];
        }

        let (i, j) = (pos.i as i32, pos.j as i32);
        let mut res = 0;
        for iter_l in DIRECTIONS.iter() {
            for iter_c in DIRECTIONS.iter() {
                if *iter_c == 0 && *iter_l == 0 {
                    continue;
                }
                let (new_i, new_j) = (i + iter_l, j + iter_c);
                if 0 <= new_i
                    && new_i < self.cells.len() as i32
                    && 0 <= new_j
                    && new_j < self.cells[0].len() as i32
                {
                    res += match self.cells[new_i as usize][new_j as usize] {
                        Cell::Occupied => 1,
                        _ => 0,
                    }
                }
            }
        }
        res
    }

    pub fn part_2(&self, pos: &Point) -> usize {
        lazy_static! {
            static ref DIRECTIONS: [i32; 3] = [-1, 0, 1];
        }

        let (i, j) = (pos.i as i32, pos.j as i32);
        let mut res = 0;
        for iter_l in DIRECTIONS.iter() {
            'next: for iter_c in DIRECTIONS.iter() {
                if *iter_c == 0 && *iter_l == 0 {
                    continue;
                }
                let (mut new_i, mut new_j) = (i + iter_l, j + iter_c);
                while 0 <= new_i
                    && new_i < self.cells.len() as i32
                    && 0 <= new_j
                    && new_j < self.cells[0].len() as i32
                {
                    if let Cell::Occupied = self.cells[new_i as usize][new_j as usize] {
                        res += 1;
                        continue 'next;
                    }
                    if let Cell::Seat = self.cells[new_i as usize][new_j as usize] {
                        continue 'next;
                    }
                    new_i += iter_l;
                    new_j += iter_c;
                }
            }
        }
        res
    }

    pub fn seat_round(&self, nr: usize, counter: impl Fn(&Self, &Point) -> usize) -> (Plane, usize) {
        let mut cells = self.cells.clone();
        let mut empty = self.empty.clone();
        let mut occupied = self.occupied.clone();

        let mut changes = 0;
        for pos in self.empty.iter() {
            if counter(self, &pos) == 0 {
                cells[pos.i][pos.j] = Cell::Occupied;
                occupied.insert(*pos);
                empty.remove(&pos);
                changes += 1;
            }
        }
        for pos in self.occupied.iter() {
            if counter(self, &pos) >= nr {
                cells[pos.i][pos.j] = Cell::Seat;
                empty.insert(*pos);
                occupied.remove(&pos);
                changes += 1;
            }
        }

        (Plane { cells, empty, occupied }, changes)
    }
}

impl Eq for Point {}
impl FromStr for Plane {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells: Vec<Vec<Cell>> = vec![];
        let mut empty = HashSet::new();
        let mut occupied = HashSet::new();

        let lines = s.split_whitespace();
        for (i, line) in lines.enumerate() {
            cells.push(vec![]);
            for (j, chr) in line.chars().enumerate() {
                cells[i].push(match chr {
                    '.' => Cell::Floor,
                    'L' => {
                        empty.insert(Point { i, j });
                        Cell::Seat
                    }
                    '#' => {
                        occupied.insert(Point { i, j });
                        Cell::Occupied
                    }
                    _ => return Err("Invalid cell"),
                })
            }
        }
        Ok(Plane {
            cells,
            empty,
            occupied,
        })
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut plane = Plane::from_str(&contents).unwrap();
    loop {
        let x = plane.seat_round(4, Plane::part_1);
        plane = x.0;
        if x.1 == 0 {
            dbg!(plane.occupied.len());
            break
        }
    }

    let mut plane = Plane::from_str(&contents).unwrap();
    loop {
        let x = plane.seat_round(5, Plane::part_2);
        plane = x.0;
        if x.1 == 0 {
            dbg!(plane.occupied.len());
            break
        }
    }
}
