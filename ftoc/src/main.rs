use std::io::stdin;
fn main() {
    println!("Please input the temp in C");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    match s.trim().parse::<u32>() {
        Ok(_i) => println!("Good input"),
        Err(_) => println!("Invalid number."),
    }
    let s: i32 = s.trim().parse().expect("Input not an integer");
    let mut out = s * 9/5;
    out = out + 32;
    println!("The temp in F is {out}");
}
