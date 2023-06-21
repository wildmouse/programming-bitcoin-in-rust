use std::{fmt::Error, ops::{Add, Sub}};

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
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields")
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement { num, prime: self.prime }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot sub two numbers in different Fields")
        }
        let num = (self.num - other.num) % self.prime;
        FieldElement { num, prime: self.prime }
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
