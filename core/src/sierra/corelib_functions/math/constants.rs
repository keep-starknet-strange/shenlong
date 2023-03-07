use cairo_lang_sierra::program::GenericArg::Value;
use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::{BasicType, StringRadix};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of an int constant.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of {un|felt}_const.
    ///
    /// # Error
    ///
    /// Panics if the int type has not been declared previously.
    pub fn int_const(&mut self, libfunc_declaration: &LibfuncDeclaration, int_ty: &str) {
        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();
        let return_type = self.types_by_name.get(int_ty).expect("Can't get int type from name");
        let debug_return_type = *self.debug.types_by_name.get(int_ty).expect("Can't get int type from name");

        // fn int_const() -> int
        let func = self.module.add_function(func_name, return_type.fn_type(&[], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        let debug_func = self.debug.create_function(func_name, &func, Some(debug_return_type), &[], None);

        // Convert the bigint value of the constant into an LLVM IR const int value. Panics if the constant
        // value is not a decimal value.

        let ret = if let Value(val) = &libfunc_declaration.long_id.generic_args[0] {
            return_type
                .as_basic_type_enum()
                .into_int_type()
                .const_int_from_string(val.to_string().as_str(), StringRadix::Decimal)
                .expect("Couldn't convert to string the int constant value")
        } else {
            // If the constant doesn't have any value it panics because a constant should have a value.
            panic!("No value for constant")
        };
        let inst = self.builder.build_return(Some(&ret));

        // Debug values
        let debug_local_var = self.debug.create_local_variable(func_name, debug_func.scope, debug_return_type, None);
        self.debug.insert_dbg_value(
            ret.into(),
            debug_local_var,
            self.builder.get_current_debug_location().unwrap(),
            inst,
        );
    }
}
