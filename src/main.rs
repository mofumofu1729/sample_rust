
fn func1(a: &i32) -> i32{
    return a*2;
}

fn main() {
    let s = String::from("hello");
    let t = String::from("hello");
    let u = String::from("hello");

    let v = vec![s, t, u];
    println!("{}", v[0]);
}
