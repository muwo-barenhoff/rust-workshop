fn main() {
    println!("Hello, world!");
}

//
// Primitive Datentypen
//

fn _data_types_and_literals() {
    // Integer Werte i8/u8/i16/u16/.../i128/u128
    let _a = 8;
    let _b = 9u8;
    let _c: u8 = 0xff;
    let _d = 65_5535;
    let _e: u8 = 0b1111_0000;
    let _f: i128 = - 23;



    // Float Werte
    let _ff1: f32 = 0.23;
    let _ff2: f64 = 0.42;

    // Bool
    let _t = true;
    let _f = false;

    // char
    let _ch2 = 'a';
    let _ch2 = '😈';

    // str
    let _s = "foo bar";

    // tuple
    let _tup: (u8, bool, char) = (42u8, false, 'c');

    // arrays
    let _a1: [u64; 4] = [1, 2, 3, 4];
}

//
// Funktionen
//

fn _f(_a: bool) -> () {}

fn _inc_16(a: u32) -> u32 {
    // explicit return
    return a + 16;
}

fn _inc_16_or_32(s: bool, a: u32) -> u32 {
    if s {
        a + 16
    } else {
        a + 32
    }
    //a + if s { 16 } else { 32}
}

//
// Loops
//
#[test]
fn loop_test() {
    let mut counter = 0;
    let res = loop {
        counter += 1;

        if counter > 12 {
            break counter * 2;
        }
    };
    println!("Result is {res}");
}

//
// structs / enums
//

pub struct S0;

pub struct S1(u8, bool);

pub struct S2 {
    pub bool_value: bool,
    pub num_value: i32,
}

pub enum E1 {
    C1(u8),
    C2(bool),
    C3 { a: u8, b: u8 },
}

//
// traits
//

trait MyEnumTrait {
    fn print_enum(&self);
}

impl MyEnumTrait for E1 {
    fn print_enum(&self) {
        match self {
            E1::C1(v) => {
                println!("C1 with {v}");
            }
            E1::C2(v) => {
                println!("C2 with {v}");
            }
            E1::C3 { a, b } => {
                println!("C3 with {a} {b}");
            }
        }
    }
}

#[test]
pub fn _struct_enums() {
    let _s0 = S0;
    let _s1 = S1(42, false);
    let _s2 = S2 {
        bool_value: false,
        num_value: 23,
    };

    let _e1 = E1::C1(2);
    let _e2 = E1::C2(false);

    match _e2 {
        E1::C1(v) => {
            println!("C1 with {v}");
        }
        E1::C2(v) => {
            println!("C2 with {v}");
        }
        E1::C3 { a, b } => {
            println!("C3 with {a} {b}");
        }
    }

    _e1.print_enum();
}

#[test]
fn test_fib() {
    const N: usize = 23;
    println!("{N}-th fibonacci number is {} {}", fib(N), fib_rec(N).1);
}

fn fib(n: usize) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _i in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn fib_rec(n: usize) -> (u128, u128) {
    match n {
        0 => (0, 0),
        1 => (0, 1),
        _ => {
            let (a, b) = fib_rec(n - 1);
            (b, a + b)
        }
    }
}
