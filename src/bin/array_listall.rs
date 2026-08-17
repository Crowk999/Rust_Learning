use std::vec;

fn main(){
    let arr:[i32;5]=[1,2,3,4,5];
    let s = &arr[1..2];
    println!("{:?}",arr.iter().position(|&x| x==4));
    for x in s{
        println!("Of the s:-{}",x);
    }
    println!("arr[2]={}",arr[2]);
    println!("{:?}",s);
    let  v: Vec<i16>=vec![11,44,333,22];
    for x in v{
    println!("v={}",x);
    }
    

}