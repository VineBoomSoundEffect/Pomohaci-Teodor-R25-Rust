fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i: u32 = 2;
    while i < (n / 2 + 1) {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}

fn coprime(n: u32, m: u32) -> bool {
    if n == 0 || m == 0 {
        return false;
    }
    let mut i: u32 = 2;
    while i <= if n < m {n} else {m} {
        if n.is_multiple_of(i) && n.is_multiple_of(i) {
            return false;
        }
        i += 1
    }
    true
}

fn beer() {
    let mut i = 99;
    loop {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer.", i);
        println!("Take one down, pass it around,");
        i -= 1;
        if i > 0 {
            println!("{} bottles of beer on the wall.", i);
        } else {
            println!("No bottles of beer on the wall.");
            break
        }
        println!();
    }
}

fn main() {
    const N: u32 = 10;
    {
        let mut i = 0;
        while i < N {
            if is_prime(i) {
                println!("{} is prime", i);
            } else {
                println!("{} is not prime", i);
            }
            i += 1;
        }
    }
    {
        let mut i = 0;
        while i < N {
            let mut j = 0;
            while j < N {
                if coprime(i, j) {
                    println!("{} and {} are coprime", i, j);
                } else {
                    println!("{} and {} are not coprime", i, j);
                }
                j += 1;
            }
            i += 1
        }
    }
    {
        beer();
    }
}
