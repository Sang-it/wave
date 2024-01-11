use bitflags::bitflags;
use wave_index::define_index_type;

define_index_type! {
    pub struct SymbolId = u32;
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    pub struct SymbolFlags: u32 {
        const None                    = 0;
        const BlockScopedVariable     = 1 << 0;
        const Variable = Self::BlockScopedVariable.bits();
        const Value = Self::Variable.bits();
    }
}

impl SymbolFlags {
    pub fn is_variable(&self) -> bool {
        self.intersects(Self::Variable)
    }

    pub fn is_type(&self) -> bool {
        !self.intersects(Self::Value)
    }
}
