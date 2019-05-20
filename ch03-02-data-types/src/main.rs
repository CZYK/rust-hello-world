fn main() {
    let guess: u8 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

//    This will not compile since it's an 8 bit integer
//    let guess: u8 = "500".parse().expect("Not a number!");
//    println!("The value of guess is: {}", guess);

    // Floating point types
    let x = 2.0; // f64
    println!("The value of x is: {}", x);

    let y: f32 = 3.0; // f32
    println!("The value of y is: {}", y);


    // Numeric Operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];


}
