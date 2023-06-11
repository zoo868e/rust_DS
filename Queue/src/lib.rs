#[derive(Debug, PartialEq, Eq)]
struct Queue<T> {
    vec: Vec<T>,
    capacity: Option<usize>,
}

impl<T: Clone> From<Vec<T>> for Queue<T> {
    fn from(vec: Vec<T>) -> Self {
        Queue {
            vec,
            capacity: None,
        }
    }
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            vec: Vec::new(),
            capacity: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Queue<T> {
        Queue {
            vec: Vec::with_capacity(capacity),
            capacity: Some(capacity),
        }
    }

    pub fn queue(&mut self, arg: T) -> Result<usize, ()> {
        match self.capacity {
            None => {
                self.vec.push(arg);
                Ok(self.vec.len())
            }
            Some(capacity) => {
                if self.vec.len() < capacity {
                    self.vec.push(arg);
                    Ok(self.vec.len())
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.vec.is_empty() {
            true => None,
            _ => {
                let item = self.vec.remove(0);
                Some(item)
            }
        }
    }

    pub fn force_queue(&mut self, arg: T) -> usize {
        match self.capacity {
            None => {
                self.vec.push(arg);
                self.vec.len()
            }
            Some(capacity) => {
                if self.vec.len() == capacity {
                    self.vec.remove(0);
                }
                self.vec.push(arg);
                self.vec.len()
            }
        }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn peek(&self) -> T {
        self.vec[0].clone()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    pub fn set_capacity<C: Into<Option<usize>>>(&mut self, capacity: C) -> Result<(), ()> {
        let capacity = capacity.into();
        if capacity == None {
            self.capacity = None;
            return Ok(());
        }
        if capacity == self.capacity {
            return Ok(());
        }
        let capacity = capacity.unwrap();
        if capacity < self.vec.len() {
            return Err(());
        }
        if let Some(old_capacity) = self.capacity {
            if capacity < old_capacity {
                self.vec.shrink_to_fit();
            }
        }
        let r = capacity - self.vec.len();
        self.vec.reserve_exact(r);
        self.capacity = Some(capacity);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue() {
        let target: Queue<i32> = Queue::new();
        let result = Queue {
            vec: Vec::new(),
            capacity: None,
        };
        assert_eq!(result, target);
    }

    #[test]
    fn new_queue_with_capacity() {
        let target: Queue<i32> = Queue::with_capacity(10);
        let result: Queue<i32> = Queue {
            vec: Vec::with_capacity(10),
            capacity: Some(10),
        };
        assert_eq!(result, target);
    }
    #[test]
    fn add_new_element() {
        let mut q = Queue::<i32>::new();
        q.queue(1);
        let result = Queue {
            vec: vec![1],
            capacity: None,
        };
        assert_eq!(result, q);
    }

    #[test]
    fn force_add_new_element() {
        let mut q = Queue::<i32>::with_capacity(10);
        for i in 0..10 {
            q.queue(i);
        }
        q.force_queue(10);
        let result: Queue<i32> = Queue {
            vec: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            capacity: Some(10),
        };
        assert_eq!(result, q);
    }

    #[test]
    fn remove_element_from_queue() {
        let mut q = Queue::<i32>::new();
        q.queue(1);
        let value = q.dequeue();
        let result = Queue::<i32>::new();
        assert_eq!(result, q);
        assert_eq!(Some(1), value);
    }
    #[test]
    fn failed_remove_element_from_queue() {
        let mut q = Queue::<i32>::new();
        let result = q.dequeue();
        let target = Option::<i32>::None;
        assert_eq!(result, target);
    }
    #[test]
    fn length_of_queue() {
        let mut q = Queue::<i32>::new();
        q.queue(1);
        q.queue(2);
        q.queue(3);
        q.queue(4);
        let result = 4;
        assert_eq!(result, q.len());
    }
    #[test]
    fn peek_queue() {
        let mut q = Queue::<i32>::new();
        q.queue(1);
        q.queue(2);
        q.queue(3);
        q.queue(45);
        let target = q.peek();
        let result = 1;
        assert_eq!(target, result);
    }

    #[test]
    fn queue_is_empty() {
        let mut q = Queue::<i32>::new();
        let result = true;
        assert_eq!(result, q.is_empty());
    }
    #[test]
    fn queue_is_not_empty() {
        let mut q = Queue::<i32>::new();
        q.queue(1);
        let result = false;
        assert_eq!(result, q.is_empty());
    }
    #[test]
    fn change_queue_capacity_from_none_to_ten() {
        let mut q = Queue::<i32>::new();
        assert_eq!(Ok(()), q.set_capacity(10));
    }
    #[test]
    fn change_queue_capacity_from_ten_to_none() {
        let mut q = Queue::<i32>::with_capacity(10);
        assert_eq!(Ok(()), q.set_capacity(None));
    }
    #[test]
    fn change_queue_capacity_smaller_with_full_capacity() {
        let mut q = Queue::<i32>::with_capacity(10);
        for i in 0..10 {
            q.queue(i);
        }
        assert_eq!(Err(()), q.set_capacity(1));
    }
    #[test]
    fn change_queue_capacity_bigger() {
        let mut q = Queue::<i32>::with_capacity(1);
        assert_eq!(Ok(()), q.set_capacity(10));
    }
}
