fn main(){
    for i in 0..6{
        print!("{} ", i);
    }
    println!();
    let mut i = 0;
    while i < 5{
        print!("{} ",i);

        if i == 3{
            println!();
            println!("exit");
            break;
        }

        i += 1;
    }
}