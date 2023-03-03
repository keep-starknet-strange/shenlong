use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a struct deconstruction operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `StructDeconstruct<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    /// Panics if the sierra file doesn't have the debug infos.
    pub fn struct_deconstruct(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();
        // Type of the struct that we have to deconstruct.

        let type_id = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => *id,
            _val => {
                panic!("Struct deconstruct only takes predefined structs")
            }
        };

        let args = self.types_by_id.get(&type_id).unwrap().into_struct_type();
        let debug_arg = *self.debug.types_by_id.get(&type_id).unwrap();

        let func = self.module.add_function(func_name, args.fn_type(&[args.into()], false), None);

        let debug_func = self.debug.create_function(func_name, &func, Some(debug_arg), &[debug_arg], None);

        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        // Only returns the struct. Can be treated in the statements.
        let inst = self.builder.build_return(Some(&func.get_first_param().unwrap()));

        // Debug values
        let debug_local_var = self.debug.create_local_variable(func_name, debug_func.scope, debug_arg, None);
        self.debug.insert_dbg_value(
            func.get_first_param().unwrap(),
            debug_local_var,
            self.builder.get_current_debug_location().unwrap(),
            inst,
        );
    }
}
