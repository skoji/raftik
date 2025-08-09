use crate::binary::parser::parse_module;

impl TryFrom<&[u8]> for crate::ast::Module {
    type Error = String;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        match parse_module(data) {
            Ok((_, module)) => Ok(module),
            Err(_) => Err("parse failed".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Module;

    #[test]
    fn test_minimal_wasm() {
        let wasm = wat::parse_str("(module)").unwrap();
        let module: Module = Module::try_from(wasm.as_ref()).unwrap();
        assert_eq!(module.magic, [0x00, 0x61, 0x73, 0x6d]);
        assert_eq!(module.version, 1);
    }

    #[test]
    fn test_module_try_from_short_data() {
        let data: &[u8] = &[0x4d, 0x4f];
        let result = Module::try_from(data);
        assert!(result.is_err());
    }
}
