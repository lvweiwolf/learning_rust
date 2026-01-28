
//! # Rust 类型别名和类型转换示例
//!
//! 本程序演示了 Rust 中类型别名的定义和使用，以及类型转换的语法。
//! 包括：
//! - 自定义类型别名的定义
//! - as 关键字进行类型转换
//! - 类型别名的使用场景

// 定义一个类型别名 NanoSecond，等价于 u64
// 用于提高代码的可读性和语义化
type NanoSecond = u64;

// 定义一个类型别名 Inch，等价于 u64  
// 用于表示英寸长度，增强代码的语义表达
type Inch = u64;

// 通过这个属性允许非驼峰命名的类型名称
// u64_t 是 C 语言风格的命名，在 Rust 中通常不推荐
#[allow(non_camel_case_types)]
type u64_t = u64;

/// 主函数：演示类型别名和类型转换
fn main() {
    // 使用类型别名 NanoSecond 声明变量 nanoseconds
    // 将 5 作为 u64_t 类型进行转换后赋值给 nanoseconds
    // 由于 u64_t 本身就是 u64，这个转换是安全的
    let nanoseconds: NanoSecond = 5 as u64_t;
    
    // 使用类型别名 Inch 声明变量 inches  
    // 将 2 作为 u64_t 类型进行转换后赋值给 inches
    let inches: Inch = 2 as u64_t;

    // 打印类型转换和计算结果
    // nanoseconds 和 inches 都是 u64 类型，可以安全相加
    // 输出结果：5 nanoseconds + 2 inches = 7 unit?
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}