use types::{CardContent, SchemaDiff};

pub trait Slicer {
    fn name(&self) -> &str;
    fn slice(&self, diff: &SchemaDiff) -> Vec<CardContent>;
}
