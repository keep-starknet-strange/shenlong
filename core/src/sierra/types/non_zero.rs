use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Declares `NonZero<T>`. A NonZero value is a number that is not 0. It should be checked like
    /// that in cairo:
    ///
    /// ```cairo
    ///  let res = felt_is_zero(felt);
    /// match res {
    ///     IsZeroResult::Zero(()) => Option::<NonZero::<felt>>::None(()),
    ///     IsZeroResult::NonZero(val) => Option::<NonZero::<felt>>::Some(val),
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `type_declaration` - the sierra type declaration.
    pub fn non_zero(&mut self, type_declaration: &TypeDeclaration) {
        match &type_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                // A type can't use an undefined type so it should be declared before so it shouldn't panic.
                // The NonZero type doesn't really make sense in LLVM IR (it does in sierra to make sure that
                // everything is provable but in LLVM IR we're not proving anything so we can consider `NonZero<T>`
                // to be just `T`).
                let inner_type = *self.types_by_id.get(id).unwrap();

                self.types_by_id.insert(type_declaration.id.id, inner_type);
                self.types_by_name.insert(type_declaration.id.debug_name.as_ref().unwrap().to_string(), inner_type);
            }
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("NonZero only takes type or user type")
            }
        };
    }
}
