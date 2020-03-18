#[derive(Debug,PartialEq)]
pub enum List {
    Cons(i64, Box<List>),
    Nil
}

pub fn head(l: &List) -> Option<i64> {
    match l {
        List::Cons(value, _) => Some(*value),
        List::Nil => None,
    }
}

pub fn tail(l: &List) -> Option<&List> {
    match l {
        List::Cons(_, rest) => Some(rest),
        List::Nil => None
    }
}

pub fn list(elems: &Vec<i64>) -> List {
    let mut ret = List::Nil;
    for x in elems.iter().rev() {
        ret = List::Cons(*x, Box::new(ret))
    }
    ret
}

pub fn length(l: &List) -> i64 {
    match l {
        List::Cons(_, rest) => length(rest) + 1,
        List::Nil => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::List::*;

    #[test]
    fn test_list() {
        assert_eq!(list(&vec![]), Nil);
        assert_eq!(list(&vec![1]), Cons(1, Box::new(Nil)));
        assert_eq!(list(&vec![1, 2]), Cons(1, Box::new(Cons(2, Box::new(Nil)))))
    }

    #[test]
    fn test_head() {
        assert_eq!(head(&Cons(1, Box::new(Nil))), Some(1));
        assert_eq!(head(&Nil), None)
    }

    #[test]
    fn test_tail() {
        assert_eq!(tail(&Cons(1, Box::new(Cons(2, Box::new(Nil))))), Some(&Cons(2, Box::new(Nil))));
        assert_eq!(tail(&Nil), None);
    }

    #[test]
    fn test_length() {
        assert_eq!(length(&list(&vec![])), 0);
        assert_eq!(length(&list(&vec![1, 2])), 2)
    }
}
