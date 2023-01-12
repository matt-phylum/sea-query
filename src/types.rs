//! Base types used throughout sea-query.

use crate::{expr::*, query::*, FunctionCall, ValueTuple, Values};
use std::fmt;

#[cfg(feature = "backend-postgres")]
use crate::extension::postgres::PgBinOper;
#[cfg(feature = "backend-sqlite")]
use crate::extension::sqlite::SqliteBinOper;
#[cfg(not(feature = "thread-safe"))]
pub use std::rc::Rc as SeaRc;
#[cfg(feature = "thread-safe")]
pub use std::sync::Arc as SeaRc;

macro_rules! iden_trait {
    ($($bounds:ident),*) => {
        /// Identifier
        pub trait Iden where $(Self: $bounds),* {
            fn prepare(&self, s: &mut dyn fmt::Write, q: char) {
                write!(s, "{}{}{}", q, self.quoted(q), q).unwrap();
            }

            fn quoted(&self, q: char) -> String {
                let mut b = [0; 4];
                let qq: &str = q.encode_utf8(&mut b);
                self.to_string().replace(qq, qq.repeat(2).as_str())
            }

            fn to_string(&self) -> String {
                let mut s = String::new();
                self.unquoted(&mut s);
                s
            }

            fn unquoted(&self, s: &mut dyn fmt::Write);
        }

        /// Identifier
        pub trait IdenStatic: Iden + Copy + 'static {
            fn as_str(&self) -> &'static str;
        }
    };
}

#[cfg(feature = "thread-safe")]
iden_trait!(Send, Sync);
#[cfg(not(feature = "thread-safe"))]
iden_trait!();

pub type DynIden = SeaRc<dyn Iden>;

pub trait IntoIden {
    fn into_iden(self) -> DynIden;
}

pub trait IdenList {
    type IntoIter: Iterator<Item = DynIden>;

    fn into_iter(self) -> Self::IntoIter;
}

impl fmt::Debug for dyn Iden {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unquoted(formatter);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Identity {
    Unary(DynIden),
    Binary(DynIden, DynIden),
    Ternary(DynIden, DynIden, DynIden),
}

impl Iden for Identity {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        match self {
            Identity::Unary(iden) => {
                write!(s, "{}", iden.to_string()).unwrap();
            }
            Identity::Binary(iden1, iden2) => {
                write!(s, "{}", iden1.to_string()).unwrap();
                write!(s, "{}", iden2.to_string()).unwrap();
            }
            Identity::Ternary(iden1, iden2, iden3) => {
                write!(s, "{}", iden1.to_string()).unwrap();
                write!(s, "{}", iden2.to_string()).unwrap();
                write!(s, "{}", iden3.to_string()).unwrap();
            }
        }
    }
}

pub trait IntoIdentity {
    fn into_identity(self) -> Identity;
}

/// Column references
#[derive(Debug, Clone)]
pub enum ColumnRef {
    Column(DynIden),
    TableColumn(DynIden, DynIden),
    SchemaTableColumn(DynIden, DynIden, DynIden),
    Asterisk,
    TableAsterisk(DynIden),
}

pub trait IntoColumnRef {
    fn into_column_ref(self) -> ColumnRef;
}

/// Table references
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum TableRef {
    /// Table identifier without any schema / database prefix
    Table(DynIden),
    /// Table identifier with schema prefix
    SchemaTable(DynIden, DynIden),
    /// Table identifier with database and schema prefix
    DatabaseSchemaTable(DynIden, DynIden, DynIden),
    /// Table identifier with alias
    TableAlias(DynIden, DynIden),
    /// Table identifier with schema prefix and alias
    SchemaTableAlias(DynIden, DynIden, DynIden),
    /// Table identifier with database and schema prefix and alias
    DatabaseSchemaTableAlias(DynIden, DynIden, DynIden, DynIden),
    /// Subquery with alias
    SubQuery(SelectStatement, DynIden),
    /// Values list with alias
    ValuesList(Vec<ValueTuple>, DynIden),
    /// Function call with alias
    FunctionCall(FunctionCall, DynIden),
}

