
fn func1(a: &i32) -> i32{
    return a*2;
}

fn main() {
    let s = 1;

    let t = func1(&s);

    println!("{}", s);

    let v = vec![4, 2, 1];
    let a: i32 = v[0];
    println!("{}", a);

    let b = v[0];
    println!("{}", b);
    
    let v = s;
    println!("{}", s);
    println!("{}", v);
}
