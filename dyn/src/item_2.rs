// We have an example trait:
trait Test {
    fn test(&self, count: usize);
}

// Two stucts implement this example trait:
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

////////////////////////////////////////////////////////////////////////////////

// When should we use dyn?
// When can we not use dyn?
// I think dyn is added before traits objects...
impl A { // dyn cannot be used

}

impl dyn Test { // dyn should be used
    // What does this do?
}

fn _construct_1() -> Box<A> { // dyn cannot be used
    Box::new(A{ value: 1 })
}

fn _construct_2() -> Box<dyn Test + Send + Sync> { // dyn should be used
    Box::new(A{ value: 1 })
}

fn _construct_3() -> impl Test + Send + Sync { // Can we use dyn?
    A{ value: 1 }
}

////////////////////////////////////////////////////////////////////////////////

struct Cat {
    a: Option<A>,
}

impl Cat {
    fn _meow1(&self) -> &dyn Test {
        self.a.as_ref().unwrap()
    }

    // Returning `dyn Test` doesn't make sense because the size cannot be known at compile time.

    fn _meow2(&self) -> &impl Test {
        self.a.as_ref().unwrap()
    }

    fn _meow3(&mut self) -> impl Test {
        self.a.take().unwrap()
    }
}

#[test]
fn example_2_1() {
    use std::sync::{Arc, Mutex};
    let mut cat = Cat{a: Some(A{ value: 0 })};

    // Doesn't work:
    // the trait `Test` is not implemented for `&dyn Test`
    // let _1: Arc<Mutex<dyn Test>> = Arc::new(Mutex::new(cat._meow1()));

    // Doesn't work:
    // the trait `Test` is not implemented for `&impl Test`
    // let _2: Arc<Mutex<dyn Test>> = Arc::new(Mutex::new(cat._meow2()));

    // Works okay:
    let _3: Arc<Mutex<dyn Test>> = Arc::new(Mutex::new(cat._meow3()));
}
