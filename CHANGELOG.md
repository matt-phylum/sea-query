# Changelog

All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.29.0 - Pending

### New Features

* Added `ValueTuple::Many` for tuple with length up to 12 https://github.com/SeaQL/sea-query/pull/564
* Added create Table `CHECK` Constraints https://github.com/SeaQL/sea-query/pull/567

### Breaking changes

* Removed `Expr::tbl`, `Expr::greater_than`, `Expr::greater_or_equal`, `Expr::less_than`, `Expr::less_or_equal`, `Expr::into_simple_expr` https://github.com/SeaQL/sea-query/pull/551
* Removed `SimpleExpr::equals` and `SimpleExpr::not_equals` https://github.com/SeaQL/sea-query/pull/551
* Removed `InsertStatement::exprs`, `InsertStatement::exprs_panic` https://github.com/SeaQL/sea-query/pull/551
* Removed `OnConflict::update_value`, `OnConflict::update_values`, `OnConflict::update_expr`, `OnConflict::update_exprs` https://github.com/SeaQL/sea-query/pull/551
* Removed `UpdateStatement::exprs`, `UpdateStatement::col_expr`, `UpdateStatement::value_expr` https://github.com/SeaQL/sea-query/pull/551
* `BigInteger` now maps to `bigint` instead of `integer` on SQLite https://github.com/SeaQL/sea-query/pull/556
* Fixed Postgres `GEN_RANDOM_UUID` https://github.com/SeaQL/sea-query/issues/568
  - `PgFunction::GetRandomUUID` -> `PgFunction::GenRandomUUID`
  - `PgFunc::get_random_uuid` -> `PgFunc::gen_random_uuid`

### House keeping

* Elided unnecessary lifetimes https://github.com/SeaQL/sea-query/pull/552
* Changed all `version = "^x.y.z"` into `version = "x.y.z"` in all Cargo.toml https://github.com/SeaQL/sea-query/pull/547/
* Disabled default features and enable only the needed ones https://github.com/SeaQL/sea-query/pull/547/

## 0.28.2 - 2023-01-04

### Enhancements

* Added `Cow<str>` conversion to `Value` https://github.com/SeaQL/sea-query/pull/550
* Added convert various `UUID` defined in `uuid::fmt` module into `sea_query::Value::Uuid` https://github.com/SeaQL/sea-query/pull/563

## 0.28.1 - 2022-12-29

### Bug fixes

* Fixes Postgres `GEN_RANDOM_UUID` https://github.com/SeaQL/sea-query/issues/568

## 0.28.0 - 2022-12-09

### New Features

* New struct `FunctionCall` which hold function and arguments https://github.com/SeaQL/sea-query/pull/475
* New trait `IdenStatic` with method `fn as_str(&self) -> &'static str` https://github.com/SeaQL/sea-query/pull/508
* New traits `PgExpr` and `SqliteExpr` for custom expressions https://github.com/SeaQL/sea-query/pull/519
* Support `BigDecimal`, `IpNetwork` and `MacAddress` for `sea-query-postgres` https://github.com/SeaQL/sea-query/pull/503

### API Additions

* Added `SelectStatement::from_function` https://github.com/SeaQL/sea-query/pull/475
* Added binary operators from the Postgres `pg_trgm` extension https://github.com/SeaQL/sea-query/pull/486
* Added `ILIKE` and `NOT ILIKE` operators https://github.com/SeaQL/sea-query/pull/473
* Added the `mul` and `div` methods for `SimpleExpr` https://github.com/SeaQL/sea-query/pull/510
* Added the `MATCH`, `->` and `->>` operators for SQLite https://github.com/SeaQL/sea-query/pull/513
* Added the `FULL OUTER JOIN` https://github.com/SeaQL/sea-query/pull/497
* Added `PgFunc::get_random_uuid` https://github.com/SeaQL/sea-query/pull/530
* Added `SimpleExpr::eq`, `SimpleExpr::ne`, `Expr::not_equals` https://github.com/SeaQL/sea-query/pull/528
* Added `PgFunc::starts_with` https://github.com/SeaQL/sea-query/pull/529
* Added `Expr::custom_keyword` and `SimpleExpr::not` https://github.com/SeaQL/sea-query/pull/535
* Added `SimpleExpr::like`, `SimpleExpr::not_like` and `Expr::cast_as` https://github.com/SeaQL/sea-query/pull/539
* Added support for `NULLS NOT DISTINCT` clause for Postgres https://github.com/SeaQL/sea-query/pull/532
* Added `Expr::cust_with_expr` and `Expr::cust_with_exprs` https://github.com/SeaQL/sea-query/pull/531
* Added support for converting `&String` to Value https://github.com/SeaQL/sea-query/issues/537

### Enhancements

* Made `value::with_array` module public and therefore making `NotU8` trait public https://github.com/SeaQL/sea-query/pull/511
* Drop the `Sized` requirement on implementers of `SchemaBuilders` https://github.com/SeaQL/sea-query/pull/524

### Bug fixes

