use cairo_lang_sierra::program::TypeDeclaration;
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn u32(&mut self, type_declaration: &TypeDeclaration) {
        let ty = self.context.custom_width_int_type(32).as_basic_type_enum();
        self.types_by_name.insert("u32".to_owned(), ty);
        self.types_by_id.insert(type_declaration.id.id, ty);
    }
}
