// use std::{intrinsics::{sqrtf32, sqrtf64}, io};
use std:: io;
//unit test
//test
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sq_root_test(){
        assert_eq!(square_root(4 as f32),2 as f32);
    }

    #[test]
    fn factorial_test(){
        assert_eq!(fact(4),24);
    }

    #[test]
    fn log_test(){
        assert_eq!(log_base_e(10 as f32),2.3025851);
    }

    #[test]
    fn power_test(){
        assert_eq!(power(2 as f32,3),8 as f32);
    }
}

fn fact (x:i32) -> i32 {
    if x==0 {
        return 1;
    }
    else if x<0 {
        return -1;
    }

    let mut res = 1;
    for i in 1..=x {
        res *= i;
    }

    res
}

fn square_root(x : f32)->f32{
    x.sqrt()
}

fn log_base_e(x:f32)->f32{
    x.ln()
}

fn power(x:f32, y:i32)->f32{
    x.powi(y)
}

fn main() {
    loop {
        println!("");
        println!("Enter your choice: ");
        println!("1. Square root");
        println!("2. Factorial");
        println!("3. Natural logarithm");
        println!("4. Power function");
        println!("5. Exit");
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read a line");
    
        let choice:u8 = choice.trim().parse().expect("Not a number");
        match choice{
            1 => {
                println!("Enter a number: ");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read a line");
                let x:f32 = x.trim().parse().expect("Not a number");
                if x <= (0 as f32){
                    println!("Undefined")
                }
                else{
                    println!("Square root of {} is {}",x,square_root(x));
                }
            }
    
            2 => {
                println!("Enter a number: ");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read a line");
                let x:i32 = x.trim().parse().expect("Not a number");
                let y = fact(x);
                if y == -1{
                    println!("Not defined");
                }
                else {
                    println!("Factorial of {} is {}",x,fact(x));
                }
            }
    
            3 => {
                println!("Enter a number: ");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read a line");
                let x:f32 = x.trim().parse().expect("Not a number");
                if x<= (0 as f32){
                    println!("Undefined");
                }
                else{
                    println!("Natural log of {} is {} ",x,log_base_e(x));
                }
            }
    
            4 => {
                println!("Enter base number: ");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read a line");
                let x:f32 = x.trim().parse().expect("Not a number");
                println!("Enter exponent number: ");
                let mut y = String::new();
                io::stdin().read_line(&mut y).expect("Failed to read a line");
                let y:i32 = y.trim().parse().expect("Not a number");
                
                println!("{} to power {} is {} ",x,y,power(x, y));
            }
            
            5 => break,

            _=> println!("Invalid choice")
        }
    }
}
