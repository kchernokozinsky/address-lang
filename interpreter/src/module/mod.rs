pub mod loader;

use parser::ast::*;
use std::collections::HashMap;

pub struct Module {
    name: String,
    path: Path,
    lines: Vec<FileLine>,
    labels: HashMap<String, usize>,
}
