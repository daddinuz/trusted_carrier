use std::{fmt, marker::PhantomData};
use trusted_carrier::{Badge, Proxy, Unique};

fn main() {
    let mut c = Container::from(trusted_carrier::badge!());
    let h1 = c.push(42);
    let h2 = c.push(24);
    println!("{} {}", c.get(h1), c.get(h2));

    // Handles may outlive parent Container
    let (c1_h1, c1_h2) = {
        let mut c1 = Container::from(trusted_carrier::badge!());
        let c1_h1 = c1.push(42);
        let c1_h2 = c1.push(24);
        (c1_h1, c1_h2)
    };

    println!("{:?}\n{:?}", c1_h1, c1_h2);

    /*
    // But they can't be used to access another container
    let mut c2 = Container::from(trusted_carrier::badge!());
    let c2_h1 = c2.push(42);
    let c2_h2 = c2.push(24);
    println!("{} {}", c2.get(c1_h1), c2.get(c1_h2));
    */

    /*
    // Cannot mix badges
    let mut c1 = Container::from(trusted_carrier::badge!());
    let c1_h1 = c1.push(42);
    let c1_h2 = c1.push(24);
    println!("{} {}", c1.get(c1_h1), c1.get(c1_h2));

    let mut c2 = Container::from(trusted_carrier::badge!());
    let c2_h1 = c2.push(42);
    let c2_h2 = c2.push(24);
    println!("{} {}", c2.get(c2_h1), c2.get(c2_h2));

    // println!("{} {}", c2.get(c1_h1), c2.get(c1_h2));
    */
}

pub struct Container<'id, Id, T>
where
    Id: Unique,
{
    array: Vec<T>,

    #[allow(dead_code)]
    badge: Badge<'id, Id>,
}

impl<'id, Id, T> From<Badge<'id, Id>> for Container<'id, Id, T>
where
    Id: Unique,
{
    fn from(badge: Badge<'id, Id>) -> Self {
        Self {
            array: Vec::new(),
            badge,
        }
    }
}

impl<'id, Id, T> Container<'id, Id, T>
where
    Id: Unique,
{
    pub fn push(&mut self, value: T) -> Handle<'id, Id, T> {
        let index = self.array.len();
        self.array.push(value);
        Handle::new(&self.badge, index)
    }

    pub fn get(&self, handle: Handle<'id, Id, T>) -> &T {
        unsafe { self.array.get_unchecked(handle.index) }
    }

    pub fn get_mut(&mut self, handle: Handle<'id, Id, T>) -> &mut T {
        unsafe { self.array.get_unchecked_mut(handle.index) }
    }
}

pub struct Handle<'id, Id, T>
where
    Id: Unique,
{
    index: usize,
    class: PhantomData<T>,

    #[allow(dead_code)]
    proxy: Proxy<'id, Id>,
}

impl<'id, Id, T> Handle<'id, Id, T>
where
    Id: Unique,
{
    fn new(badge: &Badge<'id, Id>, index: usize) -> Self {
        Self {
            index,
            class: PhantomData,
            proxy: badge.proxy(),
        }
    }
}

impl<'id, Id, T> fmt::Debug for Handle<'id, Id, T>
where
    Id: Unique,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle")
            .field("index", &self.index)
            .finish()
    }
}
