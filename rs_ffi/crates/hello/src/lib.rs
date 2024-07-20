#[no_mangle]
pub extern "C" fn add(a: usize, b: usize) -> usize {
    a + b
}
