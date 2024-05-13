mod linked_list {
    #[derive(Debug)]
    pub struct MyLinkedList<T: PartialOrd>(Option<(T, Box<MyLinkedList<T>>)>);

    impl<T: PartialOrd> MyLinkedList<T> {
        pub fn new() -> Self {
            MyLinkedList(None)
        }

        pub fn push_front(&mut self, item: T) {
            let pointer_to_next = self.0.take();
            self.0 = Some((item, Box::new(MyLinkedList(pointer_to_next))));
        }

        pub fn push_back(&mut self, item: T) {
            match self.0 {
                Some((_, ref mut pointer_to_next)) => pointer_to_next.push_back(item),
                None => self.push_front(item),
            }
        }

        pub fn insert_sorted(&mut self, item: T) {
            match self.0 {
                Some((ref current_value, ref mut pointer_to_next)) => {
                    if item < *current_value {
                        self.push_front(item);
                    } else {
                        pointer_to_next.insert_sorted(item);
                    }
                }
                None => self.push_front(item),
            }
        }
    }
}
 
mod doubly_linked_list{
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node<T>{
        data: T,
        prev: Option<Weak<RefCell<Node<T>>>>,
        next: Option<Rc<RefCell<Node<T>>>>
    }

    #[derive(Debug)]
    pub struct MyDoublyLinkedList<T>{
        first: Option<Rc<RefCell<Node<T>>>>,
        last: Option<Weak<RefCell<Node<T>>>>
    }

    impl<T> MyDoublyLinkedList<T>{
        pub fn new() -> Self{
            Self{
                first: None,
                last: None
            }
        }

        pub fn push_front(&mut self, item: T){
            match self.first.take() {
                Some(node) => {
                    let new_first = Rc::new(RefCell::new(Node{
                        data: item,
                        prev: None,
                        next: Some(node.clone()),
                    }));

                    let mut inner_node = node.borrow_mut();
                    inner_node.prev = Some(Rc::downgrade(&new_first));
                    self.first = Some(new_first);
                }
                None => {
                    let only_elem = Rc::new(RefCell::new(Node{
                        data: item,
                        prev: None,
                        next: None,
                    }));

                    self.last = Some(Rc::downgrade(&only_elem));
                    self.first = Some(only_elem);
                }
            }
        }

        pub fn push_back(&mut self, item:T){
            match self.last.take() {
                Some(node) => {
                    let new_last = Rc::new(RefCell::new(Node{
                        data: item,
                        prev: Some(node.clone()),
                        next: None,
                    }));

                    let upgraded_node = node.upgrade().unwrap();
                    let mut inner_node = upgraded_node.borrow_mut();
                    inner_node.next = Some(new_last.clone());
                    self.last = Some(Rc::downgrade(&new_last));
                }
                None => {
                    let only_elem = Rc::new(RefCell::new(Node{
                        data: item,
                        prev: None,
                        next: None,
                    }));

                    self.last = Some(Rc::downgrade(&only_elem));
                    self.first = Some(only_elem);
                }
            }
        }

        pub fn pop_front(&mut self) -> bool {
            match self.first.take() {
                Some(node)=> {

                    match node.clone().borrow_mut().next.clone() {
                        //at least two elems in the list
                        Some(second_node) => {
                            let mut second_node_inner = second_node.borrow_mut();
                            second_node_inner.prev = None;

                            self.first = Some(second_node.clone());
                        }
                        None => {
                            //only one elem in the list
                            self.first = None;
                            self.last = None;
                        }
                    }

                    true
                },
                //list is empty, do nothing
                None => false
            }
        }

        pub fn pop_back(&mut self) -> bool {
            match self.last.take() {
                Some(node) => {
                    let upgraded_node = node.upgrade().unwrap();
                    match  { upgraded_node.borrow_mut().prev.clone() }{
                        Some(penultimate) => {
                            let upgraded_penultimate = penultimate.upgrade().unwrap();
                            let mut borrowed = upgraded_penultimate.borrow_mut();
                            borrowed.next = None;

                            self.last = Some(penultimate.clone());
                        }
                        None => {
                            self.last= None;
                            self.first = None;
                        }
                    }

                    true
                }
                None => false
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::linked_lists::doubly_linked_list::MyDoublyLinkedList;
    use crate::linked_lists::linked_list::MyLinkedList;
    use super::*;

    #[test]
    fn it_works() {
        let mut my_list = MyLinkedList::new();

        my_list.push_front(5);
        my_list.push_front(1);
        my_list.push_back(7);
        my_list.push_back(9);

        println!("{:?}", my_list);
    }

    #[test]
    fn it_works_sorted() {
        let mut my_list = MyLinkedList::new();

        my_list.insert_sorted(5);
        my_list.insert_sorted(1);
        my_list.insert_sorted(9);
        my_list.insert_sorted(7);

        println!("{:?}", my_list);
    }

    #[test]
    fn check_doubly_linkjed_list() {
        let mut my_list = MyDoublyLinkedList::new();

        my_list.push_front(5);
        my_list.push_front(1);
        my_list.push_back(7);
        my_list.push_back(9);

        println!("{:?}", my_list);
        my_list.pop_front();
        my_list.pop_back();
        println!("{:?}", my_list);

    }
}
