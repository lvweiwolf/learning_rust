use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let vec = &self.0;
        
        write!(f, "[")?; // ? 用于解包错误，并返回错误

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", v)?;
        }
        
        return write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
