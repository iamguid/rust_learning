struct LinkedListItem<T> {
    prev: Option<Box<LinkedListItem<T>>>,
    next: Option<Box<LinkedListItem<T>>>,
    value: T,
}

struct LinkedList<T> {
    begin: Option<LinkedListItem<T>>,
    end: Option<LinkedListItem<T>>,
}

impl<T> Iterator for LinkedList<T> {
    type Item = Box<LinkedListItem<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(begin_item) = &self.begin {
            if let Some(next_item) = &begin_item.next {
                Some(*next_item)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            begin: None,
            end: None
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        let item = self.find_by_index(index);

        if let Some(mut item) = item {
            item.value = value
        }
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        let item = self.find_by_index(index);

        if let Some(item) = item {
            Some(item.value)
        } else {
            None
        }
    }

    pub fn get_last(&mut self) -> Option<&T> {
        if let Some(value) = &self.end {
            Some(&value.value)
        } else {
            None
        }
    }

    pub fn get_first(&mut self) -> Option<&T> {
        if let Some(value) = &self.begin {
            Some(&value.value)
        } else {
            None
        }
    }

    pub fn delete(&self, index: usize) {
    }

    pub fn insert_after(&self, value: T, after: usize) {
    }

    pub fn insert_before(&self, value: T, after: usize) {
    }

    pub fn insert_last(&self, value: T, after: usize) {
    }

    pub fn insert_first(&self, value: T, after: usize) {
    }

    pub fn len(&mut self) -> usize {
        let mut i = 0;

        for _ in self {
            i += 1;
        }

        i
    }

    fn find_by_index(&mut self, index: usize) -> Option<Box<LinkedListItem<T>>> {
        let mut i = 0;

        for item in self {
            if i == index {
                return Some(item)
            }

            i += 1;
        }

        None
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_should_be_correct() {
        let linked_list: LinkedList<i32> = LinkedList::new();
        assert!(linked_list.begin.is_none());
        assert!(linked_list.end.is_none());
    }

    #[test]
    fn first_push_should_be_add_element_to_list() {
        let linked_list: LinkedList<i32> = LinkedList::new();
    }
}
