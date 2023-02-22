use inkwell::module::Linkage;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};
use inkwell::AddressSpace;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Defines a function that wraps printf to print a value of type `ty`.
    /// Useful for debugging.
    pub fn printf_for_type(&self, ty: BasicMetadataTypeEnum<'ctx>, func_name: &str) {
        // declare i32 @printf(ptr, ...)
        // i8* is the str type
        let i8_type = self.context.i8_type();

        // return type of printf
        let i32_type = self.context.i32_type();

        let printfunc = if let Some(printfunc) = self.module.get_function("printf") {
            printfunc
        } else {
            let str_type = i8_type.ptr_type(AddressSpace::from(0));
            // Define llvm signature of printf
            let printf_type = i32_type.fn_type(&[str_type.into()], true);
            // Declare the function in the module.
            self.module.add_function("printf", printf_type, Some(Linkage::External))
        };

        // Wrapper of printf to call it with the expected main return value.
        let func = self.module.add_function(func_name, i32_type.fn_type(&[ty], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // printf format "%ld\n"
        let format_ptr = self.builder.build_alloca(i8_type.array_type(5), "prefix");
        self.builder.build_store(
            format_ptr,
            self.context.i8_type().const_array(&[
                i8_type.const_int(b'%'.into(), false),
                i8_type.const_int(b'l'.into(), false),
                i8_type.const_int(b'd'.into(), false),
                i8_type.const_int(b'\n'.into(), false),
                i8_type.const_int(b'\0'.into(), false),
            ]),
        );
        // Returns the number of characters printed (from printf).
        let res = self
            .builder
            .build_call(
                printfunc,
                &[format_ptr.into(), func.get_first_param().expect("Function should have exactly 1 param").into()],
                "printed_characters_n",
            )
            .try_as_basic_value()
            .left()
            .expect("Function should return exactly 1 value");
        self.builder.build_return(Some(&res));
    }

    /// Utility function to print a value.
    /// Must have generated the function beforehand with `printf_for_type` for the value type.
    pub fn call_print(&self, func_name: &str, value: BasicMetadataValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        self.builder
            .build_call(
                self.module
                    .get_function(func_name)
                    .unwrap_or_else(|| panic!("{func_name} function should be declared before making calls to it")),
                &[value],
                "worked",
            )
            .try_as_basic_value()
            .expect_left("Call should return a value")
    }
}
