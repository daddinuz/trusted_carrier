use std::collections::VecDeque;
use trusted_carrier::{Auth, Grant, Identity};

fn main() {
    let mut q = Queue::from(trusted_carrier::auth!());

    #[allow(clippy::needless_collect)]
    let tokens: Vec<_> = (0..10).map(|n| q.put(n)).collect();
    tokens
        .into_iter()
        .rev()
        .for_each(|t| println!("{}", q.pop(t)))
}

pub struct Queue<'id, Id, T>
where
    Id: Identity,
{
    data: VecDeque<T>,
    #[allow(dead_code)]
    auth: Auth<'id, Id>,
}

impl<'id, Id: Identity, T> From<Auth<'id, Id>> for Queue<'id, Id, T> {
    fn from(auth: Auth<'id, Id>) -> Self {
        Self {
            data: VecDeque::new(),
            auth,
        }
    }
}

impl<'id, Id: Identity, T> Queue<'id, Id, T> {
    pub fn pop(&mut self, _: Grant<'id, Id>) -> T {
        self.data.pop_front().unwrap()
    }

    pub fn put(&mut self, value: T) -> Grant<'id, Id> {
        self.data.push_back(value);
        self.auth.grant()
    }
}
