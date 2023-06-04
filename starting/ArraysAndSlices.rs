fn main(){
    let arr = [0, 1, 2, 3];
    let slice = &arr[1..3]; // random memory of a part of array is addressed by a slice
// We dont know the length of a slice
    println!("{:?}", slice);

    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr:[u8; 4], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", arr[0], arr[1]);
}