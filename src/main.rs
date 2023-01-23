fn main(){
    let mut  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *3;
    println!("the value of Three_Hours is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing is accomplished with the let within a scope or outside a scope.
    // variables are immutable after transfermation have been complete
    // Shadowing is creating a new variable when useing let
    let y = 5;
    let y = y+1;
    {
        let y = y * 2;
        println!("The value of the y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "    ";
    let spaces = spaces.len();

    //The following will crash because of data type mismatch
    // let mut spaces = "    ";
    // spaces = spaces.len();

    //signed and unsigned refer to whether the number can be negative.
    //sign is only positive.
    //unsigned can be negative.

    //Overflow causes the program to panic
    //Can handle overflowas with wrapping_*

    let u = 2.0; //f64
    let i: f32 = 3.0; //f32

    //Num operations

    // addition
    let sum = 5 +10;
    // subtraction
    let difference = 95.5 - 4.3;

    // mulipl
    let product = 4 * 30;

    // div
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // resulte in -1

    //remainder
    let remainder = 43 % 5;

    //Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    //Char type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let hear_eyed_cat = '😻';

    // Compound Types
    //   Tuple optional
    let tup: (i32, f64, u8) = (500, 6.4,1);
    //   Tuple
    let tup = (500, 6.4,1);
    let(j,l,p) = tup;
    println!("The Value of l is: {l}");

    let w: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = w.0;
    let six_point_four = w.1;
    let one = w.2;

    //Arrays
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // give arrays a type
    let o: [i32;5] = [1,2,3,4,5];

    //init a array with same value
    let t = [3;5]; //[3,3,3,3,3]

    //accessing an array
    let first = a[0];
    let second = a[1];

    //program panics if trying to access array out of range.
}