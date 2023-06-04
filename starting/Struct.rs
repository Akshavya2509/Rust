fn main(){
    let name = String::from("MockingBird");
    let bird = Bird{name, attack: 5}; // Initializing the object to the struct with struct Bird and let var = Bird{} with required args in {}
    bird.print_name();
}

struct Bird{
    name: String,
    attack: u64
}

impl Bird{ // Used to implement the struct created
    fn print_name(&self){
        println!("{} with attacking power {}",self.name, self.attack);
    }
}