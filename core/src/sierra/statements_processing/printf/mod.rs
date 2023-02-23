use inkwell::module::Linkage;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValue};
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

        let void_type = self.context.void_type();

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
        let func = self.module.add_function(func_name, void_type.fn_type(&[ty], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // printf format "%ld\n"
        let format_ptr = self.builder.build_alloca(i8_type.array_type(5), "prefix");
        self.builder.build_store(
            format_ptr,
            self.context.i8_type().const_array(&[
                i8_type.const_int(b'%'.into(), false),
                i8_type.const_int(b'0'.into(), false),
                i8_type.const_int(b'8'.into(), false),
                i8_type.const_int(b'X'.into(), false),
                i8_type.const_int(b'\0'.into(), false),
            ]),
        );

        // For now we only allow printing int types.
        let value = func.get_first_param().expect("Function should have exactly 1 param").into_int_value();
        let rounded_type = self.context.custom_width_int_type(round_up(value.get_type().get_bit_width()));
        let value = self.builder.build_int_s_extend(value, rounded_type, "rounded_size_val");
        let mut bit_width = value.get_type().get_bit_width();

        while bit_width > 0 {
            let shift_by = value.get_type().const_int(bit_width.saturating_sub(32).into(), false);
            let temp_value = self.builder.build_right_shift(value, shift_by, false, "shifted");
            let trunc = self.builder.build_int_cast(temp_value, self.context.i32_type(), "print_value_trunc");

            self.builder
                .build_call(printfunc, &[format_ptr.into(), trunc.as_basic_value_enum().into()], "printed_chars")
                .try_as_basic_value()
                .left()
                .expect("Function should return exactly 1 value");
            bit_width = bit_width.saturating_sub(32);
        }

        // Print the new line.
        let format_ptr = self.builder.build_alloca(i8_type.array_type(2), "prefix_newline");
        self.builder.build_store(
            format_ptr,
            self.context
                .i8_type()
                .const_array(&[i8_type.const_int(b'\n'.into(), false), i8_type.const_int(b'\0'.into(), false)]),
        );
        self.builder
            .build_call(printfunc, &[format_ptr.into()], "printed_chars")
            .try_as_basic_value()
            .left()
            .expect("Function should return exactly 1 value");

        self.builder.build_return(None);
    }

    /// Utility function to print a value.
    /// Must have generated the function beforehand with `printf_for_type` for the value type.
    pub fn call_print(&self, func_name: &str, value: BasicMetadataValueEnum<'ctx>) {
        self.builder.build_call(
            self.module
                .get_function(func_name)
                .unwrap_or_else(|| panic!("{func_name} function should be declared before making calls to it")),
            &[value],
            "worked",
        );
    }
}

/// rounds to the nearest power of 2 up.
#[inline]
const fn round_up(value: u32) -> u32 {
    let mut power = 1;
    while power < value {
        power *= 2;
    }
    power
}
