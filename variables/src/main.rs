fn main() {
    // 기본 변수는 불변성, 가변 변수를 사용하려면 접두어 mut을 사용해야 한다.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y+1;
    let y = y*2;
    println!("The value of y is: {}", y);
}