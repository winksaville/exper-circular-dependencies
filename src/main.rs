use std::{rc::Rc, cell::RefCell};

struct S1 {
    s2: Option<Rc<RefCell<S2>>>,
    v1: i32
}

impl<'a> S1 {
    fn new(s2: Option<Rc<RefCell<S2>>>) -> Self {
        Self {
            s2,
            v1: 0,
        }
    }

    fn add(&mut self, val: i32) {
        self.v1 += val;
    }

    fn v1(&self) -> i32 {
        self.v1
    }

    fn s2(&self) -> Rc<RefCell<S2>> {
        match &self.s2 {
            Some(s2) => Rc::clone(&s2),
            None => panic!("s2 not Initialized"),
        }
    }

    fn v2(&self) -> i32 {
        self.s2().borrow().v2()
    }
}

struct S2 {
    s1: Option<Rc<RefCell<S1>>>,
    v2: i32
}

impl S2 {
    fn new(s1: Option<Rc<RefCell<S1>>>) -> Self {
        Self {
            s1,
            v2: 0,
        }
    }

    fn add(&mut self, val: i32) {
        self.v2 += val;
    }

    fn v2(&self) -> i32 {
        self.v2
    }

    fn s1(&self) -> Rc<RefCell<S1>> {
        match &self.s1 {
            Some(s1) => Rc::clone(&s1),
            None => panic!("s1 not Initialized"),
        }
    }

    fn v1(&self) -> i32 {
        self.s1().borrow().v1()
    }
}


#[allow(unused)]
fn main() {
    let s1 = Rc::new(RefCell::new(S1::new(None)));
    let s2 = Rc::new(RefCell::new(S2::new(None)));
    println!("s1.strong_count={} s2.strong_count={}", Rc::strong_count(&s1), Rc::strong_count(&s2));
    println!("s1.weak_count={} s2.weak_count={}", Rc::weak_count(&s1), Rc::weak_count(&s2));

    s1.borrow_mut().s2 = Some(Rc::clone(&s2));
    s2.borrow_mut().s1 = Some(Rc::clone(&s1));
    println!("s1.strong_count={} s2.strong_count={}", Rc::strong_count(&s1), Rc::strong_count(&s2));
    println!("s1.weak_count={} s2.weak_count={}", Rc::weak_count(&s1), Rc::weak_count(&s2));

    {
        // Must be a separate lifetime so we don't get a runtime error:
        //   thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:93:8
        let s1_borrowed = s1.borrow();
        let s2_borrowed = s2.borrow();

        let v1 = s2_borrowed.v1();
        let v2 = s1_borrowed.v2();
        println!("v1={v1} v2={v2}");
    }

    s1.borrow_mut().add(1);
    s2.borrow_mut().add(-1);
    println!("s1.v1={} s2.v2={}", s1.borrow().v1(), s2.borrow().v2());

    let v1 = s2.borrow().v1();
    let v2 = s1.borrow().v2();
    println!("v1={v1} v2={v2}");

    s1.borrow_mut().add(1);
    s2.borrow_mut().add(-1);
    println!("s1.v1={} s2.v2={}", s1.borrow().v1(), s2.borrow().v2());

    let v1 = s2.borrow().v1();
    let v2 = s1.borrow().v2();
    println!("v1={v1} v2={v2}");

    println!("s1.strong_count={} s2.strong_count={}", Rc::strong_count(&s1), Rc::strong_count(&s2));
    println!("s1.weak_count={} s2.weak_count={}", Rc::weak_count(&s1), Rc::weak_count(&s2));
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
