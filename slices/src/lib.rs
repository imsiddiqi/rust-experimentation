/*

// Input  : immutable reference (borrow) to a `String` object
// Output : index
pub fn first_word_1(s: &String) -> usize {
    // Here, `item` is an immutable reference.
    // If `s` was mutable, we could have used `iter_mut`.
    // In which case, type of item would have been `&mut u8`.
    for (index, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    return s.len();
}

pub fn first_word_2(s: &String) -> &str {
    for (index, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    // Why are both cases valid?
    if true {
        // `s` has type `&String` which does NOT the return type?
        // My guess would be some trait implementation is responible for performing the conversion?
        // Or perhaps, there is some built-in magic (special case rules) going on?
        return s;
    } else {
        // `&s[..]` has type `&str` which matches the return type.
        // My guess would be some method is responsible for implementing the slice behaviour for `String`.
        // This method returns `&str` so the compiler is happy.
        // Or perhaps, there is some built-in magic (special case rules) going on?
        return &s[..];
    }
}

pub fn first_word_3(s: &str) -> &str {
    for (index, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    // Why are both cases valid?
    if true {
        // `s` has type `&str` which matches the return type.
        return s;
    } else {
        // `&s[..]` has type `&str` which matches the return type.
        return &s[..];
    }
}

pub fn slice_main() {
    {
        let primitive_string   : &str                = "Izaan";
        let object_string      : String      = from("Siddiqi");

        // The following do not work.
        // I don't understand why (formally) but I understand why (intuitively).
        // primitive_string[0..1];
        // object_string[0..1];

        let _primitive_slice   : &str                = &primitive_string[0..1];
        let _object_slice      : &str                = &object_string[0..1];

        // I believe the odd syntax (`<usize>::new()`) is to prevent bugs.
        let mut test_vector    : Vec<usize>     = <usize>::new();
        test_vector.push(0);
        test_vector.push(1);
        test_vector.push(2);
        test_vector.push(3);

        // Why do I need to use `&[T]` over here?
        // Compared to just using `&str` (as opposed to `&[str]`).
        // If I had to guess:
        // * str is inherently capable of pointing to a sequence
        // * for other types, we need to distinguish be between scalars and arrays
        // * `&[&str]` (NOT `&[str]`) is possible if I had a `Vec<&str>?`
        let _test_slice       : &[usize]             = &test_vector[0..1];

        let mut test_vector_2 : Vec<&str>       = <&str>::new();
        test_vector_2.push("Hello");
        test_vector_2.push("World");

        let _test_slice_2     : &[&str]              = &test_vector_2[0..1];

        // So, does `str` on its own never exist? Documentation suggests that this is the case:
        // * https://doc.rust-lang.org/std/primitive.str.html
        // * https://doc.rust-lang.org/std/str/index.html
    }

    {
        let s : String = from("Hello world!");
        println!("{}", first_word_1(&s));
        println!("{}", first_word_2(&s));

        println!("{}", first_word_3(&s));
        println!("{}", first_word_3(&s[..]));
        println!("{}", first_word_3("Hello"));
    }
}


*/
