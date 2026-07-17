fn main(){
    let arr:[i32;5]=[1,2,3,4,5];
    let _s = &arr[1..2];
    print!("{:?}",arr.iter().position(|&x| x==3));
}