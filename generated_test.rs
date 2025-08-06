use stdpython::*;

fn __module_init__() {
    print("Current directory:", &os::getcwd().unwrap_or_else(|_| "unknown".to_string()));
    print("Python executable:", &sys::executable);
}

fn main() {
    __module_init__();
}