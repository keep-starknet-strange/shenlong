use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::debug_info::{AsDIScope, DIFlagsConstants};
use inkwell::types::BasicType;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Declares the LLVM IR representation of a sierra struct.
    ///
    /// # Arguments
    ///
    /// * `type_declaration` - the sierra type declaration.
    pub fn sierra_struct(&mut self, type_declaration: &TypeDeclaration) {
        dbg!(&type_declaration);
        let mut args = vec![];
        let mut args_ids = vec![];
        for generic_arg in type_declaration.long_id.generic_args.iter() {
            match generic_arg {
                GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                    args.push(
                        self.types
                            .get(&id.to_string())
                            .expect("Type should have been defined before struct")
                            .as_basic_type_enum(),
                    );
                    args_ids.push(id.to_string());
                }
                // Ignore the user type as it is not a struct field.
                GenericArg::UserType(_) => continue,
                _val => {
                    panic!("store_temp only takes type or user type")
                }
            };
        }

        let struct_type = self.context.struct_type(&args, false).as_basic_type_enum();
        let struct_align = struct_type.into_struct_type().get_alignment();
        self.types.insert(type_declaration.id.id.to_string(), Box::from(struct_type));

        // Debug info
        // TODO: check if this really works.

        let mut bits = 0;
        let align_bits = struct_align.get_type().get_bit_width();
        let mut struct_elements = Vec::with_capacity(args.len());

        for arg in &args {
            if let Some(size) = arg.size_of() {
                bits += size.get_type().get_bit_width();
            }
        }

        let felt_type_id = self.get_type_id_from_name("felt").unwrap();
        for arg_id in &args_ids {
            let debug_type = if arg_id.eq_ignore_ascii_case(felt_type_id) {
                self.get_debug_type("felt")
            } else {
                self.get_debug_type(arg_id)
            };

            struct_elements.push(debug_type);
        }

        let debug_name = type_declaration.id.debug_name.as_ref().unwrap().as_str();

        let debug_struct_type = self.dibuilder.create_struct_type(
            self.compile_unit.as_debug_info_scope(),
            debug_name,
            self.compile_unit.get_file(),
            self.current_line_estimate,
            bits.into(),
            align_bits,
            inkwell::debug_info::DIFlags::PUBLIC,
            None,
            &struct_elements,
            0,
            None,
            debug_name,
        );

        self.ditypes.insert(debug_name.to_string(), debug_struct_type.as_type());
    }
}
