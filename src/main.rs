fn main(){
    let pair = ('A', 65);
    let _x:i8=11;

    println!("{}",pair.0); // Accessing first element
    println!("{}",pair.1); // Accessing second element

     // Destructuring a pair.
    let (letter, _number) = pair;
    println!("{}",letter);
    println!("{}",pair.0);
}