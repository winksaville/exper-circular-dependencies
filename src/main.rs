struct S1 {
    v1: i32
}

impl S1 {
    fn new() -> Self {
        Self {
            v1: 0,
        }
    }

    fn add(&mut self, val: i32) {
        self.v1 += val;
    }
}

struct S2 {
    v2: i32
}

impl S2 {
    fn new() -> Self {
        Self {
            v2: 0,
        }
    }

    fn add(&mut self, val: i32) {
        self.v2 += val;
    }
}

fn main() {
    let mut s1 = S1::new();
    s1.add(-1);
    let mut s2 = S2::new();
    s2.add(1);

    println!("s1.v1={} s2.v2={}", s1.v1, s2.v2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut s1 = S1::new();
        assert_eq!(s1.v1, 0);
        s1.add(1);
        assert_eq!(s1.v1, 1);


        let mut s2 = S2::new();
        assert_eq!(s2.v2, 0);
        s2.add(1);
        assert_eq!(s2.v2, 1);
    }
    
}