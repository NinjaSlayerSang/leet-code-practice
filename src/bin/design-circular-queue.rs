struct MyCircularQueue {
    queue: Vec<i32>,
    capacity: usize,
    size: usize,
    offset: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let capacity = k as usize;
        Self {
            queue: vec![0; capacity],
            capacity,
            size: 0,
            offset: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        !self.is_full() && {
            self.size += 1;
            self.queue[(self.offset + self.size - 1) % self.capacity] = value;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        !self.is_empty() && {
            self.size -= 1;
            self.offset += 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.queue[self.offset % self.capacity]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.queue[(self.offset + self.size - 1) % self.capacity]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

fn main() {
    let mut circular_queue = MyCircularQueue::new(3); // 设置长度为 3
    println!("{}", circular_queue.en_queue(1));
    println!("{}", circular_queue.en_queue(2));
    println!("{}", circular_queue.en_queue(3));
    println!("{}", circular_queue.en_queue(4));
    println!("{}", circular_queue.front());
    println!("{}", circular_queue.rear());
    println!("{}", circular_queue.is_full());
    println!("{}", circular_queue.de_queue());
    println!("{}", circular_queue.en_queue(4));
    println!("{}", circular_queue.rear());
}
