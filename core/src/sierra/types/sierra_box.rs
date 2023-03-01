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
                // A type can't use an undefined type so it should be declared before so it shouldn't panic.
                // The box type is ignored in LLVM IR we just define `Box<T>` as `T`.
                let inner_type = *self.types_by_id.get(id).unwrap();

                self.types_by_id.insert(*id, inner_type);
                self.types_by_name.insert(type_declaration.id.debug_name.as_ref().unwrap().to_string(), inner_type);
            }
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("Box only takes type or user type")
            }
        };
    }
}
