// 元组充当函数参数或返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    return (pair.1, pair.0);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple fisrt value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组作为元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u16, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素的元组需要一个额外的逗号
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
 }
