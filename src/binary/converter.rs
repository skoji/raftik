use crate::binary::parser::parse_module;

impl<'a> TryFrom<&'a [u8]> for crate::binary::raw_module::RawModule<'a> {
    type Error = String;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        match parse_module(data) {
            Ok((_, module)) => Ok(module),
            Err(e) => Err(format!("parse failed: {:?}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::raw_module::RawModule;
    use crate::ast::section::*;

    #[test]
    fn test_minimal_wasm() {
        let wasm = wat::parse_str("(module)").unwrap();
        let module: RawModule = RawModule::try_from(wasm.as_ref()).unwrap();
        assert_eq!(module.magic, [0x00, 0x61, 0x73, 0x6d]);
        assert_eq!(module.version, 1);
        assert_eq!(module.sections.len(), 0);
    }

    #[test]
    fn test_module_with_only_type_section() {
        let wasm = wat::parse_str(
            "(module
                (type (func))
            )",
        )
        .unwrap();
        let module: RawModule = RawModule::try_from(wasm.as_ref()).unwrap();
        assert_eq!(module.magic, [0x00, 0x61, 0x73, 0x6d]);
        assert_eq!(module.version, 1);
        assert_eq!(module.sections.len(), 1);
        assert_eq!(module.sections[0].header.id, SectionId::Type as u8);
        assert_eq!(module.sections[0].header.payload_length, 4);
    }

    #[test]
    fn test_module_try_from_short_data() {
        let data: &[u8] = &[0x4d, 0x4f];
        let result = RawModule::try_from(data);
        assert!(result.is_err());
    }
}
