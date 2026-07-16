fn main(){
    let a=47
    ;
    fibo(a);
}

fn fibo(a:i32){
    let mut i:i32=0;
    let mut j:i32=1;
    for _ in 1..=a {
        print!("{} ", i);
        let temp = i + j;
        i = j;
        j = temp;
    }
}