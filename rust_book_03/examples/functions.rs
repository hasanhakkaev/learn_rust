fn main() {
    println!("Hellow from functions!");
    another_funciton();
    func_with_args(55, 5.3);
    let y = sum(22);
    println!("value of y: {}", y);
}

// Simple function

fn another_funciton() {
    println!("Hi from anther function!");
}

// Functon with arguments
fn func_with_args(x: i32, gravity: f64) {
    println!("func with x: {}, gravity: {}", x, gravity);
}

// Function with return
fn sum(x: i32) -> i32 {
    x + 1
}
