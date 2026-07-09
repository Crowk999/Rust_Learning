fn hello(){
    println!("Hello World Again")
}
fn sum(a:i32,b:i32)->i32{
    return a+b;
}

fn main(){
    hello();
    let result = sum(1,2);
    print!("The sum is {}",result);
}