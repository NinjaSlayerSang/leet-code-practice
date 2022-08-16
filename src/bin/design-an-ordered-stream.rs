struct OrderedStream {
    queue: Vec<Option<String>>,
    ptr: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            queue: vec![None; n as usize],
            ptr: 0,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let i = (id_key - 1) as usize;
        self.queue[i] = Some(value);

        let mut result = Vec::<String>::new();

        while let Some(s) = self.queue.get(self.ptr).cloned().unwrap_or(None) {
            result.push(s);
            self.ptr += 1;
        }

        result
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

fn main() {
    let mut os = OrderedStream::new(5);
    println!("{:?}", os.insert(3, "ccccc".to_string())); // 插入 (3, "ccccc")，返回 []
    println!("{:?}", os.insert(1, "aaaaa".to_string())); // 插入 (1, "aaaaa")，返回 ["aaaaa"]
    println!("{:?}", os.insert(2, "bbbbb".to_string())); // 插入 (2, "bbbbb")，返回 ["bbbbb", "ccccc"]
    println!("{:?}", os.insert(5, "eeeee".to_string())); // 插入 (5, "eeeee")，返回 []
    println!("{:?}", os.insert(4, "ddddd".to_string())); // 插入 (4, "ddddd")，返回 ["ddddd", "eeeee"]
}
