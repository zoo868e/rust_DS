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
}
