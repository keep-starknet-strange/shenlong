use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::types::BasicType;
use inkwell::AddressSpace;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn array(&mut self, type_declaration: &TypeDeclaration) {
        match &type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name }) => {
                let val_type = self.types_by_id.get(id).unwrap().into_int_type();
                // Get the arrya value type name.
                let debug_name = debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string();
                let array_debug_name = format!("Array<{:#}>", debug_name);
                // Array capacity and len are felts for now. The reason for that is that there is no specific
                // reason.
                let felt_type = *self.types_by_name.get("felt").unwrap();
                let ty = self
                    .context
                    .struct_type(&[val_type.ptr_type(AddressSpace::default()).into(), felt_type, felt_type], false);
                // Create debug type for the array pointer.
                let ptr_debug = self.debug.create_type(9090909009, "arr_ptr", 64);
                // Save Array<T> in the types map.
                self.types_by_name.insert(array_debug_name.clone(), ty.as_basic_type_enum());
                // Save the type by its id.
                self.types_by_id.insert(type_declaration.id.id, ty.as_basic_type_enum());
                // Get the felt debug type.
                let felt_debug = *self.debug.types_by_name.get("felt").unwrap();
                // Create debug struct type for array.
                self.debug.create_struct(
                    type_declaration.id.id,
                    &array_debug_name,
                    &ty,
                    &[ptr_debug, felt_debug, felt_debug],
                )
            }
            GenericArg::UserType(_) => todo!(),
            _ => {
                panic!("Arrays only takes type or user type")
            }
        };
    }
}