pub trait IntoTableRef {
    fn into_table_ref(self) -> TableRef;
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOper {
    Not,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOper {
    And,
    Or,
    Like,
    NotLike,
    Is,
    IsNot,
    In,
    NotIn,
    Between,
    NotBetween,
    Equal,
    NotEqual,
    SmallerThan,
    GreaterThan,
    SmallerThanOrEqual,
    GreaterThanOrEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LShift,
    RShift,
    As,
    Escape,
    #[cfg(feature = "backend-postgres")]
    PgOperator(PgBinOper),
    #[cfg(feature = "backend-sqlite")]
    SqliteOperator(SqliteBinOper),
}

/// Logical chain operator
#[derive(Debug, Clone)]
pub enum LogicalChainOper {
    And(SimpleExpr),
    Or(SimpleExpr),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    CrossJoin,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
}

/// Nulls order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullOrdering {
    First,
    Last,
}

/// Order expression
#[derive(Debug, Clone)]
pub struct OrderExpr {
    pub(crate) expr: SimpleExpr,
    pub(crate) order: Order,
    pub(crate) nulls: Option<NullOrdering>,
}

/// Join on types
#[derive(Debug, Clone)]
pub enum JoinOn {
    Condition(Box<ConditionHolder>),
    Columns(Vec<SimpleExpr>),
}

/// Ordering options
#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Asc,
    Desc,
    Field(Values),
}

/// Helper for create name alias
#[derive(Debug, Clone)]
pub struct Alias(String);

/// Null Alias
#[derive(Default, Debug, Copy, Clone)]
pub struct NullAlias;

/// SQL Keywords
#[derive(Debug, Clone)]
pub enum Keyword {
    Null,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Custom(DynIden),
}

/// Like Expression
#[derive(Debug, Clone)]
pub struct LikeExpr {
    pub(crate) pattern: String,
    pub(crate) escape: Option<char>,
}

pub trait IntoLikeExpr {
    fn into_like_expr(self) -> LikeExpr;
}

/// SubQuery operators
#[derive(Debug, Copy, Clone)]
pub enum SubQueryOper {
    Exists,
    Any,
    Some,
    All,
}

// Impl begins

impl<T: 'static> IntoIden for T
where
    T: Iden,
{
    fn into_iden(self) -> DynIden {
        SeaRc::new(self)
    }
}

impl IntoIden for DynIden {
    fn into_iden(self) -> DynIden {
        self
    }
}

impl<I> IdenList for I
where
    I: IntoIden,
{
    type IntoIter = std::iter::Once<DynIden>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.into_iden())
    }
}

impl<A, B> IdenList for (A, B)
where
    A: IntoIden,
    B: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden()].into_iter()
    }
}

impl<A, B, C> IdenList for (A, B, C)
where
    A: IntoIden,
    B: IntoIden,
    C: IntoIden,
{
    type IntoIter = std::array::IntoIter<DynIden, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0.into_iden(), self.1.into_iden(), self.2.into_iden()].into_iter()
    }
}

impl IntoIdentity for String {
    fn into_identity(self) -> Identity {
        self.as_str().into_identity()
    }
}

impl IntoIdentity for &str {
    fn into_identity(self) -> Identity {
        Identity::Unary(SeaRc::new(Alias::new(self)))
    }
}

impl<T> IntoIdentity for T
where
    T: IdenStatic,
{
    fn into_identity(self) -> Identity {
        Identity::Unary(self.into_iden())
    }
}

impl<T, C> IntoIdentity for (T, C)
where
    T: IdenStatic,
    C: IdenStatic,
{
    fn into_identity(self) -> Identity {
        Identity::Binary(self.0.into_iden(), self.1.into_iden())
    }
}

impl<T, C, R> IntoIdentity for (T, C, R)
where
    T: IdenStatic,
    C: IdenStatic,
    R: IdenStatic,
{
    fn into_identity(self) -> Identity {
        Identity::Ternary(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

impl IntoColumnRef for ColumnRef {
    fn into_column_ref(self) -> ColumnRef {
        self
    }
}

impl<T: 'static> IntoColumnRef for T
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Column(self.into_iden())
    }
}

impl<S: 'static, T: 'static> IntoColumnRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::TableColumn(self.0.into_iden(), self.1.into_iden())
    }
}

