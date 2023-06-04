fn main(){
    let a = 10;
    let b = 15;
    // let is a keyword to design variables
    //u8, u16, u32, u64, u128
    let unsigned: u8 = 10; // u8 means unsigned integer of 8 bits
    //i8, i16, i32, i64, i128
    let signed: i8 = 22; // i8 means signed integer of 8 bits
    
    let float: f32 = 2.34; // f32 means floating number of 32 bits

    println!("Float: {} unsigned: {} signed: {}", float, unsigned, signed);
    let letter = "h"; // char --
    let emoji = "\u{1F600}"; //\u{} is used to represent unicode
    let bool_val: bool = true; // bool means boolean

    println!("character: {} emoji: {} boolean: {}", letter, emoji, bool_val);
    println!("hello World {} {}", a, b);

    let addition: u64 = 22*23-33+124/2;
    let is_true: bool = true && false || true;

    println!("assignment operators: {} relational operator: {}",addition, is_true);
}