#![no_std]
#![deny(unsafe_code)]

use ostd::prelude::*;

use owo_colors::OwoColorize;

struct CustomLog {}
impl log::Log for CustomLog {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()){
            return;
        }
        match record.level() {
            log::Level::Error => { print!("{}:", "ERROR".red()); }
            log::Level::Warn => { print!("{}:", "WARNING".yellow()); }
            log::Level::Info => { print!("{}:", "INFO".blue()); }
            log::Level::Debug => { print!("{}:", "DEBUG".green()); }
            log::Level::Trace => { print!("{}:", "TRACE".magenta()); }
        }
        println!("{}->{}", record.target(), record.args());
    }
    fn flush(&self) {
    }
}
static CUSTOM_LOGGER:CustomLog=CustomLog{};

#[ostd::main]
fn kernel_main() {
    println!("--------------------");
    println!("--------------------");
    println!("《大风歌》  刘邦");
    println!("大风起兮云飞扬");
    println!("威加海内兮归故乡");
    println!("安得猛士兮守四方");
    println!("--------------------");
    println!("--------------------");

    ostd::logger::inject_logger(&CUSTOM_LOGGER);
}

#[cfg(ktest)]
mod unit_tests{
    use super::*;

    #[ktest]
    fn unit_test(){
        assert_eq!(6*14,84);
    }
}
