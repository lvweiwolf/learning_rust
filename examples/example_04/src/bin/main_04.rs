
/**
 * 变量先声明
 */

fn main() {
    // 声明变量
    let a_binding;

    {
        let x = 2;
        
        // 初始化一个绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 错误：使用了未初始化的变量
    // println!("another binding: {}", another_binding); 

    another_binding = 1;

    println!("another bindingL: {}", another_binding);
}