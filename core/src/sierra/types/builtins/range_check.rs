use cairo_lang_sierra::program::TypeDeclaration;
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn range_check(&mut self, type_declaration: &TypeDeclaration) {
        let ty = self.context.custom_width_int_type(0).as_basic_type_enum();
        self.debug.create_type(type_declaration.id.id, "RangeCheck", 0);
        self.types_by_name.insert("RangeCheck".to_owned(), ty);
        self.types_by_id.insert(type_declaration.id.id, ty);
    }
}
