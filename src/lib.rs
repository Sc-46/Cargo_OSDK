#![no_std]
#![deny(unsafe_code)]

use ostd::prelude::*;

#[ostd::main]
fn kernel_main() {
    println!("Hello world from guest kernel!");
}

#[cfg(test)]
mod unit_tests{
    use super::*;

    #[ktest]
    fn simple_test(){
        assert_eq!(6*14,84);
    }
}
