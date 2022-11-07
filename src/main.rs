use std::{rc::Rc, cell::RefCell};

#[allow(unused)]
struct S1<'a> {
    s2: Option<&'a Rc<RefCell<S2<'a>>>>,
    v1: i32
}

impl<'a> S1<'a> {
    fn new(s2: Option<&'a Rc<RefCell<S2<'a>>>>) -> Self {
        Self {
            s2,
            v1: 0,
        }
    }

    fn add(&mut self, val: i32) {
        self.v1 += val;
    }
}

#[allow(unused)]
struct S2<'a> {
    s1: Option<&'a Rc<RefCell<S1<'a>>>>,
    v2: i32
}

impl<'a> S2<'a> {
    fn new(s1: Option<&'a Rc<RefCell<S1<'a>>>>) -> Self {
        Self {
            s1,
            v2: 0,
        }
    }

    fn add(&mut self, val: i32) {
        self.v2 += val;
    }
}


fn main() {
    let mut s1 = S1::new(None);
    s1.add(-1);
    let mut s2 = S2::new(None);
    s2.add(1);

    println!("s1.v1={} s2.v2={}", s1.v1, s2.v2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut s1 = S1::new(None);
        assert_eq!(s1.v1, 0);
        s1.add(1);
        assert_eq!(s1.v1, 1);


        let mut s2 = S2::new(None);
        assert_eq!(s2.v2, 0);
        s2.add(1);
        assert_eq!(s2.v2, 1);
    }
    
}