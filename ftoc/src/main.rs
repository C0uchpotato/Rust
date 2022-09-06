use std::io;
fn main() {
    println!("Please input the temp in C");
    //let index : u32 = 15;
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    match s.trim_right().parse::<u32>() {
        Ok(_i) => println!("Good input"),
        Err(_) => println!("Invalid number."),
    }
    let s: i32 = s.trim().parse().expect("Input not an integer");
    let mut out = s * 9/5;
    out = out + 32;
    println!("The temp in F is {out}");
}
