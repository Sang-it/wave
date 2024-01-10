mod arena;

pub use arena::{Box, String, Vec};

use bumpalo::Bump;
use std::{convert::From, ops::Deref};

#[derive(Default)]
pub struct Allocator {
    bump: Bump,
}

impl From<Bump> for Allocator {
    fn from(bump: Bump) -> Self {
        Self { bump }
    }
}

impl Deref for Allocator {
    type Target = Bump;

    fn deref(&self) -> &Self::Target {
        &self.bump
    }
}

#[cfg(test)]
mod test {
    use crate::Allocator;
    use bumpalo::Bump;
    use std::ops::Deref;

    #[test]
    fn test_api() {
        let bump = Bump::new();
        let allocator: Allocator = bump.into();
        {
            _ = allocator.deref();
        }
    }
}
