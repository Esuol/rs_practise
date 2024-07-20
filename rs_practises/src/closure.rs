use std::thread;

pub fn closure_test() {
    let a = 1;
    let test_fn = |x| {
        println!("x: {}", x);
    };
    test_fn(a);
}

#[derive(Debug)]
struct User {
    name: String,
}

pub fn closure_object_test() {
    let name = User {
        name: String::from("rust"),
    };

    let test_fn = || {
        println!("{:?}1", name);
    };
    test_fn();
    println!("{:?}2", name);

    let handle = thread::spawn(move || {
        println!("{:?}3", name);
    });

    // println!("{:?}4", name);

    handle.join().unwrap();

    // let a = move || {
    //     println!("{:?}", name);
    // };
    // test_a(&a);
    // test_a(&a);
}

// fn test_a<F: FnOnce() -> () + Copy>(fns: F) {
//     fns();
//     fns();
// }
