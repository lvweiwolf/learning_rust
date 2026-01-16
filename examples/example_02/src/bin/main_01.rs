fn main() {
    let logical: bool = true;

    // 强制声明类型
    let a_float: f64 = 1.0; // 常规声明
    let an_integer = 5i32; // 类型后缀声明

    // 推断类型
    let default_float = 3.0;    // f64
    let default_integer = 7;    // i32

    // 根据上下文推断类型
    let mut inferred_type = 12;
    inferred_type = 4294967296i64; // 此处推断未i64

    let mut mutable = 12;  // i32
    mutable = 21;

    // 报错！变量的类型不能改变
    // mutable = true;

    // 但可以被覆盖
    let mutable = true;
    

    println!("Hello, world!");
}
