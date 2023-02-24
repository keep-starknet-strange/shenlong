use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn non_zero(&mut self, type_declaration: &TypeDeclaration) {
        match &type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => self.types.insert(
                type_declaration.id.id.to_string(),
                // A type can't use an undefined type so it should be declared before so it shouldn't panic.
                Box::from(self.types.get(&id.to_string()).unwrap().as_basic_type_enum()),
            ),
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("NonZero only takes type or user type")
            }
        };
    }
}