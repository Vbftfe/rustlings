// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.
#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        //  创建一个空的List
        List::Nil
    }

    fn append(self, v: i32) -> List {
        // 在列表的头部添加一个元素
        match self {
            List::Cons(_, _) => List::Cons(v, Box::new(self)),
            List::Nil => List::Cons(v, Box::new(List::Nil)),
        }
    }

    fn get_next(&self) -> Option<&List> {
        match self {
            List::Cons(v, node) => Some(node.as_ref()),
            List::Nil => None,
        }
    }
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil))
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    #[test]
    fn test_append() {
        let mut list = List::new();
        list = list.append(3);
        list = list.append(4);
        assert_eq!(
            list,
            List::Cons(4, Box::new(List::Cons(3, Box::new(List::Nil))))
        );
    }
}
