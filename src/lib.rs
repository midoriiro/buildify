mod constants;

mod generator;

mod field;
#[cfg(test)]
#[path = "./field_test.rs"]
mod field_test;

mod field_rule;
#[cfg(test)]
#[path = "./field_rule_test.rs"]
mod field_rule_test;

mod field_type_segment;

#[cfg(test)]
mod test_utils;
