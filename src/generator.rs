use core::panic;
use std::fmt::Write;

use anyhow::Result;

use fake::{
    faker::{
        chrono::en::{Date, Time},
        lorem::en::Paragraph,
    },
    uuid::UUIDv4,
    Fake, Faker,
};

use crate::model::{Column, ColumnType, Table};

pub fn generate_dml(table: &Table, size: u32) -> Result<String> {
    let capacity = estimate_capacity(table, size);
    let mut buffer = String::with_capacity(capacity);

    write_table_header(table, &mut buffer)?;
    write_table_values(table, size, &mut buffer)?;

    Ok(buffer)
}

fn estimate_capacity(table: &Table, size: u32) -> usize {
    // calculate a more precise capacity based on field types
    size as usize * table.columns.len() * 1000
}

fn write_table_header(table: &Table, mut buffer: &mut String) -> Result<()> {
    write!(
        &mut buffer,
        "INSERT INTO {}.{}\n\t(",
        table.schema, table.name
    )?;

    let filtered_columns = table.columns.iter().filter(|col| !col.is_generated);
    for (idx, column) in filtered_columns.enumerate() {
        if idx != 0 {
            buffer.write_char(',')?;
        }

        write!(&mut buffer, "{}", column.name)?;
    }

    write!(&mut buffer, ") \nVALUES\n")?;
    Ok(())
}

fn write_table_values(table: &Table, size: u32, mut buffer: &mut String) -> Result<()> {
    let filtered_columns = table
        .columns
        .iter()
        .filter(|col| !col.is_generated)
        .collect::<Vec<_>>();

    for row in 1..=size {
        write!(&mut buffer, "\t(")?;

        for (idx, column) in filtered_columns.iter().enumerate() {
            if idx != 0 {
                buffer.write_char(',')?;
            }

            let is_generated_value = generate_value(column, row);
            write!(&mut buffer, "{}", is_generated_value)?;
        }

        write!(&mut buffer, ")")?;
        if row == size {
            write!(&mut buffer, ";")?;
        } else {
            write!(&mut buffer, ",")?;
        }
        writeln!(&mut buffer)?;
    }
    Ok(())
}

fn generate_value(column: &Column, row_index: u32) -> String {
    // consider constraints first
    if column.is_unique {
        return generate_unique_value(column.typ, row_index);
    }

    // consider annotations next
    // TODO: implement annotation infered values

    // default to random value with no constrains
    generate_random_value(column.typ)
}

fn generate_unique_value(column_type: ColumnType, row_index: u32) -> String {
    let row_index_serialized = row_index.to_string();
    match column_type {
        ColumnType::SmallInt => row_index_serialized,
        ColumnType::Integer => row_index_serialized,
        ColumnType::BigInt => row_index_serialized,
        ColumnType::Real => row_index_serialized,
        ColumnType::Double => row_index_serialized,
        ColumnType::Char(len) => format!(
            "'{}{}'",
            (len..len - row_index_serialized.len()).fake::<String>(),
            row_index
        ),
        ColumnType::Varchar(len) => format!(
            "'{}{}'",
            (1..len - row_index_serialized.len()).fake::<String>(),
            row_index
        ),
        ColumnType::Text => format!("'{}{}'", Paragraph(1..2).fake::<String>(), row_index),
        ColumnType::Uuid => generate_random_value(column_type),

        // TODO: the tricky ones, timestamps, dates, etc.
        _ => unimplemented!("Column type: {:?} does not support unique constraint atm", column_type),
    }
}

fn generate_random_value(column_type: ColumnType) -> String {
    match column_type {
        ColumnType::SmallInt => Faker.fake::<i16>().to_string(),
        ColumnType::Integer => Faker.fake::<i32>().to_string(),
        ColumnType::BigInt => Faker.fake::<i64>().to_string(),
        ColumnType::Real => Faker.fake::<f32>().to_string(),
        ColumnType::Double => Faker.fake::<f64>().to_string(),
        ColumnType::Decimal(precision, scale) => generate_decimal(precision, scale),

        ColumnType::Char(len) => format!("'{}'", (len..len).fake::<String>()),
        ColumnType::Varchar(len) => format!("'{}'", (1..len).fake::<String>()),
        ColumnType::Text => format!("'{}'", Paragraph(1..2).fake::<String>()),

        ColumnType::Uuid => format!("'{}'", UUIDv4.fake::<String>()),
        ColumnType::Boolean => Faker.fake::<bool>().to_string(),

        ColumnType::Date => Date().fake::<String>(),
        ColumnType::Time => Time().fake::<String>(),
        // TODO: fix timestamp, this needs to be an ISO-string
        ColumnType::Timestamp => (0..4102444800).fake::<u32>().to_string(),

        ColumnType::SmallSerial => panic!("tried to generate value for generated column type"),
        ColumnType::Serial => panic!("tried to generate value for generated column type"),
        ColumnType::BigSerial => panic!("tried to generate value for generated column type"),

        _ => unimplemented!("Random value for column type: {:?} is not supported atm", column_type),
    }
}

fn generate_decimal(precision: Option<usize>, scale: Option<usize>) -> String {
    if precision.is_none() && scale.is_none() {
        return format!("{}.{}", Faker.fake::<u32>(), Faker.fake::<u16>());
    }

    if precision.is_none() {
        panic!("encountered a decimal column with precision without scale");
    }

    if scale.is_none() {
        if let Some(_precision_value) = precision {
            // TODO: use precision value to limit maximum u32
            return format!("{}", Faker.fake::<u32>());
        }
    }

    // TODO: use precision and scala to limit maximum u32s
    format!("{},{}", Faker.fake::<u32>(), Faker.fake::<u32>())
}
