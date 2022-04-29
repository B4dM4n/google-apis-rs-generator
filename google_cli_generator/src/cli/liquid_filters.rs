use liquid_core::{Display_filter, Filter, FilterReflection, Result, Runtime, Value, ValueView};
use liquid_derive::ParseFilter;

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "rust_string_literal",
    description = "make any string printable as a Rust string",
    parsed(RustStringLiteralFilter)
)]
pub struct RustStringLiteral;

#[derive(Debug, Default, Display_filter)]
#[name = "rust_string_literal"]
struct RustStringLiteralFilter;

impl Filter for RustStringLiteralFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        Ok(Value::scalar(format!("{:?}", input.to_kstr())))
    }
}
