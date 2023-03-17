use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::values::{BasicValueEnum, FunctionValue};
use itertools::Itertools;

use crate::sierra::corelib_functions::math::DEFAULT_PRIME;
use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn print_felt_representation(&mut self, sierra_type: &TypeDeclaration, llvm_value: BasicValueEnum<'ctx>) {
        let print_func_name = format!(
            "print_felt_representation_{}",
            sierra_type.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str()
        );

        // If the appropriate function has been generated already then get it here, other generate one with
        // an empty body
        let (print_func, needs_implementation) =
            self.module.get_function(print_func_name.as_str()).map(|x| (x, false)).unwrap_or_else(|| {
                // Generate new print function
                let llvm_type = llvm_value.get_type();
                let fn_type = self.context.void_type().fn_type(&[llvm_type.into()], false);
                (self.module.add_function(print_func_name.as_str(), fn_type, None), true)
            });

        // While the builder is at the appropriate position, build the call

        self.builder.build_call(print_func, &[llvm_value.into()], "");

        if needs_implementation {
            self.builder.position_at_end(self.context.append_basic_block(print_func, "entry"));

            let sierra_type_category_name = sierra_type.long_id.generic_id.0.as_str();
            match sierra_type_category_name {
                "felt" => self.write_print_felt_body(print_func),
                "NonZero" => todo!("Print box felt representation"),
                "Box" => todo!("Print box felt representation"),
                "Struct" => self.write_print_struct_body(print_func, sierra_type),
                "Enum" => self.write_print_enum_body(print_func, sierra_type),
                _ => panic!("Attempted to generate felt representation print for {}", sierra_type_category_name),
            }
        }

        // // All those types are known in advance. A struct is a combination of multiple
        // "primitive" types
    }

    fn write_print_felt_body(&mut self, print_func: FunctionValue<'ctx>) {
        let arg = print_func.get_first_param().expect("Print felt should have a first parameter").into_int_value();

        // This is necessary because of the signed llvm representation of felts we use
        let prime = arg.get_type().const_int_from_string(DEFAULT_PRIME, inkwell::types::StringRadix::Decimal).unwrap();
        let non_negative_value = self.builder.build_int_add(arg, prime, "nn");
        let wrapped_value = self.builder.build_int_unsigned_rem(non_negative_value, prime, "wrapped");
        let printf = self
            .module
            .get_function("printf")
            .expect("printf should be defined before generating felt representation print functions");
        let felt_format_string = self.builder.build_global_string_ptr("%064X\n", "felt_printf_format");
        self.builder.build_call(printf, &[felt_format_string.as_pointer_value().into(), wrapped_value.into()], "");
        self.builder.build_return(None);
    }

    fn write_print_struct_body(&mut self, print_func: FunctionValue<'ctx>, sierra_type: &TypeDeclaration) {
        let llvm_struct_value = print_func.get_first_param().unwrap().into_struct_value();
        let num_fields = llvm_struct_value.get_type().count_fields();
        if num_fields > 0 {
            let mut field_vals = vec![];
            for idx in 0..num_fields {
                field_vals.push(
                    self.builder.build_extract_value(llvm_struct_value, idx, format!("member_{idx}").as_str()).unwrap(),
                );
            }
            let field_vals = field_vals;

            let field_types = sierra_type.long_id.generic_args[1..].iter();

            for (member_value, member_type) in field_vals.iter().zip_eq(field_types) {
                match member_type {
                    GenericArg::Type(type_id) => {
                        self.builder.position_at_end(print_func.get_basic_blocks()[0]);
                        self.print_felt_representation(
                            self.program.type_declarations.iter().find(|decl| decl.id == *type_id).unwrap(),
                            *member_value,
                        );
                    }
                    _ => panic!("Struct type declaration arguments after the first should all be resolved"),
                }
            }
            self.builder.position_at_end(print_func.get_basic_blocks()[0]);
            self.builder.build_return(None);
        } else {
            todo!("Unit types")
        }
    }

    fn write_print_enum_body(&mut self, _print_func: FunctionValue<'ctx>, _sierra_type: &TypeDeclaration) {
        todo!("Enum print");
    }
}
