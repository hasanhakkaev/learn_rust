const DEFAULT_SPEED: i32 = 500;
fn main() {
    // Variables
    let x = 123;
    println!("value of x: {}", x);

    // Constants
    const Y: i32 = 222;
    println!("value of Y: {}", Y);
    println!("value of DEFAULT_SPEED: {}", DEFAULT_SPEED * x);

    // Shadowing
    let z = "123";
    println!("Value of z: {}", z);
    let z: i32 = z.parse().unwrap();
    let a = z * 2;
    println!("Value of a: {}", a);
}
