/*Rust에서는 함수나 변수 이름을 위한 관례로 스네이크 케이스(snake case) 방식을 이용함
모든 글자를 소문자로 쓰고 밑줄(underscore)로 단어를 구분하는 방식을 사용한다.
(상수는 모든 글자를 대문자로 쓰고, 밑줄로 단어를 구분 한다는 점이 차이다.)
*/
fn main() {
//     println!("Hello, world!");

//     another_fuction();
// }

// fn another_fuction() {
//     println!("Another function.");

another_fuction(5); // <- 함수의 괄호 안에 있는 것을 '매개변수(parameter)' 또는 '인수(argument)' 라고 한다.

print_labeled_measurement(5, 'h'); // 여러 매개변수를 정의하기 위해서는 쉼표(comma,)로 구분한다.

}

fn another_fuction(x: i32) { // 함수 시그니처에서는 매개변수의 타입을 반드시 지정해야 한다.
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) { // 매개변수의 타입을 지정한다.
    println!("The measurement is : {value}{unit_label}");
}

/* 함수 본문읜 필요에 따라 표현식(Expression)으로 끝나는 구문(statement)의 나열로 구성된다.
Rust는 표현식 기반의 언어이므로 구문과 표현식의 구분은 Rust를 이해하는데 중요하다.

- 구문은 어떤 동작을 수행하고 값을 반환하지 않는 명령이다.
- 표현식은 결과값을 평가한다.
*/