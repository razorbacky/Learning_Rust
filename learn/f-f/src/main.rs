fn main() {
    let mut a = 5;
    let mut b = 5;

    let pre = {
        a += 1;
        a * 3
    };
    let post = {
        let temp = b;
        b += 1;
        temp * 3
    };

    println!("증감 연산 후 초깃값 a = {}, b = {}", a, b);
    println!("전위형 : (++a) * 3 = {}, 후위형 : (b++) * 3 = {}", pre, post);
}