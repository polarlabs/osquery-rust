use crate::plugin::table::ColumnType;
use std::collections::HashMap;

// QueryConstraints contains the constraints from the WHERE clause of the query,
// that can optionally be used to optimize the table generation. Note that the
// _osquery SQLite engine will perform the filtering with these constraints, so
// it is not mandatory that they be used in table generation.
// QueryConstraints is a map from column name to the details of the
// constraints on that column.
pub type QueryConstraints = HashMap<String, ConstraintList>;

// ConstraintList contains the details of the constraints for the given column.
#[allow(dead_code)]
pub struct ConstraintList {
    affinity: ColumnType,
    constraints: Vec<Constraint>,
}

// Constraint contains both an operator and an expression that are applied as
// constraints in the query.
#[allow(dead_code)]
struct Constraint {
    op: Operator,
    expr: String,
}

#[allow(dead_code)]
enum Operator {
    // 1
    Unique,
    // 2
    Equals,
    // 4
    GreaterThan,
    // 8
    LessThanOrEquals,
    // 16
    LessThan,
    // 32
    GreaterThanOrEquals,
    // 64
    Match,
    // 65
    Like,
    // 66
    Glob,
    // 67
    Regexp,
}
