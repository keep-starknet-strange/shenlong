use inkwell::types::BasicTypeEnum;
use tracing::debug;

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process types in the Sierra program.
    /// For each type declaration in the Sierra program, create a corresponding type in the LLVM
    /// context.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra types fails.
    pub fn process_types(&mut self) -> CompilerResult<()> {
        debug!("processing types");

        // Check that the current state is valid.
        self.check_state(&CompilationState::NotStarted)?;
        for type_declaration in self.program.type_declarations.iter() {
            self.current_line_estimate += 1;
            // All those types are known in advance. A struct is a combination of multiple "primitive" types
            let type_name = type_declaration.long_id.generic_id.0.as_str();
            match type_name {
                // Regular felt
                "felt" => self.felt(type_declaration),
                // NonZero<felt> is a felt != 0
                "NonZero" => self.non_zero(type_declaration),
                // Box<T>
                "Box" => self.sierra_box(type_declaration),
                // Regular struct
                "Struct" => self.sierra_struct(type_declaration),
                _ => debug!(type_name, "unimplemented type"),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::TypesProcessed)
    }

    /// Returns the LLVM IR type from the type name
    ///
    /// # Arguments
    ///
    /// * `type_name` - The type name (ex: "felt")
    ///
    /// # Returns
    ///
    /// The LLVM IR type.
    pub fn get_type_from_name(&self, type_name: &str) -> CompilerResult<BasicTypeEnum<'ctx>> {
        // id_from_name : name => id
        // variables : id => LLVM IR type
        // ex: return variables[id_from_name["felt"]]
        Ok(self
            .types
            .get(self.id_from_name.get(type_name).ok_or(CompilerError::TypeNotFound(type_name.to_owned()))?)
            .ok_or(CompilerError::TypeNotFound(type_name.to_owned()))?
            .as_basic_type_enum())
    }

    // Returns the type id mapped from the debug name.
    pub fn get_type_id_from_name(&self, type_name: &str) -> CompilerResult<&String> {
        self.id_from_name.get(type_name).ok_or(CompilerError::TypeNotFound(type_name.to_owned()))
    }
}
