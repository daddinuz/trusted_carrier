use trusted_carrier::{Auth, Identity, SharedToken};

fn main() {
    let mut arr = Array::new(trusted_carrier::auth!(), [0; 3]);
    let one = arr.index(1).unwrap();
    let two = arr.index(2).unwrap();

    let v1 = arr.get_mut(one);
    *v1 = 1;

    let v2 = arr.get_mut(two);
    *v2 = 2;

    println!("v1 = {} , v2 = {}", arr.get(one), arr.get(two));
}

pub struct Array<'id, Id, T, const N: usize>
where
    Id: Identity,
{
    data: [T; N],
    auth: Auth<'id, Id>,
}

impl<'id, Id, T, const N: usize> From<Auth<'id, Id>> for Array<'id, Id, T, N>
where
    T: Default + Copy,
    Id: Identity,
{
    fn from(auth: Auth<'id, Id>) -> Self {
        Self {
            data: [Default::default(); N],
            auth,
        }
    }
}

impl<'id, Id, T, const N: usize> Array<'id, Id, T, N>
where
    Id: Identity,
{
    pub fn new(auth: Auth<'id, Id>, data: [T; N]) -> Self {
        Self { auth, data }
    }

    pub fn index(&self, index: usize) -> Option<Index<'id, Id>> {
        if index >= self.data.len() {
            return None;
        }

        Some(Index::new(self.auth.grant_shared(), index))
    }

    pub fn get(&self, index: Index<'id, Id>) -> &T {
        unsafe { self.data.get_unchecked(index.index) }
    }

    pub fn get_mut(&mut self, index: Index<'id, Id>) -> &mut T {
        unsafe { self.data.get_unchecked_mut(index.index) }
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }
}

#[derive(Copy, Clone)]
pub struct Index<'id, Id>
where
    Id: Identity,
{
    index: usize,
    #[allow(dead_code)]
    token: SharedToken<'id, Id>,
}

impl<'id, Id> Index<'id, Id>
where
    Id: Identity,
{
    fn new(token: SharedToken<'id, Id>, index: usize) -> Self {
        Self { index, token }
    }
}
