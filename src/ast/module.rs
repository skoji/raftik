#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub magic: [u8; 4],
    pub version: u32,
}

impl TryFrom<&[u8]> for Module {
    type Error = String;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err("Data too short to contain a valid Module".to_string());
        }

        let magic = [data[0], data[1], data[2], data[3]];
        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

        Ok(Module { magic, version })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
