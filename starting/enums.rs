fn main(){
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C{x: 10, y: 20};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(val) = b{
        println!("{}", val);
    }

    if let MyEnum::C{x, y} = c{
        println!("{} {}", x, y);
    }
}
// Objects that represent a ccertain values
// In rust, it can store additional values
#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C {x: i32, y: i32}//Struct
}