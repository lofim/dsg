use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,

    pub schema: String,

    pub columns: Vec<Column>,

    #[serde(skip_deserializing)]
    pub generated_keys: Vec<Vec<String>>,
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

    #[serde(rename = "foreginKey")]
    pub foreign_key: Option<ForeignKey>,
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
    Uuid,
    Bit(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ForeignKey {
    OneToOne,
    OneToMany,
    ManyToMany,
}
