fn main() {
    /* let x = (let y = 6);
              //^^^ 구문 시작
    위 예제는 그 자체로 구문에 해당된다.
    구문은 값을 반환하지 않는다.
    따라서, let 구문을 다른 변수에 할당하려고 하면 에러가 발생한다.
    println!("{}", y);

    let y = 6 구문은 값을 반환하지 않으므로 x 의 값으로 바인딩 시킬 수가 없다.
    이것은 C나 Ruby와 다른 언어와의 차이점. 이런 언어들은 x = y = 6 이라고 작성하여 x와 y에 모두 6을 대입할 수 있지만, Rust에서는 불가능하다.

    - 표현식은 결과 값을 반환한다.
        let x = 3 + 5; // "3 + 5" 는 표현식(결과값 8)
    - 구문은 값을 반환하지 않고, 작업만 수행한다.
        let x = 10; // 변수 선언 구문
        */
    let y = {
        let x = 3;
        x + 1 // 표현식에서는 종결을 나타내는 세미콜론(;)을 사용하지 않는다.
        // 만약 세미콜론을 사용하게 되면, 표현식은 구문으로 변경되고 값을 반환하지 않는다.
    };
    println!("The Value of y is : {y}");
}
