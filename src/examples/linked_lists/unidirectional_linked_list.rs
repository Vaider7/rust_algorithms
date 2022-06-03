use std::collections::LinkedList;


pub fn foo() {
    let linked_list = LinkedList::from([1, 2, 3]);

    let mut shit = [1, 3, 1, 2];


    shit.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::foo;

    #[test]
    fn it_should_foo() {
        foo()
    }

}