use crate::parser::parse;
use swc_ecma_ast::{CallExpr, ExportAll, ImportDecl, NamedExport};
use swc_ecma_visit::Node;
use swc_ecma_visit::Visit;

struct DependencyVisitor {
    dependencies: Vec<String>,
    analyze_dynamic_imports: bool,
}

impl Visit for DependencyVisitor {
    fn visit_import_decl(&mut self, import_decl: &ImportDecl, _parent: &dyn Node) {
        let src_str = import_decl.src.value.to_string();
        self.dependencies.push(src_str);
    }

    fn visit_named_export(&mut self, named_export: &NamedExport, _parent: &dyn Node) {
        if let Some(src) = &named_export.src {
            let src_str = src.value.to_string();
            self.dependencies.push(src_str);
        }
    }

    fn visit_export_all(&mut self, export_all: &ExportAll, _parent: &dyn Node) {
        let src_str = export_all.src.value.to_string();
        self.dependencies.push(src_str);
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr, _parent: &dyn Node) {
        use swc_ecma_ast::{Expr::*, ExprOrSpread, ExprOrSuper::*, Ident, Lit::Str};
        if !self.analyze_dynamic_imports {
            return;
        }

        match call_expr.callee.clone() {
            Expr(box Ident(Ident { sym, .. })) =>
            {
                #[allow(clippy::cmp_owned)]
                if sym.to_string() != "import" {
                    return;
                }
            }
            _ => return,
        };

        if let Some(ExprOrSpread {
            expr: box Lit(Str(src)),
            ..
        }) = call_expr.args.get(0)
        {
            self.dependencies.push(src.value.to_string());
        }
    }
}

// Analyze the dependencies of a source file
pub fn analyze_dependencies(filename: &str, source_code: &str) -> Vec<String> {
    let parse_result = parse(filename, source_code);
    let mut collector = DependencyVisitor {
        dependencies: vec![],
        analyze_dynamic_imports: true,
    };
    collector.visit_module(&parse_result, &parse_result);
    collector.dependencies
}
