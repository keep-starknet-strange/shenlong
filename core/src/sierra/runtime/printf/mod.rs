use inkwell::module::Linkage;
use inkwell::types::StructType;
use inkwell::AddressSpace;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a struct construction operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of StructConstruct<T>.
    ///
    /// # Error
    ///
    /// Returns an error if the type T has not been declared previously.
    pub fn printf(&self, ty: StructType<'ctx>) {
        let i8_type = self.context.i8_type();
        let str_type = i8_type.ptr_type(AddressSpace::from(0));
        let i32_type = self.context.i32_type();
        let printf_type = i32_type.fn_type(&[str_type.into()], true);
        let printfunc = self.module.add_function("printf", printf_type, Some(Linkage::External));
        let func = self.module.add_function("print", self.context.i32_type().fn_type(&[ty.into()], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        let prefix_ptr = self.builder.build_alloca(i8_type.array_type(2), "prefix");
        self.builder.build_store(
            prefix_ptr,
            self.context.i8_type().const_array(&[i8_type.const_int(37, false), i8_type.const_int(100, false)]),
        );
        let res = self
            .builder
            .build_call(
                printfunc,
                &[prefix_ptr.into(), func.get_first_param().expect("Function should have exactly 1 param").into()],
                "worked",
            )
            .try_as_basic_value()
            .left()
            .expect("Function should return exactly 1 value");
        self.builder.build_return(Some(&res));
    }
}
