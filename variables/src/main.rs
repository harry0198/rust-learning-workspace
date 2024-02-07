const TWO: u32 = 1 + 1;
fn main() {
    variables();
    data_types();
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number");

    // floats
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    // minus
    let difference = 95.5 - 4.3;
    println!("Difference {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Multiplication {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // result is -1
    println!("Division precision {quotient}");
    println!("Division precisionless {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder {remainder}");

    // boolean
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // tuple - does not care for identical types. Can all be different.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // tuples love pattern matching
    let (x, y, z) = tup;
    println!("The value of y is {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    // Or.. why not do indexed access.
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrays
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // type is [type, num of elements]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // or for conciceness, make there be 5 elements of 3
    let a = [3; 5];

    // Acccess is same to java and most languages.
    let first = a[0];
    let second = a[1];
    
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]); 
    
}

fn variables() {
    // Mutable variable - requires "mut" keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Constant value is {TWO}");


    // Shadowing
    // Shadowed variables are only visible within their scope.
    // I.e. not a "rewrite" of the variable
    // Not related to mutability.
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing allows for different types. i.e. string and usize
    let spaces = "    ";
    let spaces = spaces.len();

    // Mutability does not...
    // let mut spaces = "    ";
    // spaces = spaces.len();
}