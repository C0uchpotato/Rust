use scanrs::scann;

struct Rect {
    length:u32,
    width:u32,
}

fn main() {

let mut prime = Rect{
    length:0,
    width:0,
};

println!("What is the length of your rectangle?");
prime.length = scann();
println!("What is the width of your rectangle?");
prime.width = scann();

println!("The area of your rectangle is {}", area(prime.length, prime.width))

}

fn area(length:u32, width:u32) -> u32 {
let result = prime.length*prime.width;
result

}
