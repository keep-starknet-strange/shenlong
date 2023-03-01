use cairo_lang_sierra::program::TypeDeclaration;
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

pub const FELT_INT_WIDTH: u32 = 253;
pub const DOUBLE_FELT_INT_WIDTH: u32 = 512;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Declares the LLVM IR representation for the felt type.
    ///
    /// # Arguments
    ///
    /// * `type_declaration` - the sierra type declaration
    pub fn felt(&mut self, type_declaration: &TypeDeclaration) {
        // Save felt in the id from name map to be able to get the LLVM IR type from the type name.
        self.id_from_name.insert("felt".to_owned(), type_declaration.id.id.to_string());
        // Save the felt type in the types map.
        self.types.insert(
            type_declaration.id.id.to_string(),
            // Use 253 bits to represent a felt to leave space for the sign.
            Box::new(self.context.custom_width_int_type(FELT_INT_WIDTH).as_basic_type_enum()),
        );
    }
}
