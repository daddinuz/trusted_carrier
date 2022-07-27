use std::collections::VecDeque;
use trusted_carrier::{Badge, Proxy, Unique};

fn main() {
    let mut q = Queue::from(trusted_carrier::badge!());

    #[allow(clippy::needless_collect)]
    let tokens: Vec<_> = (0..10).map(|n| q.put(n)).collect();
    tokens
        .into_iter()
        .rev()
        .for_each(|t| println!("{}", q.pop(t)))
}

pub struct Queue<'id, Id: Unique, T> {
    queue: VecDeque<T>,

    #[allow(dead_code)]
    badge: Badge<'id, Id>,
}

impl<'id, Id: Unique, T> From<Badge<'id, Id>> for Queue<'id, Id, T> {
    fn from(badge: Badge<'id, Id>) -> Self {
        Self {
            queue: VecDeque::new(),
            badge,
        }
    }
}

impl<'id, Id: Unique, T> Queue<'id, Id, T> {
    pub fn pop(&mut self, _: Proxy<'id, Id>) -> T {
        self.queue.pop_front().unwrap()
    }

    pub fn put(&mut self, value: T) -> Proxy<'id, Id> {
        self.queue.push_back(value);
        self.badge.proxy()
    }
}
