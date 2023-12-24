extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is : {}", secret_number);

    loop {

        let mut guess = String::new(); // String 인스턴스와 연결된 가변변수 생성=

        println!("Please input your guess.");
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }



    // 터미널의 표준 입력 타입인 std::io::Stdin의 인스턴스를 돌려주고

    // .read_line(&mut guess)는 사용자로부트의 입력을 받기 위해 표준 입력 핸들에서 .read_line(&mut guess)메소드 호출
    
    // read_line은 또한 io::Result 하나의 값을 돌려줌 - Ok / Err




}
