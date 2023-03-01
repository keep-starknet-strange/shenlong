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
        let mut debug_args = vec![];

        for generic_arg in type_declaration.long_id.generic_args.iter() {
            match generic_arg {
                GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                    args.push(
                        self.types_by_id
                            .get(id)
                            .expect("Type should have been defined before struct")
                            .as_basic_type_enum(),
                    );
                    debug_args.push(*self.debug.types_by_id.get(id).unwrap());
                }
                // Ignore the user type as it is not a struct field.
                GenericArg::UserType(_) => continue,
                _val => {
                    panic!("store_temp only takes type or user type")
                }
            };
        }

        let struct_type = self.context.struct_type(&args, false);
        let debug_name = type_declaration.id.debug_name.as_ref().unwrap().as_str();

        self.types_by_id.insert(type_declaration.id.id, struct_type.as_basic_type_enum());
        self.types_by_name.insert(debug_name.to_string(), struct_type.as_basic_type_enum());

        self.debug.create_debug_type_struct(type_declaration.id.id, debug_name, &struct_type, &debug_args);
    }
}
