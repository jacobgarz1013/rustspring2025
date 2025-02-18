fn evencheck(n: i32) -> bool {
    n%2==0
}

fn main() {
    let numbers = [12, 7, 15, 8, 25, 30, 19, 21, 5, 42]; 

    for &num in &numbers {
        if num %3==0 && num%5==0 {
            println!("FizzBuzz");
        } 
        else if num%3==0 {
            println!("Fizz");
        } 
        else if num%5==0 {
            println!("Buzz");
        } 
        else if evencheck(num) {
            println!("Even");
        } 
        else {
            println!("Odd");
        }
    }

    let mut sum=0;
    let mut i=0;
    while i<numbers.len() {
        sum+=numbers[i];
        i+=1;
    }

    println!("Sum: {}", sum);

    let mut max_num=numbers[0];
    for &num in &numbers {
        if num>max_num {
            max_num=num;
        }
    }
    println!("Largest number: {}", max_num);
}
