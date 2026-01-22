/**
 * 作用域和遮蔽
 */

fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5f32;

        println!("inner long: {}", long_lived_binding);
    }

    // error: cannot find value `short_lived_binding` in this scope
    // println!("outer short: {}", short_lived_binding); 

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样 “遮蔽” 了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}