use std::alloc::{alloc, dealloc, Layout};

struct MyCircularDeque {
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
impl MyCircularDeque {
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

    fn insert_front(&mut self, value: i32) -> bool {
        !self.is_full() && {
            self.offset = (self.capacity + self.offset - 1) % self.capacity;
            self.len += 1;
            self.set(self.front_index(), value);
            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        !self.is_full() && {
            self.len += 1;
            self.set(self.last_index(), value);
            true
        }
    }

    fn delete_front(&mut self) -> bool {
        !self.is_empty() && {
            self.offset = (self.offset + 1) % self.capacity;
            self.len -= 1;
            true
        }
    }

    fn delete_last(&mut self) -> bool {
        !self.is_empty() && {
            self.len -= 1;
            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.get(self.front_index())
        }
    }

    fn get_rear(&self) -> i32 {
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

impl Drop for MyCircularDeque {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr as *mut _, self.layout) }
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

fn main() {
    let mut circular_deque = MyCircularDeque::new(3); // 设置长度为 3
    println!("{}", circular_deque.insert_last(1));
    println!("{}", circular_deque.insert_last(2));
    println!("{}", circular_deque.insert_front(3));
    println!("{}", circular_deque.insert_last(4));
    println!("{}", circular_deque.insert_front(4));
    println!("{}", circular_deque.get_front());
    println!("{}", circular_deque.get_rear());
    println!("{}", circular_deque.is_full());
    println!("{}", circular_deque.delete_last());
    println!("{}", circular_deque.insert_front(4));
    println!("{}", circular_deque.get_front());
    println!("{}", circular_deque.get_rear());
    println!("{}", circular_deque.delete_front());
    println!("{}", circular_deque.insert_last(4));
    println!("{}", circular_deque.get_front());
    println!("{}", circular_deque.get_rear());
    println!("{}", circular_deque.delete_front());
    println!("{}", circular_deque.delete_last());
    println!("{}", circular_deque.is_empty());
    println!("{}", circular_deque.get_front());
    println!("{}", circular_deque.get_rear());
    println!("{}", circular_deque.delete_last());
    println!("{}", circular_deque.is_empty());
    println!("{}", circular_deque.get_front());
    println!("{}", circular_deque.get_rear());
}
