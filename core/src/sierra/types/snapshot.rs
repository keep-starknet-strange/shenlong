use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn snapshot(&mut self, type_declaration: &TypeDeclaration) {
        let ty =
            if let GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) = type_declaration.long_id.generic_args[0] {
                *self.types_by_id.get(&id).expect("Type should have been defined before struct")
            } else {
                panic!("store_temp only takes type or user type")
            };
        self.types_by_id.insert(type_declaration.id.id, ty);
    }
}
