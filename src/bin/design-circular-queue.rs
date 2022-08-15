use std::alloc::{alloc, dealloc, Layout};

struct MyCircularQueue {
    ptr: usize,
    layout: Layout,
    capacity: usize,
    offset: usize,
    len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn addr(&self, i: usize) -> usize {
        self.ptr + i * self.layout.align()
    }

    fn get(&self, i: usize) -> i32 {
        unsafe { *(self.addr(i) as *const i32) }
    }

    fn set(&mut self, i: usize, v: i32) {
        unsafe { *(self.addr(i) as *mut i32) = v }
    }

    fn front_index(&self) -> usize {
        self.offset % self.capacity
    }

    fn last_index(&self) -> usize {
        (self.offset + self.len - 1) % self.capacity
    }

    fn new(k: i32) -> Self {
        let capacity = k as usize;
        let layout = Layout::array::<i32>(capacity).unwrap();
        let ptr;
        unsafe { ptr = alloc(layout) as usize }
        Self {
            ptr,
            layout,
            capacity,
            offset: 0,
            len: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        !self.is_full() && {
            self.len += 1;
            self.set(self.last_index(), value);
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        !self.is_empty() && {
            self.offset = (self.offset + 1) % self.capacity;
            self.len -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.get(self.front_index())
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.get(self.last_index())
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

impl Drop for MyCircularQueue {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr as *mut _, self.layout) }
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
    let mut circular_queue = MyCircularQueue::new(8); // 设置长度为 3
    println!("{}", circular_queue.en_queue(3));
    println!("{}", circular_queue.en_queue(9));
    println!("{}", circular_queue.en_queue(5));
    println!("{}", circular_queue.en_queue(0));
    println!("{}", circular_queue.front());
    println!("{}", circular_queue.rear());
    println!("{}", circular_queue.is_full());
    println!("{}", circular_queue.de_queue());
    println!("{}", circular_queue.de_queue());
    println!("{}", circular_queue.front());
    println!("{}", circular_queue.rear());
    println!("{}", circular_queue.is_empty());
}
