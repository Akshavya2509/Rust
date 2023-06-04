fn main(){
    // let num = 5;
    // num = 3;
    // the above one gives error since rust is immutable by default

    let mut num = 5;
    num = 3;
    println!("{}", num);

}