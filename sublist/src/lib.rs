#[derive(Debug, PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist
}

pub fn sublist<T: Eq>(list1: &[T], list2: &[T]) -> Comparison {
    if list1 == list2 {
        Comparison::Equal
    } else if sub(list1, list2) {
        Comparison::Sublist
    } else if sub(list2, list1) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}


fn sub<T: Eq>(list1: &[T], list2: &[T]) -> bool {
    if list1.len() > list2.len() { return false }

    (0..list2.len()).any(|start| {
        let list2_sub = &list2[start..];

        if list2_sub.len() < list1.len() { return false }

        list1.iter().enumerate().all(|el| *el.1 == list2_sub[el.0])
    })
}
