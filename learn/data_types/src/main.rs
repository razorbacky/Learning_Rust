// Rust는 타입이 고정된 언어이다.
// 모든 변수의 타입이 컴파일 시에 정해져 있어야 한다는 것.

/* 스칼라 타입들
Rust는 정수형, 부동소수점 숫자, boolean, 그리고 문자까지 총 4가지 스칼라 타입을 보유하고 있다. */

fn main() {
    let guess :u32 = "42".parse().expect("Not a number!");

    let x = 2.0; //f64

    let y: f32 = 3.0; //f32
    println!("{}", guess);
    println!("x: {}, y: {}", x, y);

    // 사칙 연산
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;

    let f: bool = false; // 명시적 유형 주석 포함

    // Strings
    // 문장은 큰 따옴표("")를 사용하고, 알파벳 개별은 작은 따옴표('')를 사용한다.
    // char 타입은 Unicode Scalar를 표현하는 값이며 ASCII 보다 많은 표현이 가능하다.
    // 억양 표시가 있는 문자, 한/일/중의 표의 문자, 이모티콘, 넓이가 0인 공백문자 모두 Rust에서는 char 타입으로 사용할 수 있다.
    // 문자는 Unicode을 위한 개념이 아니기에 인간적 직관의 "문자"와 Rust의 'char' 는 동일하지 않을 수 있다.
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // // tup, 튜플. 복합타입
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //

    let tup = (500, 6.4, 1);
    let (j, k, l) = tup;

    println!("The Value of k is : {}", k);
}
