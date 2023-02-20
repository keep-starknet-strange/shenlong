use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a duplication operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `dup<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    pub fn dup(&self, libfunc_declaration: &LibfuncDeclaration) {
        // dup<T> can only duplicate the type T. If several types need the dup instruction it'll be defined
        // multiple times. Ex: dup<felt>; dup<i128>;
        // Get the type that this dup function has to handle
        let arg_type = match &libfunc_declaration.long_id.generic_args[0] {
            // Panics if the type has not been declared.
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                self.types.get(&id.to_string()).unwrap().as_basic_type_enum()
            }
            // Not sure if dup can dup user defined types
            GenericArg::UserType(_) => todo!(),
            _ => panic!("Dup only takes type or user type"),
        };
        // Return a 2 element struct that have the same value. Ex: dup<felt>(1) -> { i252 1, i252 1 }
        let return_type = self.context.struct_type(&[arg_type, arg_type], false);
        // fn dup<T>(a: T) -> {T, T}
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            return_type.fn_type(&[arg_type.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // We just defined dup to have an input parameter so it shouldn't panic.
        let arg = func.get_first_param().unwrap();
        // Use the struct as a tuple.
        let tuple = self.builder.build_alloca(return_type, "res_ptr");
        // Get a pointer to the first field address.
        let tuple_ptr = self.builder.build_struct_gep(return_type, tuple, 0, "tuple_ptr").unwrap();
        // Store the value in the struct.
        self.builder.build_store(tuple_ptr, arg);
        // Same for second field.
        let tuple_ptr_2 = self.builder.build_struct_gep(return_type, tuple, 1, "tuple_ptr_2").unwrap();
        self.builder.build_store(tuple_ptr_2, arg);
        self.builder.build_return(Some(&self.builder.build_load(return_type, tuple, "res")));
    }
}
