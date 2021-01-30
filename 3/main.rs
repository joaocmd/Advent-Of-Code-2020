use std::fs;

struct Grid {
    grid: Vec<Vec<i8>>,
    x_max: usize,
    y_max: usize,
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_input(file: &str) -> Grid {
    let contents = fs::read_to_string(file).unwrap();
    let grid: Vec<Vec<i8>> = contents
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|cell| match cell {
                    '.' => 0,
                    _ => 1,
                })
                .collect()
        })
        .collect();

    let (x_max, y_max) = (grid[0].len(), grid.len());
    Grid { grid, x_max, y_max }
}

fn count_trees(grid: &Grid, start: Pos, slope: Pos) -> usize {
    let mut pos = start;
    let mut count: usize = 0;

    while pos.y < grid.y_max {
        if grid.grid[pos.y][pos.x] == 1 {
            count += 1;
        }
        pos.x = (pos.x + slope.x) % grid.x_max;
        pos.y += slope.y;
    }
    count
}

fn main() {
    let grid = parse_input("input.txt");
    // part 1
    println!(
        "{:?}",
        count_trees(&grid, Pos { x: 0, y: 0 }, Pos { x: 3, y: 1 })
    );

    //part 2
    let res = vec![
        Pos { x: 1, y: 1 },
        Pos { x: 3, y: 1 },
        Pos { x: 5, y: 1 },
        Pos { x: 7, y: 1 },
        Pos { x: 1, y: 2 },
    ].iter().map(|x| count_trees(&grid, Pos { x: 0, y: 0 }, *x)).fold(1, |x, y| x*y);

    println!("{:?}", res)

}
