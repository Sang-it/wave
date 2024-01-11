#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AssignmentOperator {
    #[cfg_attr(feature = "serde", serde(rename = "="))]
    Assign,
}

impl AssignmentOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assign => "=",
        }
    }
}
