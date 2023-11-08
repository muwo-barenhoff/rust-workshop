use std::io::BufRead;

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
    let _s: &str = "foo bar";

    // tuple
    let _tup: (u8, bool, char) = (42u8, false, 'c');

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
    // expression
    if s {
        a + 16
    } else {
        a + 32
    }
    //a + if s { 16 } else { 32}
}


#[test]
fn test_fib() {
    const N: usize = 23;
    println!("{N}-th fibonacci number is {} {}", fib(N), fib_rec(N).1);
}

pub fn fib(n: usize) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _i in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

pub fn fib_rec(n: usize) -> (u128, u128) {
    match n {
        0 => (0, 0),
        1 => (0, 1),
        _ => {
            let (a, b) = fib_rec(n - 1);
            (b, a + b)
        }
    }
}

//
// Arrays, Vektoren und Iteratoren
//
#[test]
fn _arrays_and_vectors() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let _ys: [i32; 500] = [0; 500];
    
    // A slice
    let sl = &xs[1..5];

    println!("{sl:?}");

    let mut v : Vec<i32> = Vec::new();
    v.push(42);
    v.extend_from_slice(sl);

    println!("{v:?}");

    let it = v.iter().filter(|x| *x % 2 == 0).map(|x| x + 100);
    
    //let v2 : Vec<i32> = it.collect();
    //println!("{v2:?}");

    for x in it {
        println!("{x}");
    }

}

// Lambda Expressions / Closures


#[test]
pub fn _closures() {
    let c0 = |x: u32| { x + 1};
    let r0 = c0(42);
    println!("{r0}");

    let inc_val = 2;
    let c1 = |x: u32| { x + inc_val };
    let r1 = c1(23);
    println!("{r1}");

    let v_in = vec![0,1,2,3,4,5,6,7,8,9,10];
    let v_out = filter_u32(|x| x % 2 == 0, &v_in);
    println!("{v_out:?}")
}

pub fn filter_u32(pred: impl Fn(u32) -> bool, v: &Vec<u32>) -> Vec<u32> {
    let mut res = Vec::new();
    for e in v {
        if pred(*e) {
            res.push(*e);
        }
    }
    res
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

impl S2 {
    pub fn print_s2(&self) {
        println!("S2: {}, {}", self.bool_value, self.num_value);
    }
}

//
// traits
//

trait MyEnumTrait {
    fn print_enum(&self);
}

impl MyEnumTrait for E1 {
    fn print_enum(&self) {
        // pattern matching
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




// Ownership and borrowing 

pub fn fn_a() {
    let a : u8 = 0x23;
    let b : String = "foo bar".into();

    fn_b(a,b);

    println!("{a}");
    //println!("{b}");

}

fn fn_b(_a: u8, _b: String)  {

}


// Generics 

#[derive(Debug)]
pub struct Degree<T>(T);

impl<T: PartialOrd + From<u16>> Degree<T> {
    pub fn new(a: T) -> Option<Self> {
        if a > 0u16.into() && a < 360u16.into() {
            Some(Degree(a))
        } else {
            None
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Degree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°", self.0)
    }
}

#[test]
fn test_degree() {
    let degree_opt = Degree::new(247f32);


    if let Some(d) = degree_opt {
        println!("Valid degrees {d}");
    }

    let degree_opt2 = Degree::new(456f64);

    match degree_opt2 {
        Some(Degree(d)) => {
            println!("Valid degrees {d}");
        },
        None => {
            println!("Invalid degrees.");
        }
    }

    let deg_tup = tuple_degrees(23, 42);
    println!("{deg_tup:?}");
}

/// ? operator
pub fn tuple_degrees(a: u16, b: u16) -> Option<(Degree<u16>, Degree<u16>)> {
    let a = Degree::new(a)?;
    let b = Degree::new(b)?;
    Some((a,b))
}


pub fn count_file_lines_bytes(file_path: &std::path::Path) -> Result<(usize, usize), std::io::Error> {
    let file_handle_res = std::fs::File::open(file_path);
    match file_handle_res {
        Err(err) => { 
            println!("Unable to open {file_path:?}: {err:?}");
            return Err(err);
        },
        Ok(file_handle) => {
            let mut reader = std::io::BufReader::new(file_handle);
            let mut is_eof = false;

            let mut total_bytes = 0;
            let mut total_lines = 0;
            
            while !is_eof {
                let mut line_buf = String::new();
                let res = reader.read_line(&mut line_buf);
                match res {
                    Err(err) => {
                        println!("Unable to read from {file_path:?}: {err:?}");
                        return Err(err);                        
                    }
                    Ok(bytes_read) => {
                        total_bytes += bytes_read;
                        if bytes_read == 0 {
                            is_eof = true;
                        } else {
                            total_lines += 1;
                        }
                    }
                }
            }

            Ok((total_bytes, total_lines))
        }
    }
}

pub fn count_file_lines_bytes_short(file_path: &std::path::Path) -> Result<(usize, usize), std::io::Error> {
    let file_handle = std::fs::File::open(file_path)?;
    
    let mut reader = std::io::BufReader::new(file_handle);
    let mut is_eof = false;
    let mut total_bytes = 0;
    let mut total_lines = 0;

    while !is_eof {
        let mut line_buf = String::new();
        let bytes_read: usize = reader.read_line(&mut line_buf)?;
        total_bytes += bytes_read;
        if bytes_read == 0 {
            is_eof = true;
        } else {
            total_lines += 1;
        }
    }

    Ok((total_bytes, total_lines))
}


#[test]
pub fn test_file_count() {
    let fp = std::path::Path::new("./src/main.rs");
    match count_file_lines_bytes_short(fp) {
        Ok(res) => {
            println!("{fp:?} has {} bytes in {} lines", res.0, res.1);
        },
        Err(err) => {
            println!("Error reading {fp:?}: {err:?}");
        },
    }
}