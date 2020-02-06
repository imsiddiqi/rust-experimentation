#[cfg(test)]
mod tests {
    #[test]
    fn boxed() {

        // Experimenting with `std::boxed::Box`.

        #[derive(Clone, Default)]
        struct Test {
            value: usize,
        }

        let mut test: Test = Default::default();
        test.value = 1;
        let mut boxed_1 = Box::new(test);
        let boxed_2 = boxed_1.clone(); // This clones everything on heap.

        assert_eq!(boxed_1.value, 1);
        assert_eq!(boxed_2.value, 1);

        boxed_1.value = 2;

        // The two boxes are NOT pointing to the same object!
        assert_eq!(boxed_1.value, 2);
        assert_eq!(boxed_2.value, 1);

        // Objects can be returned from the heap:
        let test_1 = *boxed_1; // Cannot repeat.
        let test_2 = *boxed_2; // Cannot repeat.

        assert_eq!(test_1.value, 2);
        assert_eq!(test_2.value, 1);
    }

    #[test]
    fn boxed_recursive_structure() {

        // `std::boxed::Box` helps us implement recursive data structures.
        // A data structure (type T) is recursive if it contains at least one field that has type T.
        // This particular implementation isn't good (just for demonstration).
        // This is useful: https://rust-unofficial.github.io/too-many-lists/

        #[derive(Debug)]
        enum List<T> where T: Clone {
            List(T, Box<List<T>>),
            Empty,
        }

        impl<T> List<T> where T: Clone {
            fn new() -> Self {
                List::Empty
            }

            fn append(&mut self, item: T) -> &mut Self {
                match self {
                    List::Empty => {
                        *self = List::List(item.clone(), Box::new(List::new()));
                    },
                    List::List(_, next) => {
                        next.append(item);
                    }
                }

                return self;
            }
        }

        let mut list: List<usize> = List::new();
        assert_eq!(format!("{:?}", list), "Empty");
        list.append(0).append(1).append(2);
        assert_eq!(format!("{:?}", list), "List(0, List(1, List(2, Empty)))");
    }

    #[test]
    fn pinned() {
        /*

        // Let's start by looking at an example that doesn't compile:

        struct Value {
            value: usize,
        }

        struct Wrapper<'a> {
            value: Value,
            reference: &'a Value,
        }

        let value = Value { value: 0 };
        let test = Wrapper {
            value: value,
            reference: &value, // E0382
        };

        */
    }
}
