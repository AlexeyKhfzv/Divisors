use std::io;
use std::time::Instant;

fn get_degree(num: &mut u128, divisor: u128, degree: u16) {
    if *num % divisor == 0 {
        *num /= divisor;
        get_degree(num, divisor, degree + 1);
    } else { println!(" ^{degree}"); }
}

pub fn get_divisors(num: u128) {
    let mut num = num;
    if num < 4 {
        println!("{num} ^1");
        return;
    }
    if num % 2 == 0 {
        num /= 2;
        print!("2");
        get_degree(&mut num, 2, 1);
    }
    if num % 3 == 0 {
        num /= 3;
        print!("3");
        get_degree(&mut num, 3, 1);
    }
    
    let mut divisor = 5;
    while divisor * divisor < num + 1 {
        if num % divisor == 0 {
            num /= divisor;
            print!("{divisor}");
            get_degree(&mut num, divisor, 1);
        }
        if num % (divisor + 2) == 0 {
            num /= divisor + 2;
            print!("{}", divisor + 2);
            get_degree(&mut num, divisor + 2, 1);
        }
        divisor += 6;
    }
    if num != 1 { println!("{num} ^1"); }
}

fn main() {
    println!("This program returns divisors of a number.\nEnter the number:");
    
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Error while reading");
    
    let num: u128 = num.trim()
        .parse()
        .expect("Please, enter an unsigned number!");
    
    println!("----------");

    let now = Instant::now();
    get_divisors(num);
    println!("----------\nFinished: {:?}", now.elapsed());
}
