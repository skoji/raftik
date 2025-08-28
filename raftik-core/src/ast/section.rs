use super::types::ValueType;
use crate::ast::{
    instructions::RawExpression,
    types::{FunctionType, GlobalType, MemoryType, ReferenceType, TableType},
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
    Element(ElementSection<'a>),
    Code(CodeSection<'a>),
    Data(DataSection<'a>),
    DataCount(DataCountSection),
    Custom(CustomSection<'a>),
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
    pub type_indices: Vec<u32>,
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
    pub expression: RawExpression<'a>,
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

#[derive(Debug, PartialEq, Eq)]
pub struct ElementSection<'a> {
    pub elements: Vec<Element<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element<'a> {
    pub kind: ElementKind<'a>,
    pub items: ElementItems<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElementKind<'a> {
    Active {
        table_index: Option<u32>,
        offset_expression: RawExpression<'a>,
    },
    Declarative,
    Passive,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElementItems<'a> {
    Functions(Vec<u32>),
    Expressions(ReferenceType, Vec<RawExpression<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeSection<'a> {
    pub code: Vec<FunctionBody<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionBody<'a> {
    // do not hold function size here.
    pub locals: Vec<Locals>,
    pub expression: RawExpression<'a>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Locals {
    pub count: u32,
    pub value_type: ValueType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataSection<'a> {
    pub segments: Vec<DataSegment<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataSegment<'a> {
    pub mode: DataMode<'a>,
    pub data: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataMode<'a> {
    Active {
        memory_index: Option<u32>,
        offset_expression: RawExpression<'a>,
    },
    Passive,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataCountSection {
    pub count: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSection<'a> {
    pub name: String,
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
            Section::Element(_) => SectionID::Element,
            Section::Code(_) => SectionID::Code,
            Section::Data(_) => SectionID::Data,
            Section::DataCount(_) => SectionID::DataCount,
            Section::Custom(_) => SectionID::Custom,
        }
    }
}
