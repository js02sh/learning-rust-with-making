fn main() {
    //1에서 100까지 반복
    for i in 1..=100 {
        //조건과 일치하는지 확인
        if i % 3 == 0 && i % 5 == 5 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
