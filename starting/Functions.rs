fn main(){
    println!("{}", is_even(2));
}

pub fn is_even(num: u8) -> bool{
    let digit = num % 2;
    digit == 0 // return value of a function doesnt include a semicolon
}