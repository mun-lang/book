use mun_runtime::{invoke_fn, RetryResultExt, RuntimeBuilder};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut builder = RuntimeBuilder::new("main.munlib");
    let runtime = builder
        .spawn()
        .expect("Failed to spawn Runtime");

    let runtime = Rc::new(RefCell::new(runtime));
    let result: bool = invoke_fn!(runtime, "random_bool").unwrap();
    println!("random bool: {}", result);
}