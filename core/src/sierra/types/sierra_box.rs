use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Declares `Box<T>`.
    ///
    /// # Arguments
    ///
    /// * `type_declaration` - the sierra type declaration.
    pub fn sierra_box(&mut self, type_declaration: &TypeDeclaration) {
        match &type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                let type_name = type_declaration.id.id.to_string();
                let inner_type_name = id.to_string();

                self.types.insert(
                    type_name,
                    // A type can't use an undefined type so it should be declared before so it shouldn't panic.
                    // The box type is ignored in LLVM IR we just define `Box<T>` as `T`.
                    Box::from(self.types.get(&inner_type_name).unwrap().as_basic_type_enum()),
                );
            }
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("Box only takes type or user type")
            }
        };
    }
}
