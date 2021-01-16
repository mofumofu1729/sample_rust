
fn func1(a: &i32) -> i32{
    return a*2;
}

fn main() {
    let s = 1;

    let t = func1(&s);

    println!("{}", s);
}
