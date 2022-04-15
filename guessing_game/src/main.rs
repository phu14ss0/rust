extern crate rand;

use std::io; // input/output 라이브러리를 스코프로 가져옴 11
use rand::Rng;  // Rng는 메소드들이 정의되어 있는 trait
use std::cmp::Ordering; // Ordering은 열거형 Less, Greater, Equal 값을 가짐

fn main() {
    println!("Guess the number!");  // 문자열을 화면에 표시하는 매크로

    let secret_number = rand::thread_rng().gen_range(1, 101);   // thread_rng() 함수는 OS가 시드를 정하고 현재 스레드에서만 사용되는 정수 생성기를 돌려줌
                                                                // gen_range는 Rng trait에 정의된 두 숫자 사이의 임의 값을 생성하는 메소드
    println!("The secret number is: {}", secret_number);

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

    let guess: u32 = guess.trim().parse()   // 정수형 u32로 guess 변수를 재사용함
                                            // parse()메소드는 문자열을 숫자형으로 파싱함
        .expect("Please type a number!");
    
    // {}는 변경자로 값이 표시되는 위치를 나타냄(python format과 비슷)
    println!("You guessed: {}", guess);

    // Crate(크레이트) : 코드의 묶음(package), ex) rand 크레이트 : 다른 프로그램에서 사용하기 위한 library 크레이트
    // Cargo.toml 파일에 사용할 크레이트를 의존 리스트에 추가해야 사용 가능, 크레이트를 추가라고 cargo build 명령어
    // Cargo.lock은 처음 프로그램을 빌드할 때 기준을 만족하는 모든 의존 패키지 버전을 확인하고 Cargo.lock에 기록, 이후 빌드할 때 그 안에 명시된 버전을 사용한다.
    // cargo update 명령으로 사용중인 크레이트의 업데이트된 버전이 있으면 업데이트 함

    match guess.cmp(&secret_number) {   // cmp 메소드는 두 값을 비교하며 비교 가능한 모든 것들에 대한 호출 
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}