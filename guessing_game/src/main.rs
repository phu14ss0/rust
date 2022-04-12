use std::io; // input/output 라이브러리를 스코프로 가져옴 11

fn main() {
    println!("Guess the number!");  // 문자열을 화면에 표시하는 매크로
    println!("Please input your guess.");

    // 러스트 변수는 기본적으로 불변, 가변 변수는 앞에 mut을 붙임
    // new()함수로 빈 값을 생성
    let mut guess = String::new();

    // &는 참조자, 안전성과 용이성을 위해 사용, 기본적으로 불변
    // read_line() 함수의 반환값은 io::Result
    // Result는 열거형이며 Ok와 Err variants를 가짐
    // expect()는 에러 처리
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    // {}는 변경자로 값이 표시되는 위치를 나타냄(python format과 비슷)
    println!("You guessed: {}", guess);
}