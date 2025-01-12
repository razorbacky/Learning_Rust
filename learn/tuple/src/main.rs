// 튜플은 복합 타입을 지원한다.

fn main() {
    let x : (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_forth = x.1;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_forth, one);

    // 배열은 여러 값들의 집합체를 만드는 방법으로 튜플과 다르게 모든 배열 요소는 같은 타입이어야 한다.
    // 벡터 타입이라는 또 다른 것이 있다.

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    let first = a[0];
    let second = a[1];

    println!("{} {} {}", first, second, first+second);
    println!("The Value of element is: {}", element);

    // 배열은 항상 고정된 요소를 가질 떄에 사용한다.
    // 일례로 1년의 달을 표기할 때는 매달 변동 없이 동일하게 움직이므로 아래와 같은 코드를 사용할 수 있다.
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("{:?}", months);

}
