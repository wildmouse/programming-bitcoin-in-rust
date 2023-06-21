use std::fmt::Error;

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

    fn is_equal_to(&self, other: Option<&FieldElement>) -> bool {
        match other {
            Some(other) => self.num == other.num && self.prime == other.prime,
            None => false,
        }
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
fn test_field_element_represent() {
    let fm = FieldElement::new(1, 3).unwrap();
    assert_eq!(fm.represent(), "FieldElement_3(1)");
}

#[test]
fn test_field_element_is_equal_to() {
    let fm1 = FieldElement::new(1, 3).unwrap();
    let fm2 = FieldElement::new(1, 3).unwrap();
    let fm3 = FieldElement::new(2, 3).unwrap();
    assert!(fm1.is_equal_to(Some(&fm2)));
    assert!(!fm1.is_equal_to(Some(&fm3)));
    assert!(!fm1.is_equal_to(None));
}

fn main() {
    println!("Hello, world!");
}