* Wrap unions into parenthesis https://github.com/SeaQL/sea-query/pull/498
* Syntax error on empty condition https://github.com/SeaQL/sea-query/pull/505
```rust

// given
let (statement, values) = sea_query::Query::select()
    .column(Glyph::Id)
    .from(Glyph::Table)
    .cond_where(Cond::any()
        .add(Cond::all()) // empty all() => TRUE
        .add(Cond::any()) // empty any() => FALSE
    )
    .build(sea_query::MysqlQueryBuilder);

// old behavior
assert_eq!(statement, r#"SELECT `id` FROM `glyph`"#);

// new behavior
assert_eq!(
    statement,
    r#"SELECT `id` FROM `glyph` WHERE (TRUE) OR (FALSE)"#
);
```

### Breaking changes

* MSRV is up to 1.62 https://github.com/SeaQL/sea-query/pull/535
* `ColumnType::Array` definition changed from `Array(SeaRc<Box<ColumnType>>)` to `Array(SeaRc<ColumnType>)` https://github.com/SeaQL/sea-query/pull/492
* `Func::*` now returns `FunctionCall` instead of `SimpleExpr` https://github.com/SeaQL/sea-query/pull/475
* `Func::coalesce` now accepts `IntoIterator<Item = SimpleExpr>` instead of `IntoIterator<Item = Into<SimpleExpr>` https://github.com/SeaQL/sea-query/pull/475
* Removed `Expr::arg` and `Expr::args` - these functions are no longer needed https://github.com/SeaQL/sea-query/pull/475
* Moved all Postgres specific operators to `PgBinOper` https://github.com/SeaQL/sea-query/pull/507
* `Expr::value`, `Expr::gt`, `Expr::gte`, `Expr::lt`, `Expr::lte`, `Expr::add`, `Expr::div`, `Expr::sub`, `Expr::modulo`, `Expr::left_shift`, `Expr::right_shift`, `Expr::between`, `Expr::not_between`, `Expr::is`, `Expr::is_not`, `Expr::if_null` now accepts `Into<SimpleExpr>` instead of `Into<Value>` https://github.com/SeaQL/sea-query/pull/476
* `Expr::is_in`, `Expr::is_not_in` now accepts `Into<SimpleExpr>` instead of `Into<Value>` and convert it to `SimpleExpr::Tuple` instead of `SimpleExpr::Values` https://github.com/SeaQL/sea-query/pull/476
* `Expr::expr` now accepts `Into<SimpleExpr>` instead of `SimpleExpr` https://github.com/SeaQL/sea-query/pull/475
* Moved `Expr::ilike`, `Expr::not_ilike`, `Expr::matches`, `Expr::contains`, `Expr::contained`, `Expr::concatenate`, `Expr::concat`, `SimpleExpr::concatenate` and `SimpleExpr::concat` to new trait `PgExpr` https://github.com/SeaQL/sea-query/pull/519
* `Expr::equals` now accepts `C: IntoColumnRef` instead of `T: IntoIden, C: IntoIden` https://github.com/SeaQL/sea-query/pull/528
* Removed integer and date time column types' display length / precision option https://github.com/SeaQL/sea-query/pull/525

### Deprecations

* Deprecated `Expr::greater_than`, `Expr::greater_or_equal`, `Expr::less_than` and `Expr::less_or_equal` https://github.com/SeaQL/sea-query/pull/476
* Deprecated `SimpleExpr::equals`, `SimpleExpr::not_equals` https://github.com/SeaQL/sea-query/pull/528
* Deprecated `Expr::tbl`, please use `Expr::col` with a tuple https://github.com/SeaQL/sea-query/pull/540

### House keeping

* Replace `impl Default` with `#[derive(Default)]` https://github.com/SeaQL/sea-query/pull/535
* Exclude `sqlx` default features https://github.com/SeaQL/sea-query/pull/543
* Use `dtolnay/rust-toolchain` instead of `actions-rs/toolchain` in `CI` https://github.com/SeaQL/sea-query/pull/544

## 0.27.2 - 2022-11-14

* Made `value::with_array` module public and therefore making `NotU8` trait public https://github.com/SeaQL/sea-query/pull/511

## sea-query-binder 0.2.2

* Enable SQLx features only if SQLx optional dependency is enabled https://github.com/SeaQL/sea-query/pull/517

## 0.27.1 - 2022-10-18

* Fix consecutive spacing on schema statements https://github.com/SeaQL/sea-query/pull/481
* SQLite bind `rust_decimal` & `bigdecimal` as f64 https://github.com/SeaQL/sea-query/pull/480

## 0.27.0 - 2022-10-16

### New Features

