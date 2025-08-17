use crate::ast::{
    instructions::Expression,
    types::{FunctionType, GlobalType, MemoryType, TableType},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Section<'a> {
    Type(TypeSection),
    Import(ImportSection),
    Function(FunctionSection),
    Table(TableSection),
    Memory(MemorySection),
    Global(GlobalSection<'a>),
    Export(ExportSection),
    Start(StartSection),
    Unknown(UnknownSection<'a>),
    // Other section coming
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSection {
    pub types: Vec<FunctionType>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportSection {
    pub imports: Vec<Import>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportDesc {
    TypeIndex(u32),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionSection {
    pub type_indexes: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TableSection {
    pub tables: Vec<TableType>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MemorySection {
    pub memories: Vec<MemoryType>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GlobalSection<'a> {
    pub globals: Vec<Global<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Global<'a> {
    pub global_type: GlobalType,
    pub expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExportSection {
    pub exports: Vec<Export>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExportDesc {
    FunctionIndex(u32),
    TableIndex(u32),
    MemoryIndex(u32),
    GlobalIndex(u32),
}
#[derive(Debug, PartialEq, Eq)]
pub struct StartSection {
    pub start_function_index: u32,
}

// will be removed all section parser is implemented.
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownSection<'a> {
    pub id: SectionID,
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SectionID {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}

impl TryFrom<u8> for SectionID {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Custom),
            1 => Ok(Self::Type),
            2 => Ok(Self::Import),
            3 => Ok(Self::Function),
            4 => Ok(Self::Table),
            5 => Ok(Self::Memory),
            6 => Ok(Self::Global),
            7 => Ok(Self::Export),
            8 => Ok(Self::Start),
            9 => Ok(Self::Element),
            10 => Ok(Self::Code),
            11 => Ok(Self::Data),
            12 => Ok(Self::DataCount),
            _ => Err(()),
        }
    }
}

impl Section<'_> {
    pub fn id(&self) -> SectionID {
        match self {
            Section::Type(_) => SectionID::Type,
            Section::Import(_) => SectionID::Import,
            Section::Function(_) => SectionID::Function,
            Section::Table(_) => SectionID::Table,
            Section::Memory(_) => SectionID::Memory,
            Section::Global(_) => SectionID::Global,
            Section::Export(_) => SectionID::Export,
            Section::Start(_) => SectionID::Start,
            Section::Unknown(unknown) => unknown.id,
        }
    }
}
