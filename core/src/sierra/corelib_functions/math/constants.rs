use cairo_lang_sierra::program::GenericArg::Value;
use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::{BasicType, StringRadix};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt constant.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of felt_const.
    ///
    /// # Error
    ///
    /// Panics if the felt type has not been declared previously.
    pub fn felt_const(&self, libfunc_declaration: &LibfuncDeclaration) {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();
        let return_type = self.types_by_name.get("felt").expect("Can't get felt from name");
        let debug_return_type = *self.debug_types_by_name.get("felt").expect("Can't get felt from name");

        // fn felt_const() -> felt
        let func = self.module.add_function(func_name, return_type.fn_type(&[], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        self.create_function_debug(func_name, &func, Some(debug_return_type), &[]);

        // Convert the bigint value of the constant into an LLVM IR const int value. Panics if the constant
        // value is not a decimal value.
        let ret = if let Value(val) = &libfunc_declaration.long_id.generic_args[0] {
            return_type
                .as_basic_type_enum()
                .into_int_type()
                .const_int_from_string(val.to_string().as_str(), StringRadix::Decimal)
                .expect("Couldn't convert to string the felt constant value")
        } else {
            // If the constant doesn't have any value it panics because a constant should have a value.
            panic!("No value for felt constant")
        };
        self.builder.build_return(Some(&ret));
    }
}
