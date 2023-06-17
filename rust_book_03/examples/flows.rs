fn main() {
    let num = 110;

    // if-else
    let msg = if num < 5 {
        "small"
    } else if num < 200 {
        "normal"
    } else {
        "big"
    };
    println!("{}", msg);

    let mut counter = 0;
    loop {
        println!("again loop {}", counter);
        counter += 1;
        if counter > 2 {
            break;
        }
    }

    // while
    let a = [0, 10, 20, 30, 40];
    counter = 0;
    while counter < 3 {
        println!("again while {}", a[counter]);
        counter += 1;
    }

    //for (in iter())
    for element in a.iter() {
        println!("again for in {}", element);
        if element == &20 {
            break;
        }
    }

    // for (in range)
    for number in 0..3 {
        println!("again for {}", number);
    }
}