* Support `CROSS JOIN` https://github.com/SeaQL/sea-query/pull/376
* We are going through series of changes to how database drivers work
(https://github.com/SeaQL/sea-query/pull/416, https://github.com/SeaQL/sea-query/pull/423):
	1. `sea-query-binder` is now the recommended way (trait based) of working with SQLx, replacing `sea-query-driver` (macro based) https://github.com/SeaQL/sea-query/pull/434
	2. `sea-query-binder` is now a separate dependency, instead of integrated with `sea-query` https://github.com/SeaQL/sea-query/pull/432
	3. `rusqlite` support is moved to `sea-query-rusqlite` https://github.com/SeaQL/sea-query/pull/422
	4. `postgres` support is moved to `sea-query-postgres` https://github.com/SeaQL/sea-query/pull/433
* Added sub-query operators: `EXISTS`, `ALL`, `ANY`, `SOME` https://github.com/SeaQL/sea-query/pull/379
* Added support to `ON CONFLICT WHERE` https://github.com/SeaQL/sea-query/pull/447
* Added support `DROP COLUMN` for SQLite https://github.com/SeaQL/sea-query/pull/455
* Added `YEAR`, `BIT` and `VARBIT` types https://github.com/SeaQL/sea-query/pull/466
* Added support one dimension Postgres array for SQLx https://github.com/SeaQL/sea-query/pull/467

### Enhancements

* Handle Postgres schema name for schema statements https://github.com/SeaQL/sea-query/pull/385
* Added `%`, `<<` and `>>` binary operators https://github.com/SeaQL/sea-query/pull/419
* Added `RAND` function https://github.com/SeaQL/sea-query/pull/430
* Implements `Display` for `Value` https://github.com/SeaQL/sea-query/pull/425
* Added `INTERSECT` and `EXCEPT` to `UnionType` https://github.com/SeaQL/sea-query/pull/438
* Added `OnConflict::value` and `OnConflict::values` https://github.com/SeaQL/sea-query/issues/451
* `ColumnDef::default` now accepts both `Value` and `SimpleExpr` https://github.com/SeaQL/sea-query/pull/436
* `OrderedStatement::order_by_customs`, `OrderedStatement::order_by_columns`, `OverStatement::partition_by_customs`, `OverStatement::partition_by_columns` now accepts `IntoIterator<Item = T>` instead of `Vec<T>` https://github.com/SeaQL/sea-query/pull/448
* `Expr::case`, `CaseStatement::case` and `CaseStatement::finally` now accepts `Into<SimpleExpr>` instead of `Into<Expr>` https://github.com/SeaQL/sea-query/pull/460
* `UpdateStatement::value` now accept `Into<SimpleExpr>` instead of `Into<Value>` https://github.com/SeaQL/sea-query/pull/460
* `TableAlterStatement::rename_column`, `TableAlterStatement::drop_column`, `ColumnDef::new`, `ColumnDef::new_with_type` now accepts `IntoIden` instead of `Iden` https://github.com/SeaQL/sea-query/pull/472

### Bug Fixes

* `distinct_on` properly handles `ColumnRef` https://github.com/SeaQL/sea-query/pull/450
* Removed `ON` for `DROP INDEX` for SQLite https://github.com/SeaQL/sea-query/pull/462
* Change datetime string format to include microseconds https://github.com/SeaQL/sea-query/pull/468
* `ALTER TABLE` for PosgreSQL with `UNIQUE` constraint https://github.com/SeaQL/sea-query/pull/472

### Breaking changes

* Changed `in_tuples` interface to accept `IntoValueTuple` https://github.com/SeaQL/sea-query/pull/386
* Removed deprecated methods (`or_where`, `or_having`, `table_column` etc) https://github.com/SeaQL/sea-query/pull/380
* **Changed `cond_where` chaining semantics** https://github.com/SeaQL/sea-query/pull/417
```rust
// Before: will extend current Condition
assert_eq!(
    Query::select()
        .cond_where(any![Expr::col(Glyph::Id).eq(1), Expr::col(Glyph::Id).eq(2)])
        .cond_where(Expr::col(Glyph::Id).eq(3))
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE "id" = 1 OR "id" = 2 OR "id" = 3"#
);
// Before: confusing, since it depends on the order of invocation:
assert_eq!(
    Query::select()
        .cond_where(Expr::col(Glyph::Id).eq(3))
        .cond_where(any![Expr::col(Glyph::Id).eq(1), Expr::col(Glyph::Id).eq(2)])
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE "id" = 3 AND ("id" = 1 OR "id" = 2)"#
);
// Now: will always conjoin with `AND`
assert_eq!(
    Query::select()
        .cond_where(Expr::col(Glyph::Id).eq(1))
        .cond_where(any![Expr::col(Glyph::Id).eq(2), Expr::col(Glyph::Id).eq(3)])
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE "id" = 1 AND ("id" = 2 OR "id" = 3)"#
);
// Now: so they are now equivalent
assert_eq!(
    Query::select()
        .cond_where(any![Expr::col(Glyph::Id).eq(2), Expr::col(Glyph::Id).eq(3)])
        .cond_where(Expr::col(Glyph::Id).eq(1))
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE ("id" = 2 OR "id" = 3) AND "id" = 1"#
);
```
* `CURRENT_TIMESTAMP` changed from being a function to keyword https://github.com/SeaQL/sea-query/pull/441
* Update SQLite `boolean` type from `integer` to `boolean` https://github.com/SeaQL/sea-query/pull/400
* Changed type of `ColumnType::Enum` from `(String, Vec<String>)` to: https://github.com/SeaQL/sea-query/pull/435
```rust
Enum {
    name: DynIden,
    variants: Vec<DynIden>,
}
```
* Deprecated `InsertStatement::exprs`, `InsertStatement::exprs_panic`, `OnConflict::update_value`, `OnConflict::update_values`, `OnConflict::update_expr`, `OnConflict::update_exprs`, `UpdateStatement::col_expr`, `UpdateStatement::value_expr`, `UpdateStatement::exprs` https://github.com/SeaQL/sea-query/pull/460
* `InsertStatement::values`, `UpdateStatement::values` now accepts `IntoIterator<Item = SimpleExpr>` instead of `IntoIterator<Item = Value>` https://github.com/SeaQL/sea-query/pull/460
* Use native api from SQLx for SQLite to work with `time` https://github.com/SeaQL/sea-query/pull/412

### House keeping

* Cleanup `IndexBuilder` trait methods https://github.com/SeaQL/sea-query/pull/426
* Introduce `SqlWriter` trait https://github.com/SeaQL/sea-query/pull/436
* Remove unneeded `vec!` from examples https://github.com/SeaQL/sea-query/pull/448

### Upgrades

* Upgrade `sqlx` driver to 0.6.1

## 0.26.4 - 2022-10-13

### New Features

* Added support `DROP COLUMN` for SQLite https://github.com/SeaQL/sea-query/pull/455 

### Bug Fixes

* Removed `ON` for `DROP INDEX` for SQLite https://github.com/SeaQL/sea-query/pull/462
* Changed datetime display format to include microseconds https://github.com/SeaQL/sea-query/pull/468

## 0.26.3 - 2022-08-18

### Bug Fixes

* `DROP NOT NULL` for Postgres `ALTER COLUMN` https://github.com/SeaQL/sea-query/pull/394

### House keeping

* Exclude `chrono` default-features https://github.com/SeaQL/sea-query/pull/410
* Fix clippy warnings https://github.com/SeaQL/sea-query/pull/415

## 0.26.2 - 2022-07-21

### Bug Fixes

* Rename `postgres-*` features to `with-*` on `postgres` driver https://github.com/SeaQL/sea-query/pull/377

## 0.26.0 - 2022-07-02

### New Features

* Add support for `VALUES` lists https://github.com/SeaQL/sea-query/pull/351
* Introduce `sea-query-binder` https://github.com/SeaQL/sea-query/pull/275
* Convert from `IpNetwork` and `MacAddress` to `Value` https://github.com/SeaQL/sea-query/pull/364

### Enhancements

* Move `escape` and `unescape` string to backend https://github.com/SeaQL/sea-query/pull/306
* `LIKE ESCAPE` support https://github.com/SeaQL/sea-query/pull/352 #353)
* `clear_order_by` for `OrderedStatement`
* Add method to make a column nullable https://github.com/SeaQL/sea-query/pull/365
* Add `is` & `is_not` to Expr https://github.com/SeaQL/sea-query/pull/348
* Add `CURRENT_TIMESTAMP` function https://github.com/SeaQL/sea-query/pull/349
* Add `in_tuples` method to Expr https://github.com/SeaQL/sea-query/pull/345

### Upgrades

* Upgrade `uuid` to 1.0
* Upgrade `time` to 0.3
* Upgrade `ipnetwork` to 0.19
* Upgrade `bigdecimal` to 0.3
* Upgrade `sqlx` driver to 0.6

### Breaking changes

* As part of #306, the standalone functions `escape_string` and `unescape_string` are removed, and becomes backend specific. So now, you have to:

```rust
use sea_query::EscapeBuilder;

let string: String = MySqlQueryBuilder.escape_string(r#" "abc" "#);
let string: String = MysqlQueryBuilder.unescape_string(r#" \"abc\" "#);
```

* Replace `Value::Ipv4Network` and `Value::Ipv6Network`  to `Value::IpNetwork` https://github.com/SeaQL/sea-query/pull/364

* Remove some redundant feature flags `postgres-chrono`, `postgres-json`, `postgres-uuid`, `postgres-time`. Use the `with-*` equivalence

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.25.0...0.26.0

## 0.25.2 - 2022-07-01

### New features

* Introduce `sea-query-binder` https://github.com/SeaQL/sea-query/pull/275

### Enhancements

* Add method to make a column nullable https://github.com/SeaQL/sea-query/pull/365
* Add `is` & `is_not` to Expr https://github.com/SeaQL/sea-query/pull/348
* Add `CURRENT_TIMESTAMP` function https://github.com/SeaQL/sea-query/pull/349

## 0.25.1 - 2022-06-26

### Enhancements

* `clear_order_by` for `OrderedStatement`

## 0.25.0 - 2022-05-28

### New Features

* CASE WHEN statement support https://github.com/SeaQL/sea-query/pull/304
* Add support for Ip(4,6)Network and MacAddress https://github.com/SeaQL/sea-query/pull/309
* [sea-query-attr] macro for deriving `Iden` enum from struct https://github.com/SeaQL/sea-query/pull/300
* Add ability to alter foreign keys https://github.com/SeaQL/sea-query/pull/299
* Select `DISTINCT ON` https://github.com/SeaQL/sea-query/pull/313

### Enhancements

* Insert Default https://github.com/SeaQL/sea-query/pull/266
* Make `sea-query-driver` an optional dependency https://github.com/SeaQL/sea-query/pull/324
* Add `ABS` function https://github.com/SeaQL/sea-query/pull/334
* Support `IF NOT EXISTS` when create index https://github.com/SeaQL/sea-query/pull/332
* Support different `blob` types in MySQL https://github.com/SeaQL/sea-query/pull/314
* Add `VarBinary` column type https://github.com/SeaQL/sea-query/pull/331
* Returning expression supporting `SimpleExpr` https://github.com/SeaQL/sea-query/pull/335

### Bug fixes

* Fix arguments when nesting custom expressions https://github.com/SeaQL/sea-query/pull/333
* Fix clippy warnings for manual map https://github.com/SeaQL/sea-query/pull/337

### Breaking Changes

* Introducing a dedicated `ReturningClause` instead of reusing `Select` on `returning`: https://github.com/SeaQL/sea-query/pull/317

```rust
.returning(Query::select().column(Glyph::Id).take()) // before
.returning(Query::returning().columns([Glyph::Id])) // now
```

* In #333, the custom expression API changed for Postgres, users should change their placeholder from `?` to Postgres's `$N`

```rust
let query = Query::select()
    .columns([Char::Character, Char::SizeW, Char::SizeH])
    .from(Char::Table)
    .and_where(Expr::col(Char::Id).eq(1))
    .and_where(Expr::cust_with_values("6 = $2 * $1", vec![3, 2]).into())
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND 6 = 2 * 3"#
);
```

As a side effect, `??` is no longer needed for escaping `?`

```rust
let query = Query::select()
    .expr(Expr::cust_with_values(
        "data @? ($1::JSONPATH)",
        vec!["hello"],
    ))
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT data @? ('hello'::JSONPATH)"#
);
```

* In #314, `ColumnType`'s `Binary(Option<u32>)` changed to `Binary(BlobSize)`, so if you used `Binary(None)` before, you should change to `Binary(BlobSize::Blob(None))`

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.24.0...0.25.0

## 0.24.6 - 2022-05-12

* Make `sea-query-driver` an optional dependency https://github.com/SeaQL/sea-query/pull/324

## 0.24.5 - 2022-05-09

* Insert `or_default_values` https://github.com/SeaQL/sea-query/pull/266

## 0.24.4 - 2022-04-26

* update sea-query-driver

## 0.24.3 - 2022-04-26

### Bug fixes

* Fix MySQL index create statement https://github.com/SeaQL/sea-query/pull/308

### Enhancements

* Add length check on condition array https://github.com/SeaQL/sea-query/pull/307

## 0.24.2 - 2022-04-18

### Bug fixes

* Fixed https://github.com/SeaQL/sea-query/issues/303 driver breakage in 0.24.0

Notes: 0.24.0 & 0.24.1 were yanked

## 0.24.1 - 2022-04-15

### Enhancements

* #295 Add parameter for SQLx path to proc-macro https://github.com/SeaQL/sea-query/pull/297

### Bug fixes

* CTE optional columns https://github.com/SeaQL/sea-query/pull/301

## 0.24.0 - 2022-04-05

### New Features

* Add `LOWER` and `UPPER` func https://github.com/SeaQL/sea-query/pull/276
* Insert `ON CONFLICT` support https://github.com/SeaQL/sea-query/pull/279
* #174 Add support for `WINDOWS` statement https://github.com/SeaQL/sea-query/pull/271
* #142 full support lock in select https://github.com/SeaQL/sea-query/pull/289
* #269 add support for postgres `ANY`, `SOME`, `ALL` https://github.com/SeaQL/sea-query/pull/283

### Enhancements

* Add support for multiple `ALTER` operations https://github.com/SeaQL/sea-query/pull/277
* #229 add column if not exists https://github.com/SeaQL/sea-query/pull/278
* #255 Add support to CommonTableExpression columns method https://github.com/SeaQL/sea-query/pull/284
* #280 Rewrite drivers using proc-macro https://github.com/SeaQL/sea-query/pull/292

### Bug fixes

* #285 Fix timestamp_with_time_zone_len https://github.com/SeaQL/sea-query/pull/286

### Breaking changes

* The enum variants for `LockType` were renamed: `Exclusive` -> `Update` and `Shared` -> `Share`
* As part of #283, the drivers are split to the `sea-query-driver` crate
    1. Remove methods `Value::is_json` and `Value::as_ref_json` when feature: **with-json** is disabled
    2. Remove methods `Value::is_time_*` and `Value::as_ref_time_*` when feature: **with-time** is disabled
    3. Remove methods `Value::is_chrono_*` and `Value::as_ref_chrono*` when feature: **with-chrono** is disabled
    4. Remove methods `Value::is_decimal`, `Value::as_ref_decimal` and `Value::decimal_to_f64` when feature: **with-rust_decimal** is disabled
    5. Remove methods `Value::is_big_decimal`, `Value::as_ref_big_decimal` and `Value::big_decimal_to_f64` when feature: **with-bigdecimal** is disabled
    6. Remove methods `Value::is_uuid` and `Value::as_ref_uuid` when feature: **with-uuid** is disabled
    7. Remove methods `Value::is_array` and `Value::as_ref_array` when feature: **postgres-array** is disabled

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.23.0...0.24.0

## 0.23.0 - 2022-03-15

### New Features

* Supports `time` in addition to `chrono` https://github.com/SeaQL/sea-query/pull/267

### Enhancements

* Allow for trailing commas in any and all macros https://github.com/SeaQL/sea-query/pull/270

### Bug fixes

* Fix UNIQUE table index expression syntax for sqlite https://github.com/SeaQL/sea-query/pull/227

### Breaking changes

In order to co-exist with the `time` crate, `Date`, `Time`, `DateTime` etc are renamed to `ChronoDate`, `ChronoTime`, `ChronoDateTime`. In addition, new variants `TimeDate`, `TimeTime`, `TimeDateTime` and so on are introduced to `Value`.

## 0.22.0 - 2022-02-26

### New Features

* Support multiple tables in the select from by @Sytten in https://github.com/SeaQL/sea-query/pull/261
* Add support for replace insert by @Sytten in https://github.com/SeaQL/sea-query/pull/262
* Add `ColumnType` unsigned integer types by @billy1624 in https://github.com/SeaQL/sea-query/pull/211

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.21.0...0.22.0

## 0.21.0 - 2022-02-01

### Breaking Changes

* Use double quotes for quoting identifiers for SQLite by @SpyrosRoum in https://github.com/SeaQL/sea-query/pull/221

### New Features

* Implement `RETURNING` for SQLite by @SpyrosRoum in https://github.com/SeaQL/sea-query/pull/194
* Support 'NULLS LAST' and 'NULLS FIRST' by @qyihua in https://github.com/SeaQL/sea-query/pull/210
* [join-lateral]  by @rex-remind101 in https://github.com/SeaQL/sea-query/pull/224
* Insert from select by @05storm26 in https://github.com/SeaQL/sea-query/pull/238
* Add Expr::asterisk() and Expr::tbl_asterisk(table: DynIden) methods - Fix #217 by @RomainMazB in https://github.com/SeaQL/sea-query/pull/219

### Enhancements

* Implement ToTokens for IntervalField by @autarch in https://github.com/SeaQL/sea-query/pull/195
* Implemented 'Array' type for Postgres. by @kev0960 in https://github.com/SeaQL/sea-query/pull/205
* Add `Value::DateTimeLocal` by @billy1624 in https://github.com/SeaQL/sea-query/pull/249
* Add `ColumnRef::SchemaTableColumn` by @billy1624 in https://github.com/SeaQL/sea-query/pull/206
* Datetime utc by @tyt2y3 in https://github.com/SeaQL/sea-query/pull/241
* Support the use of chrono::DateTime<Utc> using the type alias DateTim… by @charleschege in https://github.com/SeaQL/sea-query/pull/222

### Bug Fixes

* Fix PostgreSQL `ColumnType::TinyInteger` mapping by @billy1624 in https://github.com/SeaQL/sea-query/pull/207
* PR without clippy warmings in file changed tab by @billy1624 in https://github.com/SeaQL/sea-query/pull/212

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.20.0...0.21.0

## 0.20.0 - 2021-12-11

### Merged PRs

* Add `TableRef::DatabaseSchemaTable` by @billy1624 in https://github.com/SeaQL/sea-query/pull/193

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.4...0.20.0

## 0.19.4 - 2021-12-11

### Merged PRs

* Binding `DateTime<FixedOffset>` for SQLx MySQL & SQLite by @billy1624 in https://github.com/SeaQL/sea-query/pull/197

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.2...0.19.4

## 0.19.2 - 2021-12-04

### Merged PRs

* Impl `ValueTuple` Up to Six by @billy1624 in https://github.com/SeaQL/sea-query/pull/200
* Basic Benchmark by @tyt2y3 in https://github.com/SeaQL/sea-query/pull/192

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.1...0.19.2

## 0.19.1 - 2021-11-25

### Merged PRs
* `driver/postgres` handle non-exhaustive `Value` by @billy1624 in https://github.com/SeaQL/sea-query/pull/191

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.0...0.19.1

## 0.19.0 - 2021-11-19

### Merged PRs
* `TableCreateStatement` and `TableDropStatement` takes any `IntoTableRef` table name. by @josh-codes in https://github.com/SeaQL/sea-query/pull/186
* Add `ColumnType::Enum` by @billy1624 in https://github.com/SeaQL/sea-query/pull/188
* Update to Rust Edition 2021 by @billy1624 in https://github.com/SeaQL/sea-query/pull/189

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.18.2...0.19.0

## 0.18.2 - 2021-11-04

### Merged PRs
* Rename "where" keywords in `SelectStatement` to suppress IDEA warnings by @baoyachi in https://github.com/SeaQL/sea-query/pull/166
* Add binary method to expr by @Progdrasil in https://github.com/SeaQL/sea-query/pull/173
* Cast expression as custom type by @billy1624 in https://github.com/SeaQL/sea-query/pull/170
* Support tuple expression by @shuoli84 in https://github.com/SeaQL/sea-query/pull/178

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.18.1...0.18.2

## 0.18.1 - 2021-10-26

+ [[#169]] Add support for Postgres interval type
+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#169]: https://github.com/SeaQL/sea-query/pull/169
[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.18.0 - 2021-10-15

+ [[#159]] Add `ValueType::column_type`
+ [[#160]] Add `FromValueTuple` trait

[#159]: https://github.com/SeaQL/sea-query/pull/159
[#160]: https://github.com/SeaQL/sea-query/pull/160

## 0.17.3 - 2021-10-26

+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.17.2 - 2021-10-15

+ [[#164]] Revert "Fix SQLite `chrono::NaiveTime` binding"

[#164]: https://github.com/SeaQL/sea-query/pull/164

## 0.17.1 - 2021-10-12 (yanked)

## 0.17.0 - 2021-10-06

+ [[#157]] Fix binding nullable custom types on db drivers

The `as_ref_*` methods on `Value` are changed:

```rust
pub fn as_ref_json(&self) -> &Json;
```

Is now

```rust
pub fn as_ref_json(&self) -> Option<&Json>;
```

[#157]: https://github.com/SeaQL/sea-query/pull/157

## 0.16.6 - 2021-10-26

+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.16.5 - 2021-09-30

+ [[#145]] Add Condition::not
+ [[#149]] Fix SQLite `chrono::NaiveTime` binding

[#145]: https://github.com/SeaQL/sea-query/pull/145
[#149]: https://github.com/SeaQL/sea-query/pull/149

## 0.16.4 - 2021-09-26

+ Fix table drop options for SQLite
+ Add `IndexCreateStatement::is_unique_key()`

## 0.16.3 - 2021-09-17

+ [[#131]] `CAST AS` expression
+ [[#131]] `InsertStatement` accepts `SimpleExpr`
+ [[#137]] SQLx Postgres driver bind `DateTime<FixedOffset>`

[#131]: https://github.com/SeaQL/sea-query/issues/131
[#137]: https://github.com/SeaQL/sea-query/pull/137

## 0.16.2 - 2021-09-15

+ [[#120]] Support `RETURNING` for `DeleteStatement`
+ [[#128]] Support `UNION` clause for `SelectStatement`

[#120]: https://github.com/SeaQL/sea-query/issues/120
[#128]: https://github.com/SeaQL/sea-query/pull/128

## 0.16.1 - 2021-09-10

+ [[#129]] MySql `ColumnType::Binary(None)` maps to "blob"

[#129]: https://github.com/SeaQL/sea-query/pull/129

## 0.16.0 - 2021-09-02

+ [[#112]] Introduce `Nullable` trait to permit custom `Option<T>`
+ [[#113]] `ValueType` trait should have a non-panic-ing method
+ [[#114]] `ValueType` revamp

    1. Remove `ValueTypeDefault`
    1. Change `type_name` to return `String`

+ [[#115]] Postgres concatenate operator (`||`)
+ [[#117]] Lock support (`FOR SHARE`, `FOR UPDATE`) for SELECT statement

[#112]: https://github.com/SeaQL/sea-query/pull/112
[#113]: https://github.com/SeaQL/sea-query/pull/113
[#114]: https://github.com/SeaQL/sea-query/pull/114
[#115]: https://github.com/SeaQL/sea-query/pull/115
[#117]: https://github.com/SeaQL/sea-query/pull/117

## 0.15.0 - 2021-08-21

+ [[#107]] Revamp `Value` to typed null value
+ Added `BigDecimal` support

The `Value::Null` variant is removed. You have to use a specific variant with a `None`.

Before:

```rust
Query::insert()
    .values_panic(vec![
        Value::Null,
        2.1345.into(),
    ])
```

After:

```rust
Query::insert()
    .values_panic(vec![
        Value::String(None),
        2.1345.into(),
    ])
```

Since we cannot handle the generic `Null` value on JSON, we removed the `json` method on `InsertStatement` and `UpdateStatement`. The following NO LONGER WORKS:

```rust
let query = Query::insert()
    .into_table(Glyph::Table)
    .json(json!({
        "aspect": 2.1345,
        "image": "24B",
    }));
```

```rust
let query = Query::update()
    .table(Glyph::Table)
    .json(json!({
        "aspect": 2.1345,
        "image": "235m",
    }));
```

In addition, if you constructed `Value` manually before (instead of using `into()` which is unaffected), you have to wrap them in an `Option`:

Before:

```rust
let (sql, values) = query.build(PostgresQueryBuilder);
assert_eq!(
	values,
	Values(vec![Value::String(Box::new("A".to_owned())), Value::Int(1), Value::Int(2), Value::Int(3)]))
);
```

After:

```rust
let (sql, values) = query.build(PostgresQueryBuilder);
assert_eq!(
	values,
	Values(vec![Value::String(Some(Box::new("A".to_owned()))), Value::Int(Some(1)), Value::Int(Some(2)), Value::Int(Some(3))]))
);
```

[#107]: https://github.com/SeaQL/sea-query/pull/107

## 0.14.1 - 2021-08-15

+ [[#87]] Fix inconsistent Ownership of self in Builder APIs
+ [[#105]] Use Arc for SeaRc with feature flag thread-safe

[#87]: https://github.com/SeaQL/sea-query/pull/87
[#105]: https://github.com/SeaQL/sea-query/pull/105

## 0.12.12 - 2021-08-14

+ [[#98]] Support Postgres full text search

[#98]: https://github.com/SeaQL/sea-query/pull/98

## 0.12.11 - 2021-08-13

+ Support SeaORM

## 0.12.10 - 2021-08-11

+ [[#89]] flattening iden enums in derive macro

[#89]: https://github.com/SeaQL/sea-query/pull/87

## 0.12.9 - 2021-08-08

+ [[#77]] Postgres `binary` type
+ [[#81]] example for CockroachDB
+ [[#84]] Fix Postgres constraint keywords
+ [[#75]] `DateTimeWithTimeZone` value type and `TimestampWithTimeZone` column type

[#77]: https://github.com/SeaQL/sea-query/pull/77
[#81]: https://github.com/SeaQL/sea-query/pull/81
[#84]: https://github.com/SeaQL/sea-query/pull/84
[#75]: https://github.com/SeaQL/sea-query/pull/75

## 0.12.8 - 2021-07-24

+ Fix Postgres `datetime` column type mapping
+ `Uuid` in schema builder

## 0.12.7 - 2021-07-13

+ `cust_with_values` allow escape `?` using `??`

## 0.12.6 - 2021-07-07

+ Fixed build error for `sqlx-sqlite`

## 0.12.5 - 2021-07-07

+ Support `Decimal` from rust_decimal

## 0.12.4 - 2021-06-23

+ Added `returning` for update statement

## 0.12.3 - 2021-06-19

+ Added `type_name` for ValueType
+ `Values` derive `Clone`
+ Added `Update::col_expr`
+ Type def Rc as `SeaRc`
+ getters for schema statements

## 0.12.2 - 2021-06-04

+ Fixed `and_where_option`
+ Added `Condition::add_option`

## 0.12.1 - 2021-06-03

+ Added `not_in_subquery`

## 0.12.0 - 2021-05-31

+ Unify `cond_where` and `and_where`. Note: will panic if calling `or_where` after `and_where`.

## 0.11.1 - 2021-05-22

+ Updated Readme

## 0.11.0 - 2021-05-19

+ Added APIs to support ORM
+ Backend and internal refactoring
+ Introduced `QueryStatementBuilder` and `SchemaStatementBuilder` traits
+ Introduced `ConditionalStatement` and `OrderedStatement` traits
+ Introduced any/all style conditions for `cond_where` and `cond_having`

## 0.10.6 - 2021-05-04

+ Postgres `ALTER TYPE` statements for `ENUM`

## 0.10.5 - 2021-05-02

+ Updated documentation

## 0.10.4 - 2021-05-02

+ `returning()` expression for Postgres insert statements
+ Remove redundant index name in foreign key expression of MySQL

## 0.10.3 - 2021-04-30

+ custom `Error` type
+ Empty value list for IN
+ Index prefix and `IndexOrder`

## 0.10.2 - 2021-04-27

+ Foreign key API `from` and `to`
+ Fix foreign key bug in `ON UPDATE`

## 0.10.1 - 2021-04-25

+ Added `index_type()` (`FullText` and `Hash`)
+ Added `unique()` to `Index`
+ Support composite primary key

## 0.10.0 - 2021-04-23

+ Use `IntoIterator` trait instead of `Vec` on most APIs
+ UUID support in `Value`
+ Rusqlite support

## 0.9.6 - 2021-04-18

+ Rename `create_if_not_exists` to `if_not_exists`
+ Remove `partition_option` from `TableAlterStatement`
+ Added `ColumnDef::extra()`

## 0.9.5 - 2021-04-17

+ Added `SchemaStatement`

## 0.9.4 - 2021-04-13

+ Fixed `DateTime` quoting bugs

## 0.9.3 - 2021-04-08

+ Update sea-query-derive to 0.1.2

## 0.9.2 - 2021-04-05

+ derive supporting enum tuple variant and custom method

## 0.9.1 - 2021-03-30

+ updated docs

## 0.9.0 - 2021-03-29

+ Introduced `IntoColumnRef` trait to consolidate `column` and `table.column`
+ Introduced `IntoTableRef` trait to consolidate `table` and `schema.table`
+ Introduced `IntoIden` trait to remove `*_dyn` methods

## 0.8.5 - 2021-03-29

+ added `into_simple_expr()`

## 0.8.4 - 2021-03-24

+ Fixing `IS NULL`

## 0.8.3 - 2021-03-23

+ derive `Debug` on most structs

## 0.8.2 - 2021-03-23

+ Added `unescape_string`

## 0.8.1 - 2021-03-23

+ Improve documentation

## 0.8.0 - 2021-03-14

+ `json` support behind features
+ backend as features (`backend-mysql`, `backend-postgres`, `backend-sqlite`)
+ added `from_schema()`, `from_schema_as()`

## 0.7.0 - 2021-03-06

+ Revamp `Value`
+ `build()` API change
+ `postgres` driver support
+ `json` and `chrono` support

## 0.6.1 - 2021-03-05

+ Added `join_as`
+ Deprecated `expr_alias`, `from_alias`

## 0.6.0 - 2021-02-20

+ Custom expression with parameters `Expr::cust_with_values()`
+ Custom function call `Func::cust()`
+ Postgres enum `Type::create().as_enum()`

## 0.5.0 - 2021-02-09

+ derive macro `#[derive(Iden)]`

## 0.4.0 - 2021-02-02

+ Added JSON binary column type `ColumnDef::json_binary()`
+ Custom column type `ColumnDef::custom()`

## 0.3.0 - 2020-12-29



## 0.2.0 - 2020-12-26



## 0.1.0 - 2020-12-16

Publish to crate.io
