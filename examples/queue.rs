use std::collections::VecDeque;
use trusted_carrier::{identity, Auth, Grant, Guard, Identity};

fn main() {
    let mut guard = Guard;
    let mut queue = Queue::new(&mut guard, identity!());

    #[allow(clippy::needless_collect)]
    let grants: Vec<_> = (0..10).map(|n| queue.put(n)).collect();
    grants
        .into_iter()
        .for_each(|g| println!("{}", queue.pop(g)))
}

pub struct Queue<'guard, Id, T>
where
    Id: Identity,
{
    data: VecDeque<T>,
    #[allow(dead_code)]
    auth: Auth<'guard, Id>,
}

impl<'guard, Id: Identity, T> Queue<'guard, Id, T> {
    pub fn new(guard: &'guard mut Guard, id: Id) -> Self {
        Self {
            data: VecDeque::new(),
            auth: Auth::new(guard, id),
        }
    }

    pub fn pop(&mut self, _: Grant<'guard, Id>) -> T {
        self.data.pop_front().unwrap()
    }

    pub fn put(&mut self, value: T) -> Grant<'guard, Id> {
        self.data.push_back(value);
        self.auth.grant()
    }
}
