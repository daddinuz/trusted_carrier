use trusted_carrier::{identity, Auth, Guard, Identity, Trusted};

fn main() {
    let mut guard = Guard;
    let mut array = Array::new(&mut guard, identity!(), [0; 3]);
    let one = array.index(1).unwrap();
    let two = array.index(2).unwrap();

    let v1 = array.get_mut(&one);
    *v1 = 1;

    let v2 = array.get_mut(&two);
    *v2 = 2;

    println!("v1={} , v2={}", array.get(&one), array.get(&two));
}

pub struct Array<'guard, Id, T, const N: usize>
where
    Id: Identity,
{
    data: [T; N],
    auth: Auth<'guard, Id>,
}

impl<'guard, Id, T, const N: usize> Array<'guard, Id, T, N>
where
    Id: Identity,
{
    pub fn new(guard: &'guard mut Guard, id: Id, data: [T; N]) -> Self {
        Self {
            auth: Auth::new(guard, id),
            data,
        }
    }

    pub fn index(&self, index: usize) -> Option<Index<'guard, Id>> {
        if index >= self.data.len() {
            return None;
        }

        Some(self.auth.grant().to(index))
    }

    pub fn get(&self, index: &Index<'guard, Id>) -> &T {
        unsafe { self.data.get_unchecked(index.value()) }
    }

    pub fn get_mut(&mut self, index: &Index<'guard, Id>) -> &mut T {
        unsafe { self.data.get_unchecked_mut(index.value()) }
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }
}

pub type Index<'guard, Id> = Trusted<'guard, Id, usize>;
