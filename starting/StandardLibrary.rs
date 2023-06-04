use std::collections::HashMap; // Importing the library
// Result has two types
//-> Err, an anum that contains an error code
//->Ok(value), A wrapper that contains a value
#[derive(Debug)]
enum MyError{
    Error1
}
fn divideWithRes(dividend: i32, divisor: i32) -> Result<i32, MyError>{
    if dividend % divisor != 0{
        Err(MyError::Error1)
    }
    else{
        Ok(dividend/divisor)
    }
}
// Option is an Enum that has two types
//-> None, to indicate failure or lack of value. and
//->Some(value), a tuple stuct that wraps a value with type T.

fn divide(dividend: i32, divisor: i32) -> Option<i32>{
    if dividend % divisor != 0{
        None
    }
    else{
        Some(dividend/divisor)
    }
}
fn main(){
    // vector
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0); //remove the elements at 0th index

    println!("{:?}", vec);

    // Maps
    let mut map = HashMap::new(); //initialization
    // Insertion
    map.insert(0, "Hi");
    map.insert(1, "Hi2");
    println!("{:?}", map);


    match map.get(&0){ // Pointer to the value corresponding the key 0
        Some(str1) => println!("{}", str1),
        _ => println!("Doesn't exist in map")
    }

    match map.get(&2){
        Some(str2) => println!("{}", str2),
        _ => println!("Doesn't exist in match")
    }

    map.remove(&0);
    println!("{:?}", map);

    // Accessing the Option
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // Unwrapping a 'Some' variant will extract the value wrapped
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a 'None' variant will 'panic!'
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());

    let divide3 = divideWithRes(4, 2);

    // match divide3{
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v)
    // }

    if divide3.is_ok(){
        println!("{}", divide3.unwrap());
    }

    let divide4 = divideWithRes(4, 3);

    // match divide4{
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v)
    // }

    // println!("{}", divide4.unwrap_or(404)); // if there is some error then it return 100 i.e the parameter we pass
    let res = divide4.expect("we crashed");
}