#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Lvwei");
    let age = 27;
    let lvwei = Person { name, age };

    println!("{:?}", lvwei);

    // 实例化结构体
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);    

    // 使用结构体更新语法创建新实例
    let bottom_right = Point {x: 5.2, ..point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用let 绑定来解构Point
    let Point {x: left_edge, y: top_edge} = point;
    let _rectangle = Rectangle {
        top_left : Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    // 实例化单元结构体
    let _unit = Unit;

    // 实例化元组结构体
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 结构元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
