use inkwell::debug_info::{AsDIScope, DIFlags, DIFlagsConstants, DIScope, DISubprogram, DIType};
use inkwell::values::FunctionValue;

use super::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Util method to create and set the current debug location.
    #[inline]
    pub fn debug_location(&self, scope: Option<DIScope<'ctx>>) {
        if let Some(dibuilder) = &self.dibuilder {
            let compile_unit = self.compile_unit.unwrap();
            let location = dibuilder.create_debug_location(
                self.context,
                self.current_line_estimate,
                0,
                scope.unwrap_or_else(|| compile_unit.as_debug_info_scope()),
                None,
            );
            self.builder.set_current_debug_location(location);
        }
    }

    /// Utility method to generate debug info for a function.
    ///
    /// Sets the current debug location too, so you don't have to call debug_location with the
    /// returned scope.
    pub fn create_function_debug(
        &self,
        func_name: &str,
        func: &FunctionValue<'ctx>,
        return_type_id: &str,
        parameter_type_ids: &[String],
    ) -> Option<DIScope<'ctx>> {
        // todo: maybe use functiontype here and dont requrie type ids?
        if let Some(dibuilder) = &self.dibuilder {
            let ditypes = self.ditypes.as_ref().unwrap();
            let compile_unit = self.compile_unit.unwrap();

            let return_type = ditypes.get(return_type_id).unwrap();
            let parameter_types: Vec<_> = parameter_type_ids.iter().map(|id| *ditypes.get(id).unwrap()).collect();

            let subroutine_type = dibuilder.create_subroutine_type(
                compile_unit.get_file(),
                Some(*return_type),
                &parameter_types,
                DIFlags::PUBLIC,
            );
            let func_scope: DISubprogram<'_> = dibuilder.create_function(
                compile_unit.as_debug_info_scope(),
                func_name,
                None,
                compile_unit.get_file(),
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
            Some(scope)
        } else {
            None
        }
    }

    /// Gets (or creates if it doesn't exist) a type debug info.
    pub fn basic_type_debug_info(&mut self, name: &str, size_in_bits: u64) -> Option<DIType<'ctx>> {
        if let Some(dibuilder) = &self.dibuilder {
            let ditypes = self.ditypes.as_mut().unwrap();
            Some(*ditypes.entry(name.to_string()).or_insert_with(|| {
                dibuilder
                    .create_basic_type(name, size_in_bits, 0x00, inkwell::debug_info::DIFlags::PUBLIC)
                    .unwrap()
                    .as_type()
            }))
        } else {
            None
        }
    }
}
