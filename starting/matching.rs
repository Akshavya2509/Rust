// Similar to switch statement

fn main(){
    let i = 4;

    match i{
        0 => println!("0"),
        1 | 2 => println!("1 , 2"),
        3..=4 => println!("In range 3 to 4(inclusive)"),
        _ => println!("default")
    }
}