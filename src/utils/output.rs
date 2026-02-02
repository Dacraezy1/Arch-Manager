use colored::Colorize;

pub fn ok(msg: &str) {
    println!("{}", msg.green());
}

pub fn info(msg: &str) {
    println!("{}", msg.blue());
}

pub fn warn(msg: &str) {
    println!("{}", msg.yellow());
}
