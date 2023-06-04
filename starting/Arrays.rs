fn main(){
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Index: {} length: {}", arr[0], arr.len());

    //Print structure of array and other objects
    println!("{:?}", other_arr); // Output : [1, 2, 3, 4, 5]
}