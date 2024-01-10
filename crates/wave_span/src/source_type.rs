use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct SourceType {
    language: Language,
    module_kind: ModuleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "camelCase"))]
pub enum Language {
    Wave,
    SExpressionWave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "camelCase"))]
pub enum ModuleKind {
    Script,
    Module,
}

#[derive(Debug)]
pub struct UnknownExtension(pub String);

impl Default for SourceType {
    fn default() -> Self {
        Self {
            language: Language::Wave,
            module_kind: ModuleKind::Script,
        }
    }
}

pub const VALID_EXTENSIONS: [&str; 4] = ["wave", "wv", "swv", "swave"];

impl SourceType {
    pub fn is_script(self) -> bool {
        self.module_kind == ModuleKind::Script
    }

    pub fn is_module(self) -> bool {
        self.module_kind == ModuleKind::Module
    }

    pub fn module_kind(self) -> ModuleKind {
        self.module_kind
    }

    pub fn is_wave(self) -> bool {
        matches!(self.language, Language::Wave)
    }

    #[must_use]
    pub fn with_script(mut self, yes: bool) -> Self {
        if yes {
            self.module_kind = ModuleKind::Script;
        }
        self
    }

    #[must_use]
    pub fn with_module(mut self, yes: bool) -> Self {
        if yes {
            self.module_kind = ModuleKind::Module;
        } else {
            self.module_kind = ModuleKind::Script;
        }
        self
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, UnknownExtension> {
        let _file_name = path
            .as_ref()
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| UnknownExtension("Please provide a valid file name.".to_string()))?;

        let extension = path
            .as_ref()
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .filter(|s| VALID_EXTENSIONS.contains(s))
            .ok_or_else(|| {
                let path = path.as_ref().to_string_lossy();
                UnknownExtension(
                    format!("Please provide a valid file extension for {path}: .wv, .swv, .wave or .swave for Wave"),
                )
            })?;

        let language = match extension {
            "wv" | "wave" => Language::Wave,
            _ => unreachable!(),
        };

        Ok(Self {
            language,
            module_kind: ModuleKind::Module,
        })
    }
}
