const MAX_U16: u16 = 0xFFFF;
const MAX_U32: u32 = 0xFFFF_FFFF;

fn is_prime(n: u16) -> bool {
    let mut i = 2;
    while i < n/2+1 {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    if x == MAX_U16 {
        return None;
    }
    let mut result: u16 = x+1;
    while result < MAX_U16 {
        if is_prime(result) {
            return Some(result);
        }
        result += 1;
    }
    return None;
}

fn add_panic(mut x: u32, mut y: u32) -> u32 {
    while y > 0 {
        if x == MAX_U32 {
            panic!("Numere prea mari!");
        }
        x += 1;
        y -= 1;
    }
    x
}

fn mul_panic(mut x: u32, mut y: u32) -> u32 {
    while y > 0 {
        if x == MAX_U32 {
            panic!("Numere prea mari!");
        }
        x = add_panic(x, y);
        y -= 1;
    }
    x
}

#[derive(Debug)]
enum MyError {
    Overflow,
}

fn add_result(mut x: u32, mut y: u32) -> Result<u32, MyError> {
    while y > 0 {
        if x == MAX_U32 {
            return Err(MyError::Overflow);
        }
        x += 1;
        y -= 1;
    }
    Ok(x)
}

fn mul_result(mut x: u32, mut y: u32) -> Result<u32, MyError> {
    while y > 0 {
        if x == MAX_U32 {
            return Err(MyError::Overflow);
        }
        x = add_result(x, y)?;
        y -= 1;
    }
    Ok(x)
}

enum CharError {
    Ascii, Digit, Base16, Letter, Printable
}

fn to_uppercase(ch: char) -> Result<char, CharError> {
    if 'A' <= ch && ch <= 'Z' {
        return Ok(ch);
    } else if 'a' <= ch && ch <= 'z' {
        return Ok((ch as u8 - 'a' as u8 + 'A' as u8) as char);
    } else {
        return Err(CharError::Letter);
    }
}

fn to_lowercase(ch: char) -> Result<char, CharError> {
    if 'a' <= ch && ch <= 'z' {
        return Ok(ch);
    } else if 'A' <= ch && ch <= 'Z' {
        return Ok((ch as u8 - 'A' as u8 + 'a' as u8) as char);
    } else {
        return Err(CharError::Letter);
    }
}

fn print_char(ch: char) -> Result<(), CharError> {
    if ch as u8 <= 32 {
        return Err(CharError::Printable);
    }
    println!("{}", ch);
    Ok(())
}

fn char_to_number(ch: char) -> Result<u8, CharError> {
    if ch as u8 > 127 {
        return Err(CharError::Ascii);
    }
    if ch < '0' || '9' < ch {
        return Err(CharError::Digit);
    }
    Ok(ch as u8 - '0' as u8)
}

fn char_to_number_hex(ch: char) -> Result<u8, CharError> {
    if ch as u8 > 127 {
        return Err(CharError::Ascii);
    }
    if '0' <= ch && ch <= '9' {
        return Ok(ch as u8 - '0' as u8);
    } else if 'A' <= ch && ch <= 'F' {
        return Ok(ch as u8 - 'A' as u8 + 10);
    } else if 'a' <= ch && ch <= 'f' {
        return Ok(ch as u8 - 'a' as u8 + 10);
    }
    Err(CharError::Digit)
}

fn print_error(e: CharError) {
    match e {
        CharError::Ascii => { println!("character is not ascii!") }
        CharError::Digit => { println!("character is not a digit!") }
        CharError::Base16 => { println!("character is not a base 16!") }
        CharError::Letter => { println!("character is not a letter!") }
        CharError::Printable => { println!("character is not printable!") }
    }
}

fn shout(s: String) -> Result<(), CharError> {
    let mut t = String::new();
    for c in s.chars() {
        t.push(to_uppercase(c)?);
    }
    print!("{}", t);
    Ok(())
}

fn main() {
    let n = 65_000;
    let mut i = next_prime(n);
    loop {
        match i {
            Some(x) => {
                println!("{}", x);
                i = next_prime(x);
            }
            None => {
                break;
            }
        }
    }

    //println!("add panic: {}", add_panic(0xFFFF_FF00, 0x0000_0FFF));
    //println!("mul panic: {}", mul_panic(0xFFFF_FF00, 0x0000_FFFF));

    let mut m = mul_result(0xFFFF_FF00, 0x0000_00FF);
    match m {
        Ok(x) => { println!("valoarea inmultirii: {}", x); }
        Err(e) => { println!("eroare la inmultire: {:?}", e); }
    }
    m = mul_result(0x00FF_FF00, 0x0000_00FF);
    match m {
        Ok(x) => { println!("valoarea inmultirii: {}", x); }
        Err(e) => { println!("eroare la inmultire: {:?}", e); }
    }

    let mut ch = to_uppercase('a');
    match ch {
        Ok(ch) => { print_char(ch); }
        Err(e) => { print_error(e); }
    }
    ch = to_lowercase('H');
    match ch {
        Ok(ch) => { print_char(ch); }
        Err(e) => { print_error(e); }
    }
    let mut x = char_to_number('4');
    match x {
        Ok(x) => { println!("{}", x); }
        Err(e) => { print_error(e); }
    }
    x = char_to_number_hex('f');
    match x {
        Ok(x) => { println!("{}", x); }
        Err(e) => { print_error(e); }
    }

    shout(String::from("hello friend"));
    shout(String::from("hellofriend"));
}
