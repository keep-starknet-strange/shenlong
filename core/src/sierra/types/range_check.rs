use cairo_lang_sierra::program::TypeDeclaration;
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn range_check(&mut self, type_declaration: &TypeDeclaration) {
        self.id_from_name.insert("RangeCheck".to_owned(), type_declaration.id.id.to_string());
        self.types.insert(
            type_declaration.id.id.to_string(),
            Box::new(self.context.custom_width_int_type(0).as_basic_type_enum()),
        );
    }
}
