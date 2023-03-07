use std::collections::HashSet;
use std::iter;

use cairo_lang_sierra::ids::{ConcreteTypeId, UserTypeId};
use cairo_lang_sierra::program::{GenericArg, TypeDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn sierra_enum(&mut self, type_declaration: &TypeDeclaration) {
        // The first argument of an enum is a user type for its name
        let type_name = get_enum_name(type_declaration);
        // This will be stored in the resulting struct as an index, for which we need a type
        let index_type_width = get_index_width(type_declaration);
        let index_type = self.context.custom_width_int_type(index_type_width).as_basic_type_enum();
        // The index type needs to be saved for later so that it can be used by enum init and enum match
        if let GenericArg::UserType(UserTypeId { id: _id, debug_name }) = &type_declaration.long_id.generic_args[0] {
            let index_type_name = format!("ut@{}", debug_name.clone().expect(DEBUG_NAME_EXPECTED));
            self.types_by_name.insert(index_type_name, index_type);
        } else {
            panic!("Expected first generic argument of enum to be user type");
        }

        // The struct representation used in the ir only needs one entry for each unique subtype
        let unique_arg_type_ids = HashSet::<_>::from_iter(get_arg_type_ids(type_declaration));

        // Now that we have the unique type ids, we can get the associated types and debug information, and
        // package them with the index into a struct
        let unique_arg_types = unique_arg_type_ids.iter().map(|type_id| {
            self.types_by_id.get(type_id).expect("Type should have been defined before enum").as_basic_type_enum()
        });
        let unique_arg_debug_types = unique_arg_type_ids
            .iter()
            .map(|type_id| *self.debug.types_by_id.get(type_id).expect("Type should have been defined before enum"));
        let struct_args: Vec<_> = iter::once(index_type).chain(unique_arg_types).collect();
        let debug_args: Vec<_> = unique_arg_debug_types.collect(); // TODO add index?
        let struct_type = self.context.struct_type(&struct_args, false);

        // We do also need to keep a record of every possible subtype in order so that enum_init and
        // enum_match can be implemented
        let enum_member_types = get_arg_type_ids(type_declaration)
            .map(|type_id| self.types_by_id.get(&type_id).expect("Type should have been defined before enum"))
            .cloned()
            .collect();

        self.types_by_id.insert(type_declaration.id.id, struct_type.as_basic_type_enum());
        self.types_by_name.insert(type_name.clone(), struct_type.as_basic_type_enum());
        self.debug.create_struct(type_declaration.id.id, &type_name, &struct_type, &debug_args);
        self.enum_member_types_by_id.insert(type_declaration.id.id, enum_member_types);
    }
}

fn get_arg_type_ids(type_declaration: &TypeDeclaration) -> impl Iterator<Item = u64> + '_ {
    type_declaration.long_id.generic_args[1..].iter().map(|arg| match arg {
        GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => *id,
        _ => panic!("Only 'type' expected for non-index enum args"),
    })
}

fn get_enum_name(type_declaration: &TypeDeclaration) -> String {
    if let GenericArg::UserType(UserTypeId { id: _id, debug_name }) = &type_declaration.long_id.generic_args[0] {
        debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string()
    } else {
        panic!("Attempted to use non-user type as enum name");
    }
}

// Calculates the number of bits required for the index type of the enum
// Always need at minimum 1 to have a sensible type
fn get_index_width(type_declaration: &TypeDeclaration) -> u32 {
    let arg_count = type_declaration.long_id.generic_args.len();

    if arg_count <= 3 {
        return 1;
    }

    (arg_count as f32 - 1.).log2().ceil() as u32
}
