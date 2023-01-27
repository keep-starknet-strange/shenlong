use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn dup(&mut self, libfunc_declaration: &LibfuncDeclaration) -> CompilerResult<()> {
        let arg_type = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                self.types.get(id).expect("Dup type should have been declared").as_basic_type_enum()
            }
            GenericArg::UserType(_) => todo!(),
            _ => panic!("Dup only takes type or user type"),
        };
        let return_type = self.context.struct_type(&[arg_type, arg_type], false);
        let func = self.module.add_function(
            libfunc_declaration.id.id.to_string().as_str(),
            return_type.fn_type(&[arg_type.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        let arg = func.get_first_param().expect("Drop function should have an input parameter");
        let tuple = self.builder.build_alloca(return_type, "res_ptr");
        let tuple_ptr =
            self.builder.build_struct_gep(tuple, 0, "tuple_ptr").expect("Pointer should be valid");
        self.builder.build_store(tuple_ptr, arg);
        let tuple_ptr_2 = self
            .builder
            .build_struct_gep(tuple, 1, "tuple_ptr_2")
            .expect("Pointer2 should be valid");
        self.builder.build_store(tuple_ptr_2, arg);
        self.builder.build_return(Some(&self.builder.build_load(tuple, "res")));
        Ok(())
    }
}
