use std::{
    fmt::Error,
    ops::{Add, Mul, Sub},
};

pub struct Point {
    a: i32,
    b: i32,
    x: Option<i32>,
    y: Option<i32>,
}

impl Point {
    fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Result<Option<Self>, &'static str> {
        if x.is_none() && y.is_none() {
            return Ok(None);
        }
        if x.is_none() ^ y.is_none() {
            return Err("not on the curve");
        }
        if y.unwrap().pow(2) != x.unwrap().pow(3) + a * x.unwrap() + b {
            Err("not on the curve")
        } else {
            Ok(Some(Point { a, b, x, y }))
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not on the same curve")
        }

        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }

        todo!("non infinite point addition"); 
        return Point {
            x: None,
            y: None,
            a: self.a,
            b: self.b,
        };
    }
}

impl PartialEq for Point { 
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

#[test]
fn test_point() {
    let p1 = Point::new(Some(-1), Some(-1), 5, 7);
    match p1 {
        Ok(p) => {
            let pp = p.unwrap();
            assert_eq!(pp.x.unwrap(), -1);
            assert_eq!(pp.y.unwrap(), -1);
        }
        Err(e) => {
            assert!(false)
        }
    }

    let p2 = Point::new(Some(-1), Some(-2), 5, 7);
    match p2 {
        Ok(p) => {
            assert!(false)
        }
        Err(e) => {
            assert_eq!(e, "not on the curve")
        }
    }

    let p3 = Point::new(Some(18), Some(77), 5, 7);
    match p3 {
        Ok(p) => match p {
            Some(pp) => {
                assert_eq!(pp.x.unwrap(), 18);
                assert_eq!(pp.y.unwrap(), 77);
            }
            None => {
                assert!(false)
            }
        },
        Err(e) => {
            assert!(false)
        }
    }

    let p4 = Point::new(None, None, 5, 7);
   
}

pub struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    fn new(num: u32, prime: u32) -> Result<Self, &'static str> {
        if num >= prime {
            Err("num is not in field range")
        } else {
            let num = num % prime;
            Ok(FieldElement { num, prime })
        }
    }

    fn represent(&self) -> String {
        format!("FieldElement_{}({})", self.prime, self.num)
    }

    fn pow(self, exponent: u32) -> Self {
        let num = self.num.pow(exponent) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields")
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot sub two numbers in different Fields")
        }
        let num = (self.num - other.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot mul two numbers in different Fields")
        }
        let num = (self.num * other.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

#[test]
fn test_field_element() {
    let fm = FieldElement::new(1, 3).unwrap();
    assert_eq!(fm.num, 1);
}

#[test]
fn test_field_element_init_error() {
    let fm = FieldElement::new(3, 3);
    match fm {
        Ok(fm) => {
            assert!(false)
        }
        Err(e) => {
            assert_eq!(e, "num is not in field range")
        }
    }
}

#[test]
// test field element with prime 57
fn test_field_element_with_prime_57() {
    let fm1 = FieldElement::new(44, 57).unwrap();
    let fm2 = FieldElement::new(33, 57).unwrap();
    let result = fm1 + fm2;
    assert_eq!(result.num, 20);

    let fm3 = FieldElement::new(9, 57).unwrap();
    let fm4 = FieldElement::new(29, 57).unwrap();
    let result = fm3 + fm4;
    assert_eq!(result.num, 38);
}

#[test]
// assert added two field elements are equal to another field element
fn test_field_element_add() {
    let fm1 = FieldElement::new(7, 13).unwrap();
    let fm2 = FieldElement::new(12, 13).unwrap();
    let fm3 = FieldElement::new(6, 13).unwrap();
    assert!(fm1 + fm2 == fm3);
}

#[test]
// test mul field element
fn test_field_element_mul() {
    let fm1 = FieldElement::new(3, 13).unwrap();
    let fm2 = FieldElement::new(12, 13).unwrap();
    let fm3 = FieldElement::new(10, 13).unwrap();
    assert!(fm1 * fm2 == fm3);
}

#[test]
// test pow field element
fn test_field_element_pow() {
    let fm1 = FieldElement::new(3, 13).unwrap();
    let fm2 = FieldElement::new(1, 13).unwrap();
    assert!(fm1.pow(3) == fm2);
}

#[test]
fn test_field_element_represent() {
    let fm = FieldElement::new(1, 3).unwrap();
    assert_eq!(fm.represent(), "FieldElement_3(1)");
}

#[test]
fn test_field_element_is_equal_to() {
    let fm1 = FieldElement::new(1, 3).unwrap();
    let fm2 = FieldElement::new(1, 3).unwrap();
    let fm3 = FieldElement::new(2, 3).unwrap();
    assert!(fm1 == fm2);
    assert!(fm1 != fm3);
}

fn main() {
    println!("Hello, world!");
}
