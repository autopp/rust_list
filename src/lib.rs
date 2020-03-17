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

#[cfg(test)]
mod tests {
    use super::*;
    use super::List::*;

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
}
