trait Test {
    fn test(&self, count: usize);
}

#[derive(Debug)]
struct A {
    value: usize,
}

impl Test for A {
    fn test(&self, count: usize) {
        for _ in 0..count {
            println!("{}", self.value);
        }
    }
}

#[derive(Debug)]
struct B {
    item: A,
}

impl Test for B {
    fn test(&self, count: usize) {
        let mut temp = 0;
        for _ in 0..count {
            temp += self.item.value;
        }

        println!("{}", temp);
    }
}

fn constructor(s: &str) -> Box<dyn Test> { // dyn should be used
    let test: Box<dyn Test> = match s {
        "a" => Box::new(A{ value: 1 }),
        "b" => Box::new(B{ item: A{ value: 1 } }),
        _   => panic!("cannot"),
    };

    test
}

fn a() -> Box<A> { // dyn cannot be used
    Box::new(A{ value: 1 })
}

impl dyn Test { // dyn should be used
    // What does this do?
}

impl A { // dyn cannot be used

}

fn constructor_a() -> impl Test + std::fmt::Debug {
    A{ value: 1 }
}

fn constructor_b() -> impl Test + std::fmt::Debug {
    B{ item: A{ value: 1 } }
}

#[test]
fn snippet_1() {
    let something_a = constructor("a");
    let something_b = constructor("b");
    something_a.test(3);
    something_b.test(3);
}

// Don't need to always use Box with dyn SomeTrait:

struct Cat {
    a: Option<A>,
}

impl Cat {
    fn meow1(&self) -> &dyn Test {
        self.a.as_ref().unwrap()
    }

    fn meow2(&self) -> &impl Test {
        self.a.as_ref().unwrap()
    }

    fn meow3(&mut self) -> impl Test {
        self.a.take().unwrap()
    }
}

#[test]
fn snippet_2() {
    use std::sync::{Arc, Mutex};
    let mut cat = Cat{a: Some(A{ value: 0 })};

    // Doesn't work: the trait `Test` is not implemented for `&dyn Test`
    // let hmm1: Arc<Mutex<dyn Test>> = Arc::new(Mutex::new(cat.meow1()));

    // Doesn't work: the trait `Test` is not implemented for `&impl Test`
    // let hmm2: Arc<Mutex<dyn Test>> = Arc::new(Mutex::new(cat.meow2()));

    let temp = cat.meow3();
    let hmm3: Arc<Mutex<dyn Test>> = Arc::new(Mutex::new(temp));
}
