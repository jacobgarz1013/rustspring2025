fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_num=42;
    let mut guess;
    let mut attempts=0;

    loop {
        guess =30+attempts*3;
        attempts+=1;

        let result = check_guess(guess, secret_num);

        if result==0 {
            println!("{} is correct! You guessed it in {} attempts.", guess, attempts);
            break;
        } 
        else if result==1 {
            println!("{} is too high!", guess);
        } 
        else {
            println!("{} is too low!", guess);
        }
    }
}
