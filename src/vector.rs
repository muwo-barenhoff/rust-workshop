
use core::ops::{Add, AddAssign};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleV2 {
    x: f64,
    y: f64,
}

impl SimpleV2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn add_v(&self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }

    pub fn add_v_assign(&mut self, rhs: SimpleV2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    pub fn scalar_mul(&self, rhs: f64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }

    pub fn scalar_mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
} 
impl Add for SimpleV2 {
    type Output = SimpleV2;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_v(rhs)
        // SimpleV2 { x: self.x + rhs. x, y: self.y + rhs.y }
    }
}

impl AddAssign for SimpleV2 {
    fn add_assign(&mut self, rhs: Self) {
        self.add_v_assign(rhs)
        // self.x += rhs.x;
        // self.y += rhs.y;
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2<T> {
    pub x: T,
    pub y: T,
}

impl From<SimpleV2> for V2<f64> {
    fn from(value: SimpleV2) -> Self {
        Self { x: value.x, y: value.y }
    }
}

impl<T: AddAssign> AddAssign for V2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Add> Add for V2<T> {
    type Output = V2<<T as Add<T>>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        V2 { x, y }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V<const DIM: usize, T> ([T; DIM]);

impl<const DIM: usize, T: Copy + Default> V<DIM, T> {
    pub fn new() -> Self {
        V([T::default(); DIM])
    }
}

impl<T> From<V2<T>> for V<2, T> {
    fn from(value: V2<T>) -> Self {
        V([value.x, value.y])
    }
}

impl <const DIM: usize, T: Add + Copy + Default> Add for V<DIM, T> 
  where <T as Add>::Output: Default + Copy
{
    type Output = V<DIM, <T as Add<T>>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = V::new();

        for i in 0 .. DIM {
            res.0[i] = self.0[i] + rhs.0[i];
        }

        res
    }
}


impl <const DIM: usize, T: AddAssign + Copy> AddAssign for V<DIM, T> 
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0 .. DIM {
            self.0[i] += rhs.0[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_simple_v2() {
        let mut a = SimpleV2 { x: 10.0 , y: 20.0  };
        let b = SimpleV2 { x: 30.0 , y: 40.0 };
        
        let c = a + b;
        println!("{c:?}");
        assert_eq!(c, SimpleV2 { x: 40.0, y: 60.0});

        a += b;
        println!("{a:?}");
        assert_eq!(a, c);
        
    }

    #[test]
    fn test_add_v2_float() {
        let mut a = V2 { x: 10.0 , y: 20.0  };
        let b =  SimpleV2 { x: 30.0 , y: 40.0 }.into();
        
        let c = a + b;
        println!("{c:?}");
        assert_eq!(c, V2 { x: 40.0, y: 60.0});

        a += b;
        println!("{a:?}");
        assert_eq!(a, c);
        


    }

    #[test]
    fn test_add_v2_int() {
        let mut a = V2 { x: 10 , y: 20  };
        let b = V2 { x: 30 , y: 40 };
        
        let c = a + b;
        println!("{c:?}");
        assert_eq!(c, V2 { x: 40, y: 60});

        a += b;
        println!("{a:?}");
        assert_eq!(a, c);
        
    }

    #[test]
    fn test_add_v_int() {
        let mut a = V([10,20]);
        let b = V2 { x: 30 , y: 40 }.into();
        
        let c = a + b;
        println!("{c:?}");
        assert_eq!(c, V([40,60]));

        a += b;
        println!("{a:?}");
        assert_eq!(a, c);
    }

    #[test]
    fn test_add_v_float() {
        let mut a = V([10.0,20.0]);
        let b = V([30.0,40.0]);
        
        let c = a + b;
        println!("{c:?}");
        assert_eq!(c, V([40.0,60.0]));

        a += b;
        println!("{a:?}");
        assert_eq!(a, c);
    }
}