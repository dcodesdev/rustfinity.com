use std::rc::Rc;
use syn::{Item, ItemFn, Stmt};

pub trait Func {
    fn file(&self) -> &Rc<syn::File>;
    fn fn_name(&self) -> &str;

    fn func<F>(&self, mut handler: F)
    where
        F: FnMut(&ItemFn),
    {
        let mut ran = false;
        self.file().items.iter().for_each(|item| {
            if let Item::Fn(f) = item {
                if f.sig.ident == self.fn_name() {
                    handler(f);
                    ran = true;
                }
            }
        });

        if !ran {
            panic!("Function {} not found", self.fn_name());
        }
    }

    fn func_stmts<F>(&self, mut handler: F)
    where
        F: FnMut(&ItemFn, &Stmt),
    {
        let mut ran = false;
        self.func(|f| {
            f.block.stmts.iter().for_each(|stmt| {
                handler(f, stmt);
                ran = true;
            })
        });

        if !ran {
            panic!("No statements found for function {}", self.fn_name());
        }
    }
}
