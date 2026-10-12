//P1
fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
    s
}

//P2
fn add_chars_n2(s: &mut String, c: char, n: i32) {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
}

//P3
fn add_space(s: &mut String, n: i32) {
    let mut i = 0;
    while i < n {
        s.push(b' ' as char);
        i += 1;
    }
}

fn add_str(s: &mut String, s2: &str) {
    s.push_str(s2);
}

fn pow(a: i32, b: i32) -> i32 {
    let mut r = 1;
    let mut i = 0;
    while i < b {
        r = r * a;
        i += 1;
    }
    r
}

fn add_integer(s: &mut String, mut n: i32) {
    if n == 0 { s.push(b'0' as char); }
    while n > 0 {
        let mut n2 = n;
        let mut i = 0;
        n = 0;
        while n2 >= 10 {
            n = n + (n2 % 10) * pow(10, i);
            n2 /= 10;
            i += 1
        }
        s.push((n2 as u8 + b'0') as char);
        if i != 0 && i % 3 == 0 {
            s.push(b'_' as char);
        }
    }
}

fn add_float(s: &mut String, n: f32) {
    let int_n = n as i32;
    add_integer(s, int_n);
    s.push(b'.' as char);
    let mut frac_n = n as f32 - int_n as f32;
    let mut i = 0;
    while i < 3 {
        frac_n *= 10f32;
        if frac_n as i32 == 0 { add_integer(s, 0); }
        i += 1;
    }
    add_integer(s, frac_n as i32);
}

//                                        I 💚
//                                        RUST.
//
//    Most            crate      306_437_968           and     lastest         is
//         downloaded        has             downloads     the         version    2.038.
                    
fn main() {

    let mut s = String::from("");
    add_integer(&mut s, 12345678);
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        //add_chars_n2(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);

    println!();
    s = String::from("");
    add_space(&mut s, 40);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n");
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "latest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038f32);
    add_str(&mut s, ".");
    println!("{}", s);
}
