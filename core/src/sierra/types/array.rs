use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::types::BasicType;
use inkwell::AddressSpace;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn array(&mut self, type_declaration: &TypeDeclaration) {
        let val_type = match type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                self.types_by_id.get(&id).unwrap().into_int_type()
            }
            GenericArg::UserType(_) => todo!(),
            _ => {
                panic!("Arrays only takes type or user type")
            }
        };

        let ty = self.context.struct_type(
            &[val_type.ptr_type(AddressSpace::default()).into(), self.context.custom_width_int_type(252).into()],
            false,
        );
        self.types_by_name.insert("Array".to_owned(), ty.as_basic_type_enum());
        self.types_by_id.insert(type_declaration.id.id, ty.as_basic_type_enum());
    }
}
