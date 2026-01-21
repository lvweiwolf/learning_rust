#![allow(dead_code)]

enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Soldier
}

fn main() {
    // 显示use
    use Status::{Poor, Rich};
    // 自动use 所有内容
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}