use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Declares the LLVM IR representation of a sierra struct.
    ///
    /// # Arguments
    ///
    /// * `type_declaration` - the sierra type declaration.
    pub fn sierra_struct(&mut self, type_declaration: &TypeDeclaration) {
        let mut args = vec![];
        for generic_arg in type_declaration.long_id.generic_args.iter() {
            match generic_arg {
                GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => args.push(
                    self.types
                        .get(&id.to_string())
                        .expect("Type should have been defined before struct")
                        .as_basic_type_enum(),
                ),
                // Ignore the user type as it is not a struct field.
                GenericArg::UserType(_) => continue,
                _val => {
                    panic!("store_temp only takes type or user type")
                }
            };
        }
        self.types.insert(
            type_declaration.id.id.to_string(),
            Box::from(self.context.struct_type(&args, false).as_basic_type_enum()),
        );
    }
}
