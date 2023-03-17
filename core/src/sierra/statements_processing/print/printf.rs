use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValue};
use tracing::warn;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Defines a function that wraps printf to print a value of type `ty`.
    /// Useful for debugging, specially for integers with a size > 64bit.
    /// Requires printf to be declared beforehand.
    pub fn printf_for_type(&mut self, ty: BasicMetadataTypeEnum<'ctx>, func_name: &str, type_name: &str) {
        if self.module.get_function(func_name).is_some() {
            warn!("Tried to redefine {} printf function.", func_name);
            return;
        }

        let void_type = self.context.void_type();

        // Wrapper of printf to call it with the expected main return value.
        let func = self.module.add_function(func_name, void_type.fn_type(&[ty], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        self.debug.create_function(func_name, &func, None, &[*self.debug.types_by_name.get(type_name).unwrap()], None);

        // For now we only allow printing int types.
        let value = func.get_first_param().expect("Function should have exactly 1 param").into_int_value();
        // We round up the number's bit size because we're printing the number by chunks of 32 bits.
        let rounded_type = self.context.custom_width_int_type(round_up(value.get_type().get_bit_width()));
        let value = self.builder.build_int_z_extend(value, rounded_type, "rounded_size_val");
        let mut bit_width = value.get_type().get_bit_width();

        while bit_width > 0 {
            let shift_by = value.get_type().const_int(bit_width.saturating_sub(32).into(), false);
            let temp_value = self.builder.build_right_shift(value, shift_by, false, "shifted");
            let trunc = self.builder.build_int_cast(temp_value, self.context.i32_type(), "print_value_trunc");

            self.call_printf("%08X", &[trunc.as_basic_value_enum().into()]);
            bit_width = bit_width.saturating_sub(32);
        }

        // Print the new line.
        self.call_printf("\n", &[]);

        self.builder.build_return(None);
    }

    /// Utility function to print a value using the generated printf function.
    /// Must have generated the function beforehand with `printf_for_type` for the value type.
    pub fn call_print_type(&self, func_name: &str, value: BasicMetadataValueEnum<'ctx>) {
        self.builder.build_call(
            self.module
                .get_function(func_name)
                .unwrap_or_else(|| panic!("{func_name} function should be declared before making calls to it")),
            &[value],
            "worked",
        );
    }

    /// A printf call, with the given format string and the given format values.
    /// Requires printf to be declared beforehand.
    ///
    /// Note that the max bit width of the values to be printed correctly is 64 for %lld and 32 for
    /// %ld.
    pub fn call_printf(&self, fmt: &str, values: &[BasicMetadataValueEnum<'ctx>]) {
        let i8_type = self.context.i8_type();
        let mut fmt_c = fmt.to_string();
        fmt_c.push('\0'); // c string null
        let to_print = fmt_c.chars().map(|c| i8_type.const_int(c.into(), false)).collect::<Vec<_>>();
        let printfunc = self.module.get_function("printf").expect("printf should be defined before");
        let format_ptr = self.builder.build_alloca(i8_type.array_type(to_print.len() as u32), "format");

        self.builder.build_store(format_ptr, self.context.i8_type().const_array(&to_print));

        let mut printf_args: Vec<BasicMetadataValueEnum<'ctx>> = vec![format_ptr.into()];
        printf_args.extend(values);

        self.builder.build_call(printfunc, &printf_args, "chars_printed");
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
