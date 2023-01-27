use cairo_lang_sierra::program::TypeDeclaration;
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn felt(&mut self, type_declaration: &TypeDeclaration) {
        self.id_from_name.insert("felt".to_owned(), type_declaration.id.id);
        self.types.insert(
            type_declaration.id.id,
            Box::new(self.context.custom_width_int_type(252).as_basic_type_enum()),
        );
    }
}
