use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let values: Vec<i32> = contents.split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // part 1
    'outer: for x in &values {
        for y in &values {
            if x + y == 2020 {
                println!("{:?}", x*y);
                break 'outer
            }
        }
    }

    // part 2
    for x in &values {
        for y in &values {
            for z in &values {
                if x + y + z == 2020 {
                    println!("{:?}", x*y*z);
                    return
                }
            }
        }
    }
}
