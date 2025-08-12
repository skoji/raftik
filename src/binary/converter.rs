use super::parser::parse_raw_module;
use crate::ast::Module;

impl TryFrom<&[u8]> for Module {
    type Error = String;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let (remaining, _) =
            parse_raw_module(input).map_err(|e| format!("Failed to parse raw module: {}", e))?;

        if !remaining.is_empty() {
            return Err("Extra data after module".to_string());
        }

        Ok(Module { type_section: None })
    }
}
