mod closure;
mod mutex_arc;
mod simple_code;

fn main() {
    simple_code::simple_thread();
    closure::closure_test();
    closure::closure_object_test();
    mutex_arc::arc_test();
    mutex_arc::mutex_test();
}
