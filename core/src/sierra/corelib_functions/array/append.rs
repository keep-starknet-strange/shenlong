use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicType;
use inkwell::AddressSpace;
use inkwell::IntPredicate::ULT;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of `array_append<T>` operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `array_append<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    pub fn array_append(&self, libfunc_declaration: &LibfuncDeclaration) {
        let (val_ty, array_ty) = match &libfunc_declaration.long_id.generic_args[0] {
            // Panics if the type has not been declared.
            GenericArg::Type(ConcreteTypeId { id, debug_name }) => (
                *self.types_by_id.get(id).unwrap(),
                self.types_by_name
                    .get(&format!("Array<{:#}>", debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string()))
                    .unwrap()
                    .into_struct_type(),
            ),
            // Not sure if dup can dup user defined types
            GenericArg::UserType(_) => todo!(),
            _ => panic!("array_push only takes type or user type"),
        };

        // fn array_append<Array<T>>(array: Array<T>, val: T) -> Array<T>
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            array_ty.fn_type(&[array_ty.into(), val_ty.into()], false),
            None,
        );
        let arg = func.get_first_param().unwrap();
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // Get the felt type because length and capacity are felts.
        let felt_type = *self.types_by_name.get("felt").unwrap();
        // Get the ptr of the array.
        let array_ptr = self.builder.build_alloca(array_ty, "array_ptr");
        self.builder.build_store(array_ptr, arg);
        // Get the ptr to the values.
        let ptr_ptr = self.builder.build_struct_gep(array_ty, array_ptr, 0, "ptr_ptr").unwrap();
        let ptr = self.builder.build_load(
            array_ty.get_field_type_at_index(0).unwrap().ptr_type(AddressSpace::default()),
            ptr_ptr,
            "ptr",
        );
        // Get the array current length.
        let len_ptr = self.builder.build_struct_gep(array_ty, array_ptr, 1, "len_ptr").unwrap();
        let len = self.builder.build_load(felt_type.ptr_type(AddressSpace::default()), len_ptr, "len");
        // Get the array current capacity.
        let capacity_ptr = self.builder.build_struct_gep(array_ty, array_ptr, 2, "capacity_ptr").unwrap();
        let capacity = self.builder.build_load(felt_type.ptr_type(AddressSpace::default()), capacity_ptr, "capacity");

        // Check if the array has enough capacity to add a new value.
        let check_array_cap =
            self.builder.build_int_compare(ULT, len.into_int_value(), capacity.into_int_value(), "is_array_big_enough");

        // if then
        let then_bb = self.context.append_basic_block(func, "then");
        // else
        let else_bb = self.context.append_basic_block(func, "else");

        self.builder.build_conditional_branch(check_array_cap, then_bb, else_bb);
        self.builder.position_at_end(then_bb);
        let empty_cell_id = len.into_int_value().const_add(felt_type.into_int_type().const_int(1, false));
        let empty_cell;
        unsafe {
            empty_cell = self.builder.build_gep(
                array_ty.get_field_type_at_index(0).unwrap(),
                ptr.into_pointer_value(),
                &[empty_cell_id],
                "empty_cell",
            );
        }
        self.builder.build_store(empty_cell, func.get_last_param().unwrap());
    }
}
