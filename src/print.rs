pub fn print(str: &str) {
    for char in str.bytes() {
        crate::arch::misc::write_uart(char);
    }
}
