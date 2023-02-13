use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a struct deconstruction operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of StructDeconstruct<T>.
    ///
    /// # Error
    ///
    /// Returns an error if the type T has not been declared previously.
    pub fn struct_deconstruct(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        // Type of the struct that we have to deconstruct.
        let args = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => self
                .types
                .get(&id.to_string())
                .expect("Type should have been defined before struct")
                .as_basic_type_enum()
                .into_struct_type(),
            _val => {
                panic!("Struct deconstruct only takes predefined structs")
            }
        };
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            args.fn_type(&[args.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        // Only returns the struct. Can be treated in the statements.
        self.builder
            .build_return(Some(&func.get_first_param().expect("This functions should take exactly 1 argument ")));
    }
}
