fn main() {
    let mut n: i32 = 100;
    n += 1;
    {
        let n = 50;
        println!("{}", {n});
    }
    println!("{}", {n});
}
