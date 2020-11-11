use std::sync::Arc;
use swc_common::comments::SingleThreadedComments;
use swc_common::FileName;
use swc_common::Globals;
use swc_common::SourceMap;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::JscTarget;
use swc_ecma_parser::Parser;
use swc_ecma_parser::StringInput;
use swc_ecma_parser::{Syntax, TsConfig};

pub fn parse(file_name: &str, source: &str) -> swc_ecma_ast::Module {
    let source_map = Arc::new(SourceMap::default());
    swc_common::GLOBALS.set(&Globals::new(), || {
        let swc_source_file =
            source_map.new_source_file(FileName::Custom(file_name.to_string()), source.to_string());

        let mut ts_config = TsConfig::default();
        ts_config.dynamic_import = true;
        let syntax = Syntax::Typescript(ts_config);
        let comments = SingleThreadedComments::default();
        let lexer = Lexer::new(
            syntax,
            JscTarget::Es2019,
            StringInput::from(&*swc_source_file),
            Some(&comments),
        );

        let mut parser = Parser::new_from(lexer);
        parser.parse_module().expect("Unable to parse source file")
    })
}
