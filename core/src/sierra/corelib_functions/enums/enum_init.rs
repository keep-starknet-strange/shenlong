use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use num_traits::ToPrimitive;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of an enum initialisation operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `enum_init<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    /// Panics if the sierra file doesn't have the debug infos.
    pub fn enum_init(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        // Type of the enum that we have to construct.
        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();
        println!("Processing enum init {func_name}");
        println!("{:?}", libfunc_declaration);
        let (full_enum_type_id, enum_index_type_name) = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name }) => {
                (id, format!("ut@{}", debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string()))
            }
            _val => {
                panic!("Enum init only takes predefined enums")
            }
        };
        let enum_index_parameter = match &libfunc_declaration.long_id.generic_args[1] {
            GenericArg::Value(val) => val.to_usize().unwrap(),
            _ => {
                panic!("Enum init only takes predefined enums")
            }
        };
        let enum_index_type = self.types_by_name.get(&enum_index_type_name).unwrap().into_int_type();
        println!("Enum index: {} of type {:?}", enum_index_parameter, enum_index_type);
        let return_type = self.types_by_id.get(full_enum_type_id).unwrap().into_struct_type();
        println!("Return type: {:?}", return_type);

        // Find the location in the struct representation at which the payload is held
        let data_offset = self
            .enum_packing_index_by_id
            .get(full_enum_type_id)
            .expect("Enum data packing locations should have been registered before processing of enum_init")
            [enum_index_parameter];

        // TODO fix debug handling of enum types, shouldn't be indexing into the struct. 0 used here so it
        // doesn't break for now
        let debug_arg_type = self.debug.struct_types_by_id.get(full_enum_type_id).unwrap()[0];
        let debug_return_type = *self.debug.types_by_id.get(full_enum_type_id).unwrap();

        let input_type = return_type.get_field_type_at_index(data_offset as u32).unwrap();

        // fn enum_init<T, i>(T[i]) -> T
        // which is to say that to construct an enum we only need input for the value that's actually stored
        let func = self.module.add_function(func_name, return_type.fn_type(&[input_type.into()], false), None);

        // TODO finish implementing debug func
        let _debug_func =
            self.debug.create_function(func_name, &func, Some(debug_return_type), &[debug_arg_type], None);

        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // Allocate memory for the enum on the stack.
        let enum_ptr = self.builder.build_alloca(return_type, "res_ptr");
        // Store the index in the struct
        let index_to_store = enum_index_type.const_int(enum_index_parameter as u64, false);
        let ptr_to_index = self.builder.build_struct_gep(return_type, enum_ptr, 0u32, "index_ptr").unwrap();
        self.builder.build_store(ptr_to_index, index_to_store);
        // Store the payload in the struct
        let value_to_store = func.get_first_param().expect("enum_init should have one parameter");
        let ptr_to_member =
            self.builder.build_struct_gep(return_type, enum_ptr, data_offset as u32, "member_ptr").unwrap();
        self.builder.build_store(ptr_to_member, value_to_store);

        // Finally, return the enum
        self.builder.build_return(Some(&self.builder.build_load(return_type, enum_ptr, "res")));

        // // Store each field in the struct.
        // for (i, param) in func.get_params().iter().enumerate() {
        //     let tuple_ptr = self
        //         .builder
        //         .build_struct_gep(return_type, struct_ptr, i as u32, format!("field_{i}_ptr").as_str())
        //         .unwrap();
        //     self.builder.build_store(tuple_ptr, *param);
        // }
        // self.builder.build_return(Some(&self.builder.build_load(return_type, struct_ptr, "res")));

        // // Debug values
        // for (i, (value, arg_ty)) in func.get_params().iter().zip([debug_arg_type]).enumerate() {
        //     let debug_local_var = self.debug.create_local_variable(&i.to_string(), debug_func.scope,
        // arg_ty, None);     self.debug.insert_dbg_value(
        //         *value,
        //         debug_local_var,
        //         self.builder.get_current_debug_location().unwrap(),
        //         func.get_first_basic_block().unwrap().get_first_instruction().unwrap(),
        //     );
        // }
        println!("\n\n\n");
        // panic!();
    }
}
