
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
     struct User{
        name:String,
        age:u8,
    }   

    let user = User{
        name:"Adhrit".to_string(),
        age:16,
    };

    println!("The Struct Name:{} and age:{}",user.name,user.age);
}