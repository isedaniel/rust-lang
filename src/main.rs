
fn main() {
    let x = 1;
    println!("x is {x}");

    {
        let x = x + 1;
        println!("x + 1 in the inner scope is {x}");
    }

    let x = x + 2;
    println!("x + 2 after the scope is {x}");
}
