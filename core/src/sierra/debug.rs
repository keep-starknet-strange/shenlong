use std::borrow::Cow;

use inkwell::debug_info::{AsDIScope, DIFlags, DIFlagsConstants, DILocation, DIScope, DISubprogram, DIType};
use inkwell::types::StructType;
use inkwell::values::FunctionValue;
use regex::Regex;

use super::llvm_compiler::{DebugCompiler, FunctionDebugInfo};

impl<'a, 'ctx> DebugCompiler<'a, 'ctx> {
    /// Create and set the current debug location.
    #[inline]
    pub fn debug_location(&self, scope: Option<DIScope<'ctx>>) -> DILocation {
        let location = self.debug_builder.create_debug_location(
            self.context,
            self.get_line(),
            0,
            scope.unwrap_or_else(|| self.compile_unit.as_debug_info_scope()),
            None,
        );
        self.builder.set_current_debug_location(location);
        location
    }

    /// Utility method to generate debug info for a function.
    ///
    /// Sets the current debug location too, so you don't have to call debug_location with the
    /// returned scope.
    pub fn create_function(
        &mut self,
        func_name: &str,
        func: &FunctionValue<'ctx>,
        return_type: Option<DIType<'ctx>>,
        parameter_types: &[DIType<'ctx>],
        scope_line: Option<usize>,
    ) -> DIScope<'ctx> {
        let debug_func_name = self.clear_function_name(func_name);
        let subroutine_type = self.debug_builder.create_subroutine_type(
            self.compile_unit.get_file(),
            return_type,
            parameter_types,
            DIFlags::PUBLIC,
        );
        let func_scope: DISubprogram<'_> = self.debug_builder.create_function(
            self.compile_unit.as_debug_info_scope(),
            &debug_func_name,
            Some(func_name),
            self.compile_unit.get_file(),
            self.get_line(), // line number
            subroutine_type,
            true,
            true,
            scope_line.map(|x| x as u32).unwrap_or_else(|| self.get_line()), // scope line
            DIFlags::PUBLIC,
            false,
        );
        func.set_subprogram(func_scope);
        let scope = func_scope.as_debug_info_scope();

        let mut params = Vec::with_capacity(parameter_types.len());

        for (i, param) in parameter_types.iter().enumerate() {
            let local_var = self.debug_builder.create_parameter_variable(
                scope,
                &i.to_string(),
                i as u32 + 1,
                self.compile_unit.get_file(),
                self.get_line(),
                *param,
                true,
                DIFlags::PUBLIC,
            );
            params.push(local_var);
        }

        let info = FunctionDebugInfo { function: func_scope, params };

        self.functions.insert(func_name.to_string(), info);

        self.debug_location(Some(scope));
        scope
    }

    /// Creates a type debug info by name.
    pub fn create_type(&mut self, id: u64, name: &str, size_in_bits: u64) -> DIType<'ctx> {
        let debug_type =
            self.debug_builder.create_basic_type(name, size_in_bits, 0x00, DIFlags::PUBLIC).unwrap().as_type();

        self.types_by_id.insert(id, debug_type);
        self.types_by_name.insert(name.to_string(), debug_type);
        debug_type
    }

    /// Creates a struct debug type.
    pub fn create_struct(
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

        let debug_struct_type = self.debug_builder.create_struct_type(
            self.compile_unit.as_debug_info_scope(),
            name,
            self.compile_unit.get_file(),
            self.get_line(),
            bits,
            align_bits,
            inkwell::debug_info::DIFlags::PUBLIC,
            None,
            fields,
            0,
            None,
            name,
        );

        self.types_by_id.insert(id, debug_struct_type.as_type());
        self.types_by_name.insert(name.to_string(), debug_struct_type.as_type());
        debug_struct_type.as_type()
    }

    /// Returns the id for a function return struct type id.
    ///
    /// Arbitrarily the decided function generated struct return types have id = the function id +
    /// 100_000.
    pub const fn get_fn_struct_type_id(&self, func_id: u64) -> u64 {
        func_id + 100_000
    }

    /// Returns the id for a function return struct type name.
    ///
    /// Arbitrarily decided the generated struct return types have name = "return_type_{}" where {}
    /// is the function name.
    pub fn get_fn_struct_type_name(&self, func_debug_name: &str) -> String {
        format!("return_type_{}", func_debug_name)
    }

    /// User defined functions in sierra have a namespace before, it's not really useful for
    /// debugging.
    ///
    /// a::a::fn_name -> fn_name
    pub fn clear_function_name<'s>(&self, name: &'s str) -> Cow<'s, str> {
        let regex = Regex::new(r#".*::"#).unwrap();
        regex.replace(name, "")
    }
}
