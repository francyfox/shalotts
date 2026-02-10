use std::path::Path;

use oxc::allocator::Allocator;
use oxc::parser::{ParseOptions, Parser};
use oxc::span::SourceType;
use oxc_codegen::Codegen;

pub fn fmt(path: &Path, source_text: &String) -> String {
    let Ok(source_type) = SourceType::from_path(path) else {
        return source_text.clone();
    };
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, source_text, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();
    let program = parser.program;

    Codegen::new().build(&program).code
}
