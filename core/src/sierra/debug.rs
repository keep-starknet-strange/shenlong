use inkwell::debug_info::{AsDIScope, DIFlags, DIFlagsConstants, DIScope, DISubprogram, DIType};
use inkwell::types::StructType;
use inkwell::values::FunctionValue;

use super::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Create and set the current debug location.
    #[inline]
    pub fn debug_location(&self, scope: Option<DIScope<'ctx>>) {
        let location = self.dibuilder.create_debug_location(
            self.context,
            self.current_line_estimate,
            0,
            scope.unwrap_or_else(|| self.compile_unit.as_debug_info_scope()),
            None,
        );
        self.builder.set_current_debug_location(location);
    }

    /// Utility method to generate debug info for a function.
    ///
    /// Sets the current debug location too, so you don't have to call debug_location with the
    /// returned scope.
    pub fn create_function_debug(
        &self,
        func_name: &str,
        func: &FunctionValue<'ctx>,
        return_type: Option<DIType<'ctx>>,
        parameter_types: &[DIType<'ctx>],
    ) -> DIScope<'ctx> {
        let subroutine_type = self.dibuilder.create_subroutine_type(
            self.compile_unit.get_file(),
            return_type,
            parameter_types,
            DIFlags::PUBLIC,
        );
        let func_scope: DISubprogram<'_> = self.dibuilder.create_function(
            self.compile_unit.as_debug_info_scope(),
            func_name,
            None,
            self.compile_unit.get_file(),
            self.current_line_estimate, // line number
            subroutine_type,
            true,
            true,
            0, // scope line
            DIFlags::PUBLIC,
            false,
        );
        func.set_subprogram(func_scope);
        let scope = func_scope.as_debug_info_scope();
        self.debug_location(Some(scope));
        scope
    }

    /// Creates a type debug info by name.
    pub fn create_debug_type(&mut self, id: u64, name: &str, size_in_bits: u64) -> DIType<'ctx> {
        let debug_type = self
            .dibuilder
            .create_basic_type(name, size_in_bits, 0x00, inkwell::debug_info::DIFlags::PUBLIC)
            .unwrap()
            .as_type();

        self.debug_types_by_id.insert(id, debug_type);
        self.debug_types_by_name.insert(name.to_string(), debug_type);
        debug_type
    }

    /// Creates a struct debug type.
    pub fn create_debug_type_struct(
        &mut self,
        id: u64,
        name: &str,
        struct_type: &StructType<'ctx>,
        fields: &[DIType<'ctx>],
    ) -> DIType<'ctx> {
        let struct_align = struct_type.get_alignment();

        let mut bits = 0;
        let align_bits = struct_align.get_type().get_bit_width();

        for arg in fields {
            bits += arg.get_size_in_bits();
        }

        let debug_struct_type = self.dibuilder.create_struct_type(
            self.compile_unit.as_debug_info_scope(),
            name,
            self.compile_unit.get_file(),
            self.current_line_estimate,
            bits,
            align_bits,
            inkwell::debug_info::DIFlags::PUBLIC,
            None,
            fields,
            0,
            None,
            name,
        );

        self.debug_types_by_id.insert(id, debug_struct_type.as_type());
        self.debug_types_by_name.insert(name.to_string(), debug_struct_type.as_type());
        debug_struct_type.as_type()
    }

    /// Returns the id for a function return struct type id.
    ///
    /// Arbitrarily the decided function generated struct return types have id = the function id +
    /// 100_000.
    pub const fn get_debug_function_return_struct_type_id(func_id: u64) -> u64 {
        func_id + 100_000
    }

    /// Returns the id for a function return struct type name.
    ///
    /// Arbitrarily decided the generated struct return types have name = "return_type_{}" where {}
    /// is the function name.
    pub fn get_debug_function_return_struct_type_name(func_debug_name: &str) -> String {
        format!("return_type_{}", func_debug_name)
    }
}
