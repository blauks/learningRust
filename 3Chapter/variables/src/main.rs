fn main() {
    //let x = 5;
    //println!("x is: {}", x);
    //x = 6;
    //println!("x is: {}", x); Dette funker ikke

    let mut x = 5;
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);

    // Dette funker siden man gjør x til immutable

    const BIG_NUMBER: u32 = 112;
    println!("The big number is {}", BIG_NUMBER);

    let eight_bit: i8 = 1;
    let sixteen_bit: i16 = 2;
    let thirtytwo_bit: i32 = 4; //i32 default
    let sixtyfour_bit: i64 = 8;
    let hundredtwentyeight_bit: i128 = 16;
    // These are the integer types, can also use u insted of i for unsigned
    // isize and usize can also be used, then the size will be different on different PCs
    // You can write integers in Decimal, Hex, Octal, Binary and Byte
    // In all except byte you can use _ as a seperator

    let float_number: f32 = 1.4;
    // can also be f64 which is default

    let boolean: bool = true;
    // ehh pretty self explanatory

    let character = 'a';
    // a char uses single quotes, can be emojies!

    let tup = (100, 'x', 6.9);
    let (a, b, c) = tup;
    println!("{}", a);

    let tupTwo: (i8, i16, bool) = (100, 300, false);
    // Tuple
    // Can be access using a dot (.)

    let threehundred: i16 = tupTwo.1;

    let arr = [1, 2, 4, 5];
    // In arrays all the elements has to be of the same type
    // Cannot add or remove items from an array

    let arr_two: [i8; 2] = [1, 4];
    // i8 defines what datatype, and 2 defines how many items
    // You access an element in an array like you do in any other programming language arrTwo[0] for example

    // Rust will panic if you try to access an element that is out of range
    this_is_a_function("Hello!", 16);
    println!("{}", give_me_number());

    let if_example = if tupTwo.2 { 2 } else { 1 };
}

fn this_is_a_function(p: &str, y: i32) {
    let x = p;

    let z = {
        let x: i32 = y;
        x + 4
    };

    // x+4 is written without a semicolon, which means that it is an expression. This is done to "return" the value after calculation

    println!("{} : {}", x, z);
}

fn give_me_number() -> i32 {
    // a function that returns a 32 bit signed integer
    16 // final expression
}

// in functions you must always write the data type
// The final expression is the returned value, can return early if you use return

// if for while og loop, funker som det gjør i basically alle andre programmerings språk :)
