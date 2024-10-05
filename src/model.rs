use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub columns: Vec<Column>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub typ: ColumnType,
    pub annotation: Option<String>,
    
    // constraints
    #[serde(rename = "isGenerated")]
    pub is_generated: bool,

    #[serde(rename = "isUnique")]
    pub is_unique: bool,
    
    #[serde(rename = "isNullable")]
    pub is_nullable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColumnType {
    // Numberic
    SmallInt,
    Integer,
    BigInt,
    
    // Precision, Scale
    Decimal(Option<usize>, Option<usize>),

    // Char
    Char(usize),
    Varchar(usize),
    Text,

    // Numbers
    Real,
    Double,
    SmallSerial,
    Serial,
    BigSerial,

    // Monetary
    Money,
    Bytea,

    // Time
    Timestamp,
    Date,
    Time,
    Interval,

    // Bool
    Boolean,

    // Misc
    UUID,
    Bit(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Constraint {
    // Cols
    Unique(Vec<String>),
    
    // We probably don't care for PKs either.
    // They will be treated as unique.
    PrimaryKey(Vec<String>),
    
    // Cols, Refs
    ForeginKey(Vec<String>, Vec<String>),
}
