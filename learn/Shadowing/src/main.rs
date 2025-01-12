// Shadowing 과 mut 은 다르다.

fn main() {

    // Shadowing
    let x = 5;

    let x  = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // mut
    let mut y = 10;

    y = 9;

    y = 11;

    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // 이 처럼 사용할 수 없다.
    let mut spaces = "   ";
    spaces = spaces.len();
}
