use List::*;

enum List {
    // Cons: 元祖结构体，包含链表的一个元素和一个执行下一节点的指针
    Cons(u32, Box<List>),
    // Nil: 末节点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    fn new() -> List {
        return Nil;
    }

    fn prepend(self, elem: u32) -> List {
        return Cons(elem, Box::new(self));
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 self 进行匹配(match)，因为这个方法的行为取决于 self 的取值种类。
        // self 为 &List 类型， *self 为List类型，匹配一个具体的 T 类型要好过匹配引导 &T
        match *self {
            // 不能得到 tail 的所有权，因为 self 是借用的；
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
