use inkwell::debug_info::{AsDIScope, DIFlags, DIFlagsConstants, DIScope, DISubprogram, DIType};
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
        return_type_id: Option<&str>,
        parameter_type_ids: &[&str],
    ) -> DIScope<'ctx> {
        let return_type = return_type_id.map(|id| self.get_debug_type(id));
        let parameter_types: Vec<_> = parameter_type_ids.iter().map(|id| self.get_debug_type(id)).collect();

        let subroutine_type = self.dibuilder.create_subroutine_type(
            self.compile_unit.get_file(),
            return_type,
            &parameter_types,
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

    /// Returns the debug type.
    ///
    /// Panics if it doesn't exist yet.
    pub fn get_debug_type(&self, name: &str) -> DIType<'ctx> {
        self.ditypes.get(name).cloned().expect("type should exist")
    }

    /// Creates a type debug info.
    pub fn create_debug_type(&mut self, name: &str, size_in_bits: u64) -> DIType<'ctx> {
        *self.ditypes.entry(name.to_string()).or_insert_with(|| {
            self.dibuilder
                .create_basic_type(name, size_in_bits, 0x00, inkwell::debug_info::DIFlags::PUBLIC)
                .unwrap()
                .as_type()
        })
    }
}
