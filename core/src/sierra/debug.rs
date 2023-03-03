use std::borrow::Cow;

use inkwell::debug_info::{
    AsDIScope, DIFlags, DIFlagsConstants, DILocalVariable, DILocation, DIScope, DISubprogram, DIType,
};
use inkwell::types::StructType;
use inkwell::values::{BasicValueEnum, FunctionValue, InstructionValue};
use regex::Regex;

use super::llvm_compiler::{DebugCompiler, FunctionDebugInfo};

impl<'a, 'ctx> DebugCompiler<'a, 'ctx> {
    #[inline]
    pub fn create_debug_location(&self, line: u32, scope: DIScope<'ctx>) -> DILocation {
        let location = self.debug_builder.create_debug_location(self.context, line, 0, scope, None);
        location
    }

    /// Create and set the current debug location.
    #[inline]
    pub fn debug_location(&self, scope: Option<DIScope<'ctx>>) -> DILocation<'ctx> {
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
    ///
    /// Returns the function scope and the function argument local variables.
    pub fn create_function(
        &mut self,
        func_name: &str,
        func: &FunctionValue<'ctx>,
        return_type: Option<DIType<'ctx>>,
        parameter_types: &[DIType<'ctx>],
        scope_line: Option<usize>,
    ) -> FunctionDebugInfo<'ctx> {
        let debug_func_name = self.clear_function_name(func_name);
        let subroutine_type = self.debug_builder.create_subroutine_type(
            self.compile_unit.get_file(),
            return_type,
            parameter_types,
            DIFlags::PUBLIC,
        );
        let function: DISubprogram<'_> = self.debug_builder.create_function(
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
        func.set_subprogram(function);
        let scope = function.as_debug_info_scope();

        let arg_local_vars: Vec<DILocalVariable<'ctx>> = parameter_types
            .iter()
            .enumerate()
            .map(|(i, param)| {
                self.debug_builder.create_parameter_variable(
                    scope,
                    &i.to_string(),
                    i as u32 + 1,
                    self.compile_unit.get_file(),
                    self.get_line(),
                    *param,
                    true,
                    DIFlags::ZERO,
                )
            })
            .collect();

        let location = self.debug_location(Some(scope));

        let info = FunctionDebugInfo {
            function,
            params: parameter_types.to_vec(),
            params_local_vars: arg_local_vars,
            location,
            scope,
        };

        self.functions.insert(func_name.to_string(), info.clone());
        info
    }

    pub fn create_local_variable(
        &self,
        name: &str,
        scope: DIScope<'ctx>,
        ty: DIType<'ctx>,
        arg: Option<u32>,
    ) -> DILocalVariable<'ctx> {
        self.debug_builder.create_parameter_variable(
            scope,
            name,
            arg.unwrap_or(0),
            self.compile_unit.get_file(),
            self.get_line(),
            ty,
            true,
            DIFlags::ZERO,
        )
    }

    /// Creates local variables for a function call.
    pub fn create_function_call_local_vars(
        &self,
        func: &FunctionDebugInfo<'ctx>,
        scope: DIScope<'ctx>,
    ) -> Vec<DILocalVariable<'ctx>> {
        func.params
            .iter()
            .enumerate()
            .map(|(i, param)| {
                self.debug_builder.create_parameter_variable(
                    scope,
                    &i.to_string(),
                    0, /* must be 0 here, since its not a subprogram parameter, but a variable passed to the
                        * subprogram. */
                    self.compile_unit.get_file(),
                    self.get_line(),
                    *param,
                    true,
                    DIFlags::ZERO,
                )
            })
            .collect()
    }

    /// Inserts a variable value debug info before the instruction given.
    pub fn insert_dbg_value(
        &self,
        value: BasicValueEnum<'ctx>,
        local_var: DILocalVariable<'ctx>,
        loc: DILocation<'ctx>,
        inst: InstructionValue<'ctx>,
    ) {
        self.debug_builder.insert_dbg_value_before(value, local_var, None, loc, inst);
    }

    /// Creates a type debug info by name.
    pub fn create_type(&mut self, id: u64, name: &str, size_in_bits: u64) -> DIType<'ctx> {
        dbg!(size_in_bits);
        // 5 = signed integer https://llvm.org/docs/LangRef.html#dibasictype
        // lldb wont print the value if its more than 128 bits...
        let debug_type =
            self.debug_builder.create_basic_type(name, size_in_bits.min(128), 5, DIFlags::ZERO).unwrap().as_type();

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
        self.struct_types_by_id.insert(id, fields.to_vec());
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
