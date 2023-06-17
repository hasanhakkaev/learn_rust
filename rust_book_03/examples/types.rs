fn main() {
    // Integer (scalar type)
    // signed i8,i16,i32,i64,i128
    // unsigned u8,816,u32,u64,u128

    let x: i64 = 123;
    println!("Value of x: {}", x);

    // Floating (scalar type)
    let y = 321.123;
    println!(" Value of y: {}", y);

    // Numeric operations
    let z = x as f32 / 2.0;
    println!("Value of z: {}", z);

    // Boolean (scalar type)
    let happy = true;
    let coding = false;
    println!("happy coding? {}", happy && coding);

    // Char (scalar type)
    let c: char = 'c';
    println!("value of c: {}", c);

    // Tuple (compund type)
    let point3d_with_gravity = (1, 2, 3.4);
    println!("value of point3d: {:?}", point3d_with_gravity);
    println!("alue of gravity: {}", point3d_with_gravity.2);

    // Array (compound type)
    let a: [i64; 5] = [1, 2, 3, 4, 5];
    println!("value of a: {:?}", a);

    let ski_months = ["Dec", "Jan", "Feb", "Mar", "Apr"];
    println!("value of ski months: {:?}", ski_months);
    println!("first ski month: {}", ski_months[0]);

    let b: [i32; 6] = [12; 6];
    println!("value of b {:?}", b);
}
