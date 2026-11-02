use std::{fs};

fn main() {
    //P1
    let mut s = fs::read_to_string("input.txt");
    match s {
        Ok(s) => {
            println!("{}", s);
            let mut max_chars = 0;
            let mut max_bytes = 0;
            let mut biggest_line_by_chars = "";
            let mut biggest_line_by_bytes = "";
            for line in s.lines() {
                let mut nr_chars = 0;
                for _ in line.chars() { nr_chars += 1; }
                if max_chars < nr_chars {
                    max_chars = nr_chars;
                    biggest_line_by_chars = line;
                }

                let mut nr_bytes = 0;
                for _ in line.bytes() { nr_bytes += 1; }
                if max_bytes < nr_bytes {
                    max_bytes = nr_bytes;
                    biggest_line_by_bytes = line;
                }
            }
            println!("{} is biggest by chars, with length {}", biggest_line_by_chars, max_chars);
            println!("{} is biggest by bytes, with length {}", biggest_line_by_bytes, max_bytes);
        }
        Err(e) => { println!("Error: {}", e); }
    }

    println!();
    println!();
    //P2
    s = fs::read_to_string("input2.txt");
    match s {
        Ok(s) => {
            if s.is_ascii() {
                let mut t = String::new();
                for c in s.chars() {
                    if ('a' <= c && c < (b'a' + 13) as char) ||
                       ('A' <= c && c < (b'A' + 13) as char)
                    {
                        t.push((c as u8 + 13) as char);
                    } else if ((b'a' + 13) as char <= c && c <= 'z') ||
                              ((b'A' + 13) as char <= c && c <= 'Z')
                    {
                        t.push((c as u8 - 13) as char);
                    } else {
                        t.push(c);
                    }
                }
                println!("{}", t);
            } else { panic!("NU ESTE ASCII"); }
        }
        Err(e) => { println!("Error: {}", e); }
    }

    println!();
    println!();
    //P3
    s = fs::read_to_string("input3.txt");
    match s {
        Ok(s) => {
            let mut aux: String = s.replace(" pt ", " pentru ");
            aux = aux.replace(" ptr ", " pentru ");
            aux = aux.replace(" dl ", " domnul ");
            aux = aux.replace(" dna ", " doamna ");
            println!("{}", aux);
        }
        Err(e) => { println!("Error: {}", e); }
    }

    //P4 TODO
    s = fs::read_to_string("/etc/hosts");
    match s {
        Ok(s) => {
            println!("{}", s);
            for line in s.lines() {
                if !starts_with("#") {
                    for word in line.split_whitespace() {
                        //println!("{} => {}")
                    }
                }
            }
        }
        Err(e) => { println!("Error: {}", e); }
    }
}
