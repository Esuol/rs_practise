mod closure;
mod simple_code;

fn main() {
    simple_code::simple_thread();
    closure::closure_test();
    closure::closure_object_test()
}
