use super::*;

impl TableBuilder for MysqlQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        column_def.name.prepare(sql.as_writer(), self.quote());

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(column_type, sql);
        }

        for column_spec in column_def.spec.iter() {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match column_type {
                ColumnType::Char(length) => match length {
                    Some(length) => format!("char({})", length),
                    None => "char".into(),
                },
                ColumnType::String(length) => match length {
                    Some(length) => format!("varchar({})", length),
                    None => "varchar(255)".into(),
                },
                ColumnType::Text => "text".into(),
                ColumnType::TinyInteger | ColumnType::TinyUnsigned => "tinyint".into(),
                ColumnType::SmallInteger | ColumnType::SmallUnsigned => "smallint".into(),
                ColumnType::Integer | ColumnType::Unsigned => "int".into(),
                ColumnType::BigInteger | ColumnType::BigUnsigned => "bigint".into(),
                ColumnType::Float => "float".into(),
                ColumnType::Double => "double".into(),
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => format!("decimal({}, {})", precision, scale),
                    None => "decimal".into(),
                },
                ColumnType::DateTime => "datetime".into(),
                ColumnType::Timestamp => "timestamp".into(),
                ColumnType::TimestampWithTimeZone => "timestamp".into(),
                ColumnType::Time => "time".into(),
                ColumnType::Date => "date".into(),
                ColumnType::Year(length) => {
                    match length {
                        Some(length) => match length {
                            MySqlYear::Two => "year(2)".into(),
                            MySqlYear::Four => "year(4)".into(),
                        },
                        None => "year".into(),
                    }
                }
                ColumnType::Interval(_, _) => "unsupported".into(),
                ColumnType::Binary(blob_size) => match blob_size {
                    BlobSize::Tiny => "tinyblob".into(),
                    BlobSize::Blob(length) => {
                        match length {
                            Some(length) => format!("binary({})", length),
                            None => "blob".into(),
                        }
                    }
                    BlobSize::Medium => "mediumblob".into(),
                    BlobSize::Long => "longblob".into(),
                },
                ColumnType::VarBinary(length) => format!("varbinary({})", length),
                ColumnType::Bit(length) => {
                    match length {
                        Some(length) => format!("bit({})", length),
                        None => "bit".into(),
                    }
                }
                ColumnType::VarBit(length) => {
                    format!("bit({})", length)
                }
                ColumnType::Boolean => "bool".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("money({}, {})", precision, scale),
                    None => "money".into(),
                },
                ColumnType::Json => "json".into(),
                ColumnType::JsonBinary => "json".into(),
                ColumnType::Uuid => "binary(16)".into(),
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum { variants, .. } => format!(
                    "ENUM('{}')",
                    variants
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("', '")
                ),
                ColumnType::Array(_) => unimplemented!("Array is not available in MySQL."),
                ColumnType::Cidr => unimplemented!("Cidr is not available in MySQL."),
                ColumnType::Inet => unimplemented!("Inet is not available in MySQL."),
                ColumnType::MacAddr => unimplemented!("MacAddr is not available in MySQL."),
            }
        )
        .unwrap();
        if matches!(
            column_type,
            ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned
        ) {
            write!(sql, " ").unwrap();
            write!(sql, "UNSIGNED").unwrap();
        }
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        "AUTO_INCREMENT"
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            write!(sql, " ").unwrap();
        }
        alter.options.iter().fold(true, |first, option| {
            if !first {
                write!(sql, ", ").unwrap();
            };
            match option {
                TableAlterOption::AddColumn(AddColumnOption {
                    column,
                    if_not_exists,
                }) => {
                    write!(sql, "ADD COLUMN ").unwrap();
                    if *if_not_exists {
                        write!(sql, "IF NOT EXISTS ").unwrap();
                    }
                    self.prepare_column_def(column, sql);
                }
                TableAlterOption::ModifyColumn(column_def) => {
                    write!(sql, "MODIFY COLUMN ").unwrap();
                    self.prepare_column_def(column_def, sql);
                }
                TableAlterOption::RenameColumn(from_name, to_name) => {
                    write!(sql, "RENAME COLUMN ").unwrap();
                    from_name.prepare(sql.as_writer(), self.quote());
                    write!(sql, " TO ").unwrap();
                    to_name.prepare(sql.as_writer(), self.quote());
                }
                TableAlterOption::DropColumn(column_name) => {
                    write!(sql, "DROP COLUMN ").unwrap();
                    column_name.prepare(sql.as_writer(), self.quote());
                }
                TableAlterOption::DropForeignKey(name) => {
                    let mut foreign_key = TableForeignKey::new();
                    foreign_key.name(&name.to_string());
                    let drop = ForeignKeyDropStatement {
                        foreign_key,
                        table: None,
                    };
                    self.prepare_foreign_key_drop_statement_internal(&drop, sql, Mode::TableAlter);
                }
                TableAlterOption::AddForeignKey(foreign_key) => {
                    let create = ForeignKeyCreateStatement {
                        foreign_key: foreign_key.to_owned(),
                    };
                    self.prepare_foreign_key_create_statement_internal(
                        &create,
                        sql,
                        Mode::TableAlter,
                    );
                }
            };
            false
        });
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "RENAME TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        write!(sql, " TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }
}
