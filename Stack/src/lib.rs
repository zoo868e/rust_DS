#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    data: Vec<T>,
    capacity: Option<usize>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::<T>::new(),
            capacity: None,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            data: Vec::<T>::with_capacity(capacity),
            capacity: Some(capacity),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn push(&mut self, value: T) -> Result<usize, &'static str> {
        match self.capacity {
            None => {
                self.data.push(value);
                Ok(self.data.len())
            }
            Some(capacity) => {
                if self.data.len() >= capacity {
                    Err("The stack is full")
                } else {
                    self.data.push(value);
                    Ok(self.data.len())
                }
            }
        }
    }
    pub fn force_push(&mut self, value: T) -> usize {
        if self.data.len() >= self.capacity.unwrap() {
            self.data.pop();
        }
        self.data.push(value);
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.data.is_empty() {
            Err("The stack is empty")
        } else {
            Ok(self.data.pop().unwrap())
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    pub fn set_capacity<C: Into<Option<usize>>>(
        &mut self,
        capacity: C,
    ) -> Result<Option<usize>, &'static str> {
        let capacity = capacity.into();
        match capacity {
            None => {
                self.capacity = None;
                self.data.shrink_to_fit();
                Ok(None)
            }
            Some(capacity) => {
                if capacity < self.data.len() {
                    Err("The capacity is too small")
                } else {
                    self.capacity = Some(capacity);
                    self.data.shrink_to_fit();
                    self.data.reserve(capacity - self.data.len());
                    Ok(Some(capacity))
                }
            }
        }
    }
    pub fn force_set_capacity<C: Into<Option<usize>>>(&mut self, capacity: C) -> Option<usize> {
        let capacity = capacity.into();
        match capacity {
            None => {
                self.capacity = None;
                self.data.shrink_to_fit();
                self.capacity
            }
            Some(capacity) => {
                if capacity < self.data.len() {
                    while capacity < self.data.len() {
                        self.data.pop();
                    }
                } else if capacity > self.data.capacity() {
                    self.data.reserve(capacity - self.data.capacity());
                }
                self.capacity = Some(capacity);
                self.capacity
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Initialize_without_capacity() {
        let target = Stack::<i32>::new();
        let result = Stack {
            data: Vec::<i32>::new(),
            capacity: None,
        };
        assert_eq!(result, target);
        let result = 0;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Initialize_with_capacity() {
        let target = Stack::<i32>::with_capacity(10);
        let result = Stack {
            data: Vec::<i32>::with_capacity(10),
            capacity: Some(10),
        };
        assert_eq!(result, target);
        let result = 0;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Push_element_to_empty_stack() {
        let mut target = Stack::<i32>::new();
        let result = Ok(1);
        assert_eq!(result, target.push(1));
        let result = Stack {
            data: vec![1],
            capacity: None,
        };
        assert_eq!(result, target);
    }
    #[test]
    fn Push_element_to_fulled_stack() {
        let mut target = Stack::<i32>::with_capacity(10);
        for i in 0..10 {
            target.push(i);
        }
        let result = Err("The stack is full");
        assert_eq!(result, target.push(10));
        let result = 10;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Force_push_element_to_full_stack() {
        let mut target = Stack::<i32>::with_capacity(10);
        for i in 0..10 {
            target.push(i);
        }
        let result = 10;
        assert_eq!(result, target.force_push(10));
        let result = Stack {
            data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10],
            capacity: Some(10),
        };
        assert_eq!(result, target);
    }
    #[test]
    fn Stack_is_empty() {
        let target = Stack::<i32>::new();
        let result = true;
        assert_eq!(result, target.is_empty());
    }
    #[test]
    fn Stack_is_not_empty() {
        let mut target = Stack::<i32>::with_capacity(10);
        target.push(1);
        let result = false;
        assert_eq!(result, target.is_empty());
    }
    #[test]
    fn Pop_element_from_empty_stack() {
        let mut target = Stack::<i32>::new();
        let result = Err("The stack is empty");
        assert_eq!(result, target.pop());
        let result = 0;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Pop_element_from_full_stack() {
        let mut target = Stack::<i32>::with_capacity(10);
        for i in 0..10 {
            target.push(i);
        }
        let result = 9;
        assert_eq!(result, target.pop().unwrap());
        let result: usize = 9;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Peek_element_from_empty_stack() {
        let target = Stack::<i32>::new();
        let result = None;
        assert_eq!(result, target.peek());
        let result = 0;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Peek_element_from_stack() {
        let mut target = Stack::<i32>::with_capacity(10);
        for i in 0..10 {
            target.push(i);
        }
        let result = 9;
        assert_eq!(&result, target.peek().unwrap());
        let result = 10;
        assert_eq!(result, target.len());
    }
    #[test]
    fn Set_capacity_of_none_capacity_empty_stack() {
        let mut target = Stack::<i32>::new();
        let result = Ok(Some(10));
        assert_eq!(result, target.set_capacity(10));
        let result = Stack {
            data: Vec::<i32>::with_capacity(10),
            capacity: Some(10),
        };
        assert_eq!(result, target);
    }
    #[test]
    fn Set_capacity_of_empty_stack_to_none() {
        let mut target = Stack::<i32>::with_capacity(10);
        let result = Ok(None);
        assert_eq!(result, target.set_capacity(None));
        let result = Stack {
            data: Vec::<i32>::new(),
            capacity: None,
        };
        assert_eq!(result, target);
    }
    /*
       Remove the redundant memory allocations.
    */
    #[test]
    fn Set_capacity_of_stack_to_none() {
        let mut target = Stack::<i32>::with_capacity(10);
        target.push(1);
        let result = Ok(None);
        assert_eq!(result, target.set_capacity(None));
        let result = Stack {
            data: vec![1],
            capacity: None,
        };
        assert_eq!(result, target);
    }
    #[test]
    fn Set_capacity_too_small() {
        let mut target = Stack::<i32>::new();
        for i in 0..10 {
            target.push(i);
        }
        let result = Err("The capacity is too small");
        assert_eq!(result, target.set_capacity(5));
        let result = Stack {
            data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            capacity: None,
        };
        assert_eq!(result, target);
    }
    /*
       Pop the exceeding elements from the stack if the new capacity is smaller than the current capacity.
    */
    #[test]
    fn Force_to_set_capacity_too_small() {
        let mut target = Stack::<i32>::new();
        for i in 0..10 {
            target.push(i);
        }
        let result = Some(5);
        assert_eq!(result, target.force_set_capacity(5));
        let result = Stack {
            data: vec![0, 1, 2, 3, 4],
            capacity: Some(5),
        };
        assert_eq!(result, target);
    }
    #[test]
    fn Get_the_size_of_stack() {
        let mut target = Stack::<i32>::new();
        for i in 0..10 {
            target.push(i);
        }
        let result = 10;
        assert_eq!(result, target.len());
    }
}
