use syn::{visit::Visit, Expr, ExprCast, Type};

#[derive(Debug)]
pub struct AsVisitor {
    as_usages: Vec<String>,
}

impl AsVisitor {
    pub fn new(fn_name: &str, file: &syn::File) -> Self {
        let mut visitor = Self {
            as_usages: Vec::new(),
        };

        let function = file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Fn(item_fn) = item {
                    if item_fn.sig.ident.to_string() == fn_name {
                        return Some(item_fn);
                    }
                }

                None
            })
            .expect(&format!("Function {fn_name} not found"));

        visitor.visit_item_fn(function);

        visitor
    }

    pub fn usages(&self) -> &Vec<String> {
        &self.as_usages
    }
}

impl<'ast> Visit<'ast> for AsVisitor {
    fn visit_expr_cast(&mut self, expr_cast: &'ast ExprCast) {
        if let Type::Path(path) = &*expr_cast.ty {
            if path.path.is_ident("u32") {
                if let Expr::Path(expr_path) = &*expr_cast.expr {
                    self.as_usages
                        .push(expr_path.path.segments.first().unwrap().ident.to_string());
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct FnVisitor {
    functions: Vec<String>,
}

impl FnVisitor {
    pub fn new(file: &syn::File) -> Self {
        let mut visitor = Self {
            functions: Vec::new(),
        };

        visitor.visit_file(file);

        visitor
    }

    pub fn functions(&self) -> &Vec<String> {
        &self.functions
    }
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let name = i.sig.ident.to_string();
        self.functions.push(name);
    }
}
