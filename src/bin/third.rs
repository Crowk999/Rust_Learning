fn main(){
    let mut x:i8=2;
    println!("{}",x);
    x =3;
    println!("{}",x);

    for a in 1..5{
        println!("The value of a:-{}",a);
    }

    while x<=3{
        println!("The while loop");
        x += 1;
    }
}