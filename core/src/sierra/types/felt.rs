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
        // Use 253 bits to represent a felt to leave space for the sign.
        let felt_type = self.context.custom_width_int_type(FELT_INT_WIDTH).as_basic_type_enum();

        let debug_name = type_declaration.id.debug_name.as_ref().unwrap().as_str();

        // Save the felt type in the types map.
        self.types_by_id.insert(type_declaration.id.id, felt_type);
        self.types_by_name.insert(debug_name.to_string(), felt_type);
        self.debug.create_debug_type(type_declaration.id.id, debug_name, FELT_INT_WIDTH.into());
    }
}
