use std::mem;


fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // ‘len’ 返回数组大小
    println!("array size: {}", xs.len());

    // 数组实在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以被自动借用成为 slice
    println!("borrow the whole array as slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界下表会引发致命错误(panic)
    // println!("{}", xs[5]);
}
