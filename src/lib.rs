#![no_std]
#![deny(unsafe_code)]

use ostd::prelude::*;

#[ostd::main]
fn kernel_main() {
    println!();
    println!();
    println!("Hello world from guest kernel!");
    println!("《大风歌》  刘邦");
    println!("大风起兮云飞扬");
    println!("威加海内兮归故乡");
    println!("安得猛士兮守四方");
    println!();
    println!();
}

#[cfg(test)]
mod unit_tests{
    use super::*;

}