impl<S: 'static, T: 'static, U: 'static> IntoColumnRef for (S, T, U)
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::SchemaTableColumn(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

impl IntoTableRef for TableRef {
    fn into_table_ref(self) -> TableRef {
        self
    }
}

impl<T: 'static> IntoTableRef for T
where
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::Table(self.into_iden())
    }
}

impl<S: 'static, T: 'static> IntoTableRef for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::SchemaTable(self.0.into_iden(), self.1.into_iden())
    }
}

impl<S: 'static, T: 'static, U: 'static> IntoTableRef for (S, T, U)
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::DatabaseSchemaTable(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

impl TableRef {
    /// Add or replace the current alias
    pub fn alias<A>(self, alias: A) -> Self
    where
        A: IntoIden,
    {
        match self {
            Self::Table(table) => Self::TableAlias(table, alias.into_iden()),
            Self::TableAlias(table, _) => Self::TableAlias(table, alias.into_iden()),
            Self::SchemaTable(schema, table) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            Self::DatabaseSchemaTable(database, schema, table) => {
                Self::DatabaseSchemaTableAlias(database, schema, table, alias.into_iden())
            }
            Self::SchemaTableAlias(schema, table, _) => {
                Self::SchemaTableAlias(schema, table, alias.into_iden())
            }
            Self::DatabaseSchemaTableAlias(database, schema, table, _) => {
                Self::DatabaseSchemaTableAlias(database, schema, table, alias.into_iden())
            }
            Self::SubQuery(statement, _) => Self::SubQuery(statement, alias.into_iden()),
            Self::ValuesList(values, _) => Self::ValuesList(values, alias.into_iden()),
            Self::FunctionCall(func, _) => Self::FunctionCall(func, alias.into_iden()),
        }
    }
}

impl Alias {
    pub fn new(n: &str) -> Self {
        Self(n.to_owned())
    }
}

impl Iden for Alias {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

impl NullAlias {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iden for NullAlias {
    fn unquoted(&self, _s: &mut dyn fmt::Write) {}
}

impl LikeExpr {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            escape: None,
        }
    }

    pub fn str(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_owned(),
            escape: None,
        }
    }

    pub fn escape(self, c: char) -> Self {
        Self {
            pattern: self.pattern,
            escape: Some(c),
        }
    }
}

impl IntoLikeExpr for LikeExpr {
    fn into_like_expr(self) -> LikeExpr {
        self
    }
}

impl IntoLikeExpr for &str {
    fn into_like_expr(self) -> LikeExpr {
        LikeExpr::str(self)
    }
}

impl IntoLikeExpr for String {
    fn into_like_expr(self) -> LikeExpr {
        LikeExpr::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_identifier() {
        let query = Query::select()
            .column(Alias::new("hello-World_"))
            .to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(
            query.to_string(MysqlQueryBuilder),
            r#"SELECT `hello-World_`"#
        );
        #[cfg(feature = "backend-postgres")]
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            r#"SELECT "hello-World_""#
        );
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            r#"SELECT "hello-World_""#
        );
    }

    #[test]
    fn test_quoted_identifier_1() {
        let query = Query::select().column(Alias::new("hel`lo")).to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT `hel``lo`"#);
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel`lo""#);

        let query = Query::select().column(Alias::new("hel\"lo")).to_owned();

        #[cfg(feature = "backend-postgres")]
        assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT "hel""lo""#);
    }

    #[test]
    fn test_quoted_identifier_2() {
        let query = Query::select().column(Alias::new("hel``lo")).to_owned();

        #[cfg(feature = "backend-mysql")]
        assert_eq!(query.to_string(MysqlQueryBuilder), r#"SELECT `hel````lo`"#);
        #[cfg(feature = "backend-sqlite")]
        assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel``lo""#);

        let query = Query::select().column(Alias::new("hel\"\"lo")).to_owned();

        #[cfg(feature = "backend-postgres")]
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            r#"SELECT "hel""""lo""#
        );
    }
}
