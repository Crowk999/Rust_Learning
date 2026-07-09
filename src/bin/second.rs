fn hello(){
    println!("Hello World Again")
}

fn sum(a:i32,b:i32)->i32{
    return a+b;
}

fn diff(a:i32,b:i32)->i32{
    return a-b;
}

fn debuger(x:i8)->i8{
    return dbg!(x*2);
}

fn main(){
    hello();
    debuger(2);
    let result = sum(1,2);
    let diff:i32 = diff(5,3);
    print!("The sum is {}",result);
    println!("The diff is {}", diff);
    eprint!("This is the error message")
}