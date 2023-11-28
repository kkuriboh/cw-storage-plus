use crate::{Key, Path, PrimaryKey};
use cosmwasm_std::{StdResult, Storage};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Set<'a, V> {
    namespace: &'a [u8],
    _val_type: PhantomData<V>,
}

impl<'a, V> Set<'a, V> {
    pub const fn new(namespace: &'a str) -> Self {
        Set {
            namespace: namespace.as_bytes(),
            _val_type: PhantomData,
        }
    }

    pub fn namespace(&self) -> &'a [u8] {
        self.namespace
    }
}

impl<'a, V> Set<'a, V>
where
    V: PrimaryKey<'a>,
{
    pub fn key(&self, v: V) -> Path<()> {
        Path::new(
            self.namespace,
            &v.key().iter().map(Key::as_ref).collect::<Box<_>>(),
        )
    }

    pub fn exists(&self, store: &dyn Storage, v: V) -> bool {
        self.key(v).has(store)
    }

    pub fn save(&self, store: &mut dyn Storage, k: V) -> StdResult<()> {
        self.key(k).save(store, &())
    }
}
