/* Variables and Constant 변수와 상수
    변수 선언은 let 으로 선언한다. 기본적으로 Rust의 변수는 immutable이나, "let mut" 처럼 'mut' 을 사용하여 가변성을 부여할 수 있다. */

fn main() {
    let hello = String::from("Hello, world!");
    let apples = 5;

    println!("{hello}, {apples}");

    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    // 상수 선언 const, 상수는 immutable이며 mutable로 바꿀 수 없다.
    // 또한 Rust에서 상수명은 모두 대문자로 작성하며 단어와 단어 사이에는 _(언더바)를 사용하여 분리한다.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    /* 이 상수의 이름은 THREE_HOURS_IN_SECONDS 이며, 값은 60(분당 초의 개수), 60(시간당 분의 개수),
    3(이 프로그램에서 알아 둘 필요가 있는 시간의 숫자)를 모두 곱한 값이다. */
    // 참조 : https://doc.rust-lang.org/reference/const_eval.html //
    
    // 상수는 선언된 스코프 내에서 프로그램이 동작하는 전체 시간 동안 유효하다.

    println!("{}", THREE_HOURS_IN_SECONDS);
}
