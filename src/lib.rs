pub mod first;
pub mod second;

#[cfg(test)]
mod tests {
    use super::first::LinkedList;
    #[test]
    fn it_works() {
        let mut empty_list = LinkedList::empty();
        empty_list.push(123);
    }
}
