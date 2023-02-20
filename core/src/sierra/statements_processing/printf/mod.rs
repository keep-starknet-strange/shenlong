use inkwell::module::Linkage;
use inkwell::types::StructType;
use inkwell::AddressSpace;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR wrapper for the printf function.
    pub fn printf(&self, ty: StructType<'ctx>) {
        // declare i32 @printf(ptr, ...)
        // i8* is the str type
        let i8_type = self.context.i8_type();
        let str_type = i8_type.ptr_type(AddressSpace::from(0));
        // return type of printf
        let i32_type = self.context.i32_type();
        // Define llvm signature of printf
        let printf_type = i32_type.fn_type(&[str_type.into()], true);
        // Declare the function in the module.
        let printfunc = self.module.add_function("printf", printf_type, Some(Linkage::External));
        // Wrapper of printf to call it with the expected main return value.
        let func = self.module.add_function("print", self.context.i32_type().fn_type(&[ty.into()], false), None);
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
        // Return if print has worked.
        let res = self
            .builder
            .build_call(
                printfunc,
                &[format_ptr.into(), func.get_first_param().expect("Function should have exactly 1 param").into()],
                "worked",
            )
            .try_as_basic_value()
            .left()
            .expect("Function should return exactly 1 value");
        self.builder.build_return(Some(&res));
    }
}
