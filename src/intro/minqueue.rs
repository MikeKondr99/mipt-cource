#![forbid(unsafe_code)]

#[derive(Default)]
pub struct MinStack<T> {
    stack: Vec<(T, T)>, // .0 value .1 min
}

impl<T: Clone + Ord> MinStack<T> {
    pub fn new() -> Self {
        MinStack { stack: vec![] }
    }

    pub fn push(&mut self, val: T) {
        if self.is_empty() {
            self.stack.push((val.clone(), val));
        } else {
            let min = self.min().unwrap().to_owned();
            self.stack.push((val.clone(), val.min(min)));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop().map(|x| x.0)
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last().map(|x| &x.0)
    }

    pub fn min(&self) -> Option<&T> {
        self.stack.last().map(|x| &x.1)
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

#[derive(Default)]
pub struct MinQueue<T> {
    old: MinStack<T>,
    new: MinStack<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        MinQueue {
            old: MinStack::new(),
            new: MinStack::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.new.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.old.is_empty() {
            self.new_to_old();
            self.old.pop()
        } else {
            self.old.pop()
        }
    }

    fn new_to_old(&mut self) {
        while !self.new.is_empty() {
            self.old.push(self.new.pop().unwrap());
        }
    }

    pub fn front(&mut self) -> Option<&T> {
        if self.old.is_empty() {
            self.new_to_old();
            self.old.peek()
        } else {
            self.old.peek()
        }
    }

    pub fn min(&self) -> Option<&T> {
        let m1 = self.new.min();
        let m2 = self.old.min();
        match (m1, m2) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(x)) => Some(x),
            (Some(x), Some(y)) => Some(x.min(y)),
        }
    }

    pub fn len(&self) -> usize {
        self.new.len() + self.old.len()
    }

    pub fn is_empty(&self) -> bool {
        self.new.is_empty() && self.old.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MinQueue;
    use ntest::timeout;
    use rand::Rng;
    use std::collections::VecDeque;

    struct NaiveMinQueue<T> {
        data: VecDeque<T>,
    }

    impl<T: Clone + Ord> NaiveMinQueue<T> {
        pub fn new() -> Self {
            Self {
                data: VecDeque::new(),
            }
        }

        pub fn push(&mut self, val: T) {
            self.data.push_back(val);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.data.pop_front()
        }

        pub fn front(&self) -> Option<&T> {
            self.data.front()
        }

        pub fn min(&self) -> Option<&T> {
            self.data.iter().min()
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    #[test]
    fn it_works() {
        let mut queue = MinQueue::new();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.front(), None);
        assert_eq!(queue.min(), None);

        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.front(), None);
        assert_eq!(queue.min(), None);

        queue.push(2);
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &2);
        assert_eq!(queue.min().unwrap(), &2);

        queue.push(3);
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &2);
        assert_eq!(queue.min().unwrap(), &2);

        queue.push(4);
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &2);
        assert_eq!(queue.min().unwrap(), &2);

        queue.push(1);
        assert_eq!(queue.len(), 4);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &2);
        assert_eq!(queue.min().unwrap(), &1);

        queue.push(5);
        assert_eq!(queue.len(), 5);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &2);
        assert_eq!(queue.min().unwrap(), &1);

        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.len(), 4);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &3);
        assert_eq!(queue.min().unwrap(), &1);

        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &4);
        assert_eq!(queue.min().unwrap(), &1);

        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &1);
        assert_eq!(queue.min().unwrap(), &1);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
        assert_eq!(queue.front().unwrap(), &5);
        assert_eq!(queue.min().unwrap(), &5);

        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.front(), None);
        assert_eq!(queue.min(), None);

        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.front(), None);
        assert_eq!(queue.min(), None);
    }

    #[test]
    fn compare_with_naive() {
        let mut queue = MinQueue::new();
        let mut naive = NaiveMinQueue::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            if rng.gen_range(0..3) == 0 {
                assert_eq!(queue.pop(), naive.pop());
            } else {
                let value = rng.gen::<u64>();
                queue.push(value);
                naive.push(value);
            }
            assert_eq!(queue.len(), naive.len());
            assert_eq!(queue.is_empty(), naive.is_empty());
            assert_eq!(queue.front(), naive.front());
            assert_eq!(queue.min(), naive.min());
        }
    }

    #[test]
    #[timeout(2000)]
    fn stress() {
        let mut queue = MinQueue::new();
        let mut rng = rand::thread_rng();
        for _ in 0..300000 {
            match rng.gen_range(0..4) {
                0 => {
                    queue.pop();
                }
                1 => {
                    queue.min();
                }
                _ => {
                    queue.push(rng.gen::<u64>());
                }
            }
        }
    }
}
