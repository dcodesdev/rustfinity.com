use std::rc::Rc;

/// Helper functions for macros
#[derive(Debug)]
pub struct Const {
    file: Rc<syn::File>,
}

impl Const {
    pub fn new(file: Rc<syn::File>) -> Self {
        Self { file }
    }

    pub fn constants(&self) -> Vec<syn::ItemConst> {
        self.file
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Const(item) => Some(item.clone()),
                _ => None,
            })
            .collect()
    }
}
