use cairo_lang_sierra::ids::{ConcreteTypeId, UserTypeId};
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn sierra_enum(&mut self, type_declaration: &TypeDeclaration) {
        let mut args = vec![];
        let mut debug_args = vec![];
        let mut ty_name = "".to_owned();
        for generic_arg in type_declaration.long_id.generic_args.iter() {
            match generic_arg {
                GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                    args.push(
                        self.types_by_id
                            .get(id)
                            .expect("Type should have been defined before struct")
                            .as_basic_type_enum(),
                    );
                    debug_args
                        .push(*self.debug.types_by_id.get(id).expect("Type should have been defined before struct"));
                }

                GenericArg::UserType(UserTypeId { id: _id, debug_name }) => {
                    ty_name = debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string();
                }
                _val => {
                    panic!("store_temp only takes type or user type")
                }
            };
        }
        let ty = self.context.struct_type(&args, false);
        self.types_by_id.insert(type_declaration.id.id, ty.as_basic_type_enum());
        self.types_by_name.insert(ty_name.clone(), ty.as_basic_type_enum());
        self.debug.create_struct(type_declaration.id.id, &ty_name, &ty, &debug_args);
    }
}
