use std::rc::Rc;

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

// This function returns A or B.
// Because the return type is not fixed, we have to return some type of reference to either object.
// Here we use Box.
fn constructor_1(s: &str) -> Box<dyn Test> { // dyn should be used
    let test: Box<dyn Test> = match s {
        "a" => Box::new(A{ value: 1 }),
        "b" => Box::new(B{ item: A{ value: 1 } }),
        _   => panic!("cannot"),
    };

    test
}

// And... Here we use Rc.
fn constructor_2(s: &str) -> Rc<dyn Test> { // dyn should be used
    let test: Rc<dyn Test> = match s {
        "a" => Rc::new(A{ value: 1 }),
        "b" => Rc::new(B{ item: A{ value: 1 } }),
        _   => panic!("cannot"),
    };

    test
}

#[test]
fn example_1_1() {
    let something_a = constructor_1("a");
    let something_b = constructor_1("b");
    something_a.test(3);
    something_b.test(3);
}

#[test]
fn example_1_2() {
    let something_a = constructor_2("a");
    let something_b = constructor_2("b");
    something_a.test(3);
    something_b.test(3);
}
