fn main(){
    let str: &str = "Hello world"; // Similar to slice. this memory i storing hello world
    let mut string: String = String :: from("Hello world"); // dynamic allocation of a string
    // The string one is a STL
    let slice = &string[..6];
    slice.len();

    string.push('!');
    string.push_str(" Bob");
    string = string.replace("Hello", "Bye");
    string = string.replace(" world", "");
    println!("{}", string);
}