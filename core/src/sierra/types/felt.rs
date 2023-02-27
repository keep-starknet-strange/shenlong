use cairo_lang_sierra::program::TypeDeclaration;
use inkwell::debug_info::DIFlagsConstants;
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

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
            Box::new(self.context.custom_width_int_type(253).as_basic_type_enum()),
        );

        if let Some(dibuilder) = &self.dibuilder {
            self.ditypes.as_mut().unwrap().insert(
                type_declaration.id.id.to_string(),
                dibuilder.create_basic_type("felt", 252, 0x00, inkwell::debug_info::DIFlags::PUBLIC).unwrap().as_type(),
            );
        }
    }
}
