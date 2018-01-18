#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    // Delete this field
    // _dummy is needed to avoid unused parameter error during compilation
    head: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}


impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node { data: self.data.clone(), next: self.next.clone() }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        match self.head  {
            None =>  0,
            Some(ref head) => {
                let mut count = 1;
                let mut current = head;

                while let Some(ref node) = current.next {
                    current = node;
                    count += 1;
                }

                count
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let next = match self.head {
            None => None,
            Some(ref node) => Some(node.clone())
        };

        self.head = Some(Box::new(Node{data: element, next: next}));
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped = match self.head {
            None => None,
            Some(ref h) => Some(h.clone())
        };

        match &popped {
            &None => (),
            &Some(ref h) => {
                self.head = h.next.clone()
            }
        };

         match popped {
             None => None,
             Some(node) => Some(node.data)
         }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref node) => Some(&node.data)
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        match self.head  {
            None => SimpleLinkedList::new(),
            Some(ref head) => {
                let mut current = head;
                let mut vec = Vec::new();

                while let Some(ref node) = current.next {
                    vec.push(current.data.clone());
                    current = node;
                }

                vec.push(current.data.clone());

                SimpleLinkedList::from(vec.as_ref())
            }
        }
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in item {
            list.push(i.clone());
        }

        list
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        match self.head  {
            None => Vec::new(),
            Some(ref head) => {
                let mut current = head;
                let mut vec = Vec::new();

                while let Some(ref node) = current.next {
                    vec.push(current.data.clone());
                    current = node;
                }

                vec.push(current.data.clone());

                vec.reverse();
                vec
            }
        }
    }
}
