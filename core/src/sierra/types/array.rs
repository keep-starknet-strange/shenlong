use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn array(&mut self, type_declaration: &TypeDeclaration) {
        let array_type = match type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                self.types.get(&id.to_string()).unwrap().as_basic_type_enum()
            }
            GenericArg::UserType(_) => todo!(),
            _ => {
                panic!("Arrays only takes type or user type")
            }
        };

        self.id_from_name.insert("Array".to_owned(), type_declaration.id.id.to_string());
        self.types.insert(type_declaration.id.id.to_string(), Box::new(array_type.array_type(0).as_basic_type_enum()));
    }
}
