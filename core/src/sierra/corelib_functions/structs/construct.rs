use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicMetadataTypeEnum;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a struct construction operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `StructConstruct<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    /// Panics if the sierra file doesn't have the debug infos.
    pub fn struct_construct(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        // Type of the struct that we have to construct.

        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();

        let type_id = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => *id,
            _val => {
                panic!("Struct construct only takes predefined structs")
            }
        };

        let return_type = self.types_by_id.get(&type_id).unwrap().into_struct_type();
        let debug_arg_types = self.debug.struct_types_by_id.get(&type_id).unwrap().clone();
        let debug_return_type = *self.debug.types_by_id.get(&type_id).unwrap();

        // fn StructConstruct<T>(field_1: t1, field2: t2 ...) -> T
        let func = self.module.add_function(
            func_name,
            return_type.fn_type(
                &return_type
                    .get_field_types()
                    .into_iter()
                    .map(|arg_type| arg_type.into())
                    .collect::<Vec<BasicMetadataTypeEnum>>(),
                false,
            ),
            None,
        );

        let debug_func = self.debug.create_function(func_name, &func, Some(debug_return_type), &debug_arg_types, None);

        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // Allocate memory for the struct.
        let struct_ptr = self.builder.build_alloca(return_type, "res_ptr");
        // Store each field in the struct.
        for (i, param) in func.get_params().iter().enumerate() {
            let tuple_ptr = self
                .builder
                .build_struct_gep(return_type, struct_ptr, i as u32, format!("field_{i}_ptr").as_str())
                .unwrap();
            self.builder.build_store(tuple_ptr, *param);
        }
        self.builder.build_return(Some(&self.builder.build_load(return_type, struct_ptr, "res")));

        // Debug values
        for (i, (value, arg_ty)) in func.get_params().iter().zip(debug_arg_types).enumerate() {
            let debug_local_var = self.debug.create_local_variable(&i.to_string(), debug_func.scope, arg_ty, None);
            self.debug.insert_dbg_value(
                *value,
                debug_local_var,
                self.builder.get_current_debug_location().unwrap(),
                func.get_first_basic_block().unwrap().get_first_instruction().unwrap(),
            );
        }
    }
}
