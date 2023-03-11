# python-ast-rs
A Rust library for accessing a Python AST using the Python 3 ast library. Because it relies on Python itself to parse Python, it should be very closer to the reference implementation. However, it is possible that changes in the underlying Python language could prevent it from working at all if there are dramatic changes to the syntax tree.

This project is at a very early state, and should be considered completely unstable.

## Useage

Reading a Python file into an ast works like this:

```Rust
use python_ast::parse;

fn read_python_file(input: std::path::Path) {
    let py = read_to_string(input).unwrap();
    let ast = parse(&py, "__main__").unwrap();

    println!("{:?}", ast);
}

```


