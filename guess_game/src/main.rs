//////////////////////
// web code에서 테스트함.
// 웹 코드에서는 cargo doc --open 이 안됨.


// 라이브러리
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range(start..=end)
    // .thread_rng, 1~100 사이의 난수를 생성함
    // .gen_range, start..=end 사이의 범위를 지정함

    // println!("The secret number is : {secret_number}"); // 정상적으로 secret_number를 출력하는지 확인하는 코드

    loop { // 반복문

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
        // std::io::stdin 라이브러리 없이 개별 선언
            .read_line(&mut guess) // &mut guess의 인자를 .read_line의 인수로 전달
            .expect("Failed to read line"); // 오류 처리


            // ixx : 부호를 갖는 정수 타입 (부호 표현 가능) . -1, +2, -3, +4, -5 (음/양수)
            // uxx : 부호를 갖지 않는 정수 타입 (부호 표현 불가) . 1, 2, 3, 4, 5
            // fxx : 실수 타입

        // let guess: u64 = guess.trim().parse().expect("Please type a number!");
        // 이전에 생성된 guess 변수값을 새로운 값으로 가리는 Shadowing

        // 오류 처리 추가
        let guess: u64 = match guess.trim().parse() { // .expact 를 match 표현식으로 교체
            Ok(num) => num, // 배리언트가 OK 인 경우
            Err(_) => {println!{"Error, Not Number Type. input Number type"}; // 배리언트가 Err 인 경우
            continue}
        };
        // 오류 처리 방식 : 프로그램을 종료하는 대신 오기입한 값을 무시하고, 다시 요청

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // guess와 &secret_number 를 비교
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("Matched, You Win!");
                    break; // 정답이 일치하면 loop 종료
            }
        }
    }
}