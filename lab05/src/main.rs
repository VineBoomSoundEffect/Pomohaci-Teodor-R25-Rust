use std::fs;
use serde_derive::Deserialize;

//P2: (scrie good job :))
type Canvas = [[u8;100];55];
fn new_canvas() -> Canvas {
    [['.' as u8;100];55]
}

fn print(c: Canvas) {
    for line in c {
        for cell in line {
            print!("{}", cell as char);
        }
        println!();
    }
}

fn set_pixels(c: &mut Canvas, points: &[(usize,usize,u8)]) {
    for point in points {
        c[point.0][point.1] = point.2;
    }
}

//P3:
#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let content = fs::read_to_string("src/input.json");
    match content {
        Ok(c) => {
            let mut min = 999;
            let mut max = 0;
            for line in c.lines() {
            let person: Result<Person, _> = serde_json::from_str(&line);
            match person {
                Ok(p) => {
                    if min > p.age {
                        min = p.age;
                    }
                    if max < p.age {
                        max = p.age;
                    }
                }
                Err(e) => {println!("{}", e);}
            }
            }
            println!("MIN AGE: {}; MAX AGE: {}", min, max);
        }
        Err(e) => {println!("{}", e);}
    }
}
