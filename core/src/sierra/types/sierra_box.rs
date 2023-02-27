use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::debug_info::DIFlagsConstants;

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
                    type_name.clone(),
                    // A type can't use an undefined type so it should be declared before so it shouldn't panic.
                    // The box type is ignored in LLVM IR we just define `Box<T>` as `T`.
                    Box::from(self.types.get(&inner_type_name).unwrap().as_basic_type_enum()),
                );

                // Debug info
                if let Some(dibuilder) = &self.dibuilder {
                    let ditypes = self.ditypes.as_mut().unwrap();
                    let inner_type = ditypes.get(&inner_type_name).unwrap();
                    let debug_type_name = type_declaration.id.debug_name.as_ref().unwrap().as_str();
                    ditypes.insert(
                        type_name,
                        dibuilder
                            .create_basic_type(
                                debug_type_name,
                                inner_type.get_size_in_bits(),
                                0x00,
                                inkwell::debug_info::DIFlags::PUBLIC,
                            )
                            .unwrap()
                            .as_type(),
                    );
                }
            }
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("Box only takes type or user type")
            }
        };
    }
}
