var searchIndex = JSON.parse('{\
"shenlong":{"doc":"Shenlong is a library for building LLVM IR from Cairo.","t":"AAAAEGRNNNNNNNNNNNLLLLLLLLLLLLLLLEGDNNNNNNMLLLLMLLLLLLLMLLLLLLLLLLLLLLMLLLLLMMMLLLLLLLLMLLLLMLLLLLLLLLLMLLLMM","n":["core","sierra","errors","llvm_compiler","CompilerError","CompilerResult","DEBUG_NAME_EXPECTED","FuncNotFound","InvalidState","InvalidStateTransition","LlvmPrintError","NoDebugName","NoReturnType","NoReturnValue","NoTypeProvided","PathNotFound","TypeNotFound","VarNotFound","borrow","borrow_mut","fmt","fmt","from","from","from","into","provide","source","to_string","try_from","try_into","type_id","upcast","CompilationState","CompilationStateTransition","Compiler","CoreLibFunctionsProcessed","Finalized","FunctionsProcessed","NotStarted","StatementsProcessed","TypesProcessed","basic_blocks","borrow","borrow","borrow_mut","borrow_mut","builder","check_state","clone","clone_into","collect_jumps","compile_from_code","compile_from_file","compile_sierra_program_to_llvm","context","dup","eq","equivalent","felt","felt_add","felt_const","felt_is_zero","felt_mul","felt_sub","fmt","from","from","get_type_from_name","hash","id_from_name","init_state_transitions","into","into","into_box","jump","jump_dests","llvm_output_path","module","modulo","move_to","non_zero","printf","process_core_lib_functions","process_funcs","process_statements_from","process_types","program","rename","sierra_box","sierra_struct","state","state","store_temp","struct_construct","struct_deconstruct","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","types","unbox","upcast","upcast","valid_state_transitions","variables"],"q":["shenlong","shenlong::core","shenlong::core::sierra","","shenlong::core::sierra::errors","","","","","","","","","","","","","","","","","","","","","","","","","","","","","shenlong::core::sierra::llvm_compiler","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["The core library of Shenlong compiler.","Sierra related modules.","","LLVM compiler.","Compiler errors.","","","LLVM IR function not found","Compiler state doesn’t exist.","Invalid compiler state transition.","LLVM IR Error (verify or write to file).","Object has no debug name.","LLVM IR function has no return type.","LLVM IR function has no return value.","No LLVM IR type was provided.","File or folder not found.","LLVM IR type not found.","LLVM IR variable not found.","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","Compilation state. This is used to keep track of the …","A compilation state transition. This is a tuple of two …","Compiler is the main entry point for the LLVM backend. It …","The core library functions have been processed.","The compilation has been finalized. This is the final …","The functions have been processed.","The compilation has not started yet.","The statements have been processed.","The types have been processed.","Calls in the main function.","","","","","The LLVM builder.","Check if the compilation is in a valid state. If the …","","","Collect all the jump destinations in the sierra program. …","Compile a Sierra program code to LLVM IR.","Compile a Sierra program file to LLVM IR.","Compiles a Sierra <code>Program</code> representation to LLVM IR.","The LLVM context.","Implementation of the LLVM IR conversion of a duplication …","","","","Implementation of the LLVM IR conversion of a felt …","Implementation of the LLVM IR conversion of a felt …","Implementation of the LLVM IR conversion of felt == 0.","Implementation of the LLVM IR conversion of a felt …","Implementation of the LLVM IR conversion of a felt …","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the LLVM IR type from the type name","","Mapping from type name to program id.","Initialize valid state transitions.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Implementation of the LLVM IR conversion of into_box.","Implementation of the LLVM IR conversion of felt == 0.","","The LLVM IR output path.","The LLVM module.","Implementation of the LLVM IR conversion of a felt …","Move the compilation state to the next state.","","Implementation of the LLVM IR wrapper for the printf …","Process core library functions in the Sierra program.","Process types in the Sierra program. For each type …","Process statements in the Sierra program.","Process types in the Sierra program. For each type …","The Sierra program.","Implementation of the LLVM IR conversion of the rename …","","","Get the current compilation state.","The current compilation state.","Implementation of the LLVM IR conversion of a store_temp …","Implementation of the LLVM IR conversion of a struct …","Implementation of the LLVM IR conversion of a struct …","","","","","","","","The types.","Implementation of the LLVM IR conversion of unbox.","","","The valid state transitions.","The variables of the program."],"i":[0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,13,13,13,13,13,13,12,12,13,12,13,12,12,13,13,12,12,12,12,12,12,13,13,12,12,12,12,12,12,13,12,13,12,13,12,12,12,13,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,12,13,12,13,12,13,12,12,12,13,12,12],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[1,2],[[4,[3]]]],[[1,2],[[4,[3]]]],[5,1],[6,1],[[]],[[]],[7],[1,[[9,[8]]]],[[],10],[[],4],[[],4],[[],11],[[]],0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],0,[[12,13],[[4,[1]]]],[13,13],[[]],[12],[[14,15,[9,[14]]],[[4,[1]]]],[[15,15,[9,[14]]],[[4,[1]]]],[[16,15,[9,[14]]],[[4,[1]]]],0,[[12,17]],[[13,13],18],[[],18],[[12,19]],[[12,17]],[[12,17]],[[12,20,21],[[4,[1]]]],[[12,17]],[[12,17]],[[13,2],[[4,[3]]]],[[]],[[]],[[12,14],[[4,[22,1]]]],[13],0,[[],[[24,[18,23]]]],[[]],[[]],[[12,17]],[[12,21]],0,0,0,[12],[[12,13],[[4,[1]]]],[[12,19]],[[12,25]],[12,[[4,[1]]]],[12,[[4,[1]]]],[[12,21],[[4,[1]]]],[12,[[4,[1]]]],0,[[12,17]],[[12,19]],[[12,19]],[12,13],0,[[12,17]],[[12,17]],[[12,17]],[[]],[[],4],[[],4],[[],4],[[],4],[[],11],[[],11],0,[[12,17]],[[]],[[]],0,0],"p":[[4,"CompilerError"],[3,"Formatter"],[3,"Error"],[4,"Result"],[3,"LLVMString"],[3,"Error"],[3,"Demand"],[8,"Error"],[4,"Option"],[3,"String"],[3,"TypeId"],[3,"Compiler"],[4,"CompilationState"],[15,"str"],[3,"Path"],[3,"Program"],[3,"LibfuncDeclaration"],[15,"bool"],[3,"TypeDeclaration"],[3,"GenInvocation"],[15,"usize"],[4,"BasicTypeEnum"],[3,"RandomState"],[3,"HashMap"],[3,"StructType"]]},\
"shenlong_cli":{"doc":"Command line interface of Shenlong. Shenlong is a library …","t":"AADENNDELLLLLLLLLLLLLLLLLLMMLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMHH","n":["cli","emoji","Command","Commands","CompileToLlvm","Sierra","SierraCommands","SierraSubCommands","augment_args","augment_args","augment_args_for_update","augment_args_for_update","augment_subcommands","augment_subcommands","augment_subcommands_for_update","augment_subcommands_for_update","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","command","command","command","command","command_for_update","command_for_update","config","fmt","fmt","from","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","group_id","group_id","has_subcommand","has_subcommand","into","into","into","into","run","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","upcast","upcast","upcast","upcast","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","llvm_output_path","program_path","target_triple","CHECK_MARK_BUTTON","SPARKLE"],"q":["shenlong_cli","","shenlong_cli::cli","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","shenlong_cli::cli::SierraSubCommands","","","shenlong_cli::emoji",""],"d":["","","Shenlong command line interface.","List of supported commands.","Compiles a Sierra program to LLVM IR.","Ethereum related subcommands","Sierra related commands.","Sierra related subcommands.","","","","","","","","","","","","","","","","","","","List of supported commands.","Ethereum related subcommands.","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Main entry point for the Shenlong command line interface.","","","","","","","","","","","","","","","","","","","","","","","","","The path to the output LLVM IR file. If not specified, the …","The path to the Sierra program to compile.","The target triple, e.g x86_64-pc-linux-gnu","",""],"i":[0,0,0,0,5,10,0,0,7,2,7,2,10,5,10,5,7,10,2,5,7,10,2,5,7,2,7,2,7,2,7,2,5,7,10,2,5,7,10,2,5,7,10,2,5,7,2,10,5,7,10,2,5,7,7,10,2,5,7,10,2,5,7,10,2,5,7,10,2,5,7,10,2,5,7,10,2,5,17,17,17,0,0],"f":[0,0,0,0,0,0,0,0,[1,1],[1,1],[1,1],[1,1],[1,1],[1,1],[1,1],[1,1],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],1],[[],1],0,0,[[],1],[[],1],0,[[2,3],4],[[5,3],4],[[]],[[]],[[]],[[]],[6,[[9,[7,8]]]],[6,[[9,[10,8]]]],[6,[[9,[2,8]]]],[6,[[9,[5,8]]]],[6,[[9,[7,8]]]],[6,[[9,[10,8]]]],[6,[[9,[2,8]]]],[6,[[9,[5,8]]]],[[],[[12,[11]]]],[[],[[12,[11]]]],[13,14],[13,14],[[]],[[]],[[]],[[]],[7,15],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],16],[[],16],[[],16],[[],16],[[]],[[]],[[]],[[]],[[7,6],[[9,[8]]]],[[10,6],[[9,[8]]]],[[2,6],[[9,[8]]]],[[5,6],[[9,[8]]]],[[7,6],[[9,[8]]]],[[10,6],[[9,[8]]]],[[2,6],[[9,[8]]]],[[5,6],[[9,[8]]]],0,0,0,0,0],"p":[[3,"Command"],[3,"SierraCommands"],[3,"Formatter"],[6,"Result"],[4,"SierraSubCommands"],[3,"ArgMatches"],[3,"Command"],[6,"Error"],[4,"Result"],[4,"Commands"],[3,"Id"],[4,"Option"],[15,"str"],[15,"bool"],[6,"Result"],[3,"TypeId"],[13,"CompileToLlvm"]]},\
"shenlong_core":{"doc":"","t":"AAAEGRNNNNNNNNNNNLLLLLLLLLLLLLLLEGDNNNNNNMLLLLMLLLLLLLMLLLLLLLLLLLLLLMLLLLLMMMLLLLLLLLMLLLLMLLLLLLLLLLMLLLMM","n":["sierra","errors","llvm_compiler","CompilerError","CompilerResult","DEBUG_NAME_EXPECTED","FuncNotFound","InvalidState","InvalidStateTransition","LlvmPrintError","NoDebugName","NoReturnType","NoReturnValue","NoTypeProvided","PathNotFound","TypeNotFound","VarNotFound","borrow","borrow_mut","fmt","fmt","from","from","from","into","provide","source","to_string","try_from","try_into","type_id","upcast","CompilationState","CompilationStateTransition","Compiler","CoreLibFunctionsProcessed","Finalized","FunctionsProcessed","NotStarted","StatementsProcessed","TypesProcessed","basic_blocks","borrow","borrow","borrow_mut","borrow_mut","builder","check_state","clone","clone_into","collect_jumps","compile_from_code","compile_from_file","compile_sierra_program_to_llvm","context","dup","eq","equivalent","felt","felt_add","felt_const","felt_is_zero","felt_mul","felt_sub","fmt","from","from","get_type_from_name","hash","id_from_name","init_state_transitions","into","into","into_box","jump","jump_dests","llvm_output_path","module","modulo","move_to","non_zero","printf","process_core_lib_functions","process_funcs","process_statements_from","process_types","program","rename","sierra_box","sierra_struct","state","state","store_temp","struct_construct","struct_deconstruct","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","types","unbox","upcast","upcast","valid_state_transitions","variables"],"q":["shenlong_core","shenlong_core::sierra","","shenlong_core::sierra::errors","","","","","","","","","","","","","","","","","","","","","","","","","","","","","shenlong_core::sierra::llvm_compiler","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Sierra related modules.","","LLVM compiler.","Compiler errors.","","","LLVM IR function not found","Compiler state doesn’t exist.","Invalid compiler state transition.","LLVM IR Error (verify or write to file).","Object has no debug name.","LLVM IR function has no return type.","LLVM IR function has no return value.","No LLVM IR type was provided.","File or folder not found.","LLVM IR type not found.","LLVM IR variable not found.","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","Compilation state. This is used to keep track of the …","A compilation state transition. This is a tuple of two …","Compiler is the main entry point for the LLVM backend. It …","The core library functions have been processed.","The compilation has been finalized. This is the final …","The functions have been processed.","The compilation has not started yet.","The statements have been processed.","The types have been processed.","Calls in the main function.","","","","","The LLVM builder.","Check if the compilation is in a valid state. If the …","","","Collect all the jump destinations in the sierra program. …","Compile a Sierra program code to LLVM IR.","Compile a Sierra program file to LLVM IR.","Compiles a Sierra <code>Program</code> representation to LLVM IR.","The LLVM context.","Implementation of the LLVM IR conversion of a duplication …","","","","Implementation of the LLVM IR conversion of a felt …","Implementation of the LLVM IR conversion of a felt …","Implementation of the LLVM IR conversion of felt == 0.","Implementation of the LLVM IR conversion of a felt …","Implementation of the LLVM IR conversion of a felt …","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the LLVM IR type from the type name","","Mapping from type name to program id.","Initialize valid state transitions.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Implementation of the LLVM IR conversion of into_box.","Implementation of the LLVM IR conversion of felt == 0.","","The LLVM IR output path.","The LLVM module.","Implementation of the LLVM IR conversion of a felt …","Move the compilation state to the next state.","","Implementation of the LLVM IR wrapper for the printf …","Process core library functions in the Sierra program.","Process types in the Sierra program. For each type …","Process statements in the Sierra program.","Process types in the Sierra program. For each type …","The Sierra program.","Implementation of the LLVM IR conversion of the rename …","","","Get the current compilation state.","The current compilation state.","Implementation of the LLVM IR conversion of a store_temp …","Implementation of the LLVM IR conversion of a struct …","Implementation of the LLVM IR conversion of a struct …","","","","","","","","The types.","Implementation of the LLVM IR conversion of unbox.","","","The valid state transitions.","The variables of the program."],"i":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,13,13,13,13,13,13,12,12,13,12,13,12,12,13,13,12,12,12,12,12,12,13,13,12,12,12,12,12,12,13,12,13,12,13,12,12,12,13,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,12,13,12,13,12,13,12,12,12,13,12,12],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[1,2],3],[[1,2],3],[4,1],[[]],[5,1],[[]],[6],[1,[[8,[7]]]],[[],9],[[],10],[[],10],[[],11],[[]],0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],0,[[12,13],14],[13,13],[[]],[12],[[15,16,[8,[15]]],14],[[16,16,[8,[15]]],14],[[17,16,[8,[15]]],14],0,[[12,18]],[[13,13],19],[[],19],[[12,20]],[[12,18]],[[12,18]],[[12,21,22],14],[[12,18]],[[12,18]],[[13,2],3],[[]],[[]],[[12,15],[[14,[23]]]],[13],0,[[],[[24,[19]]]],[[]],[[]],[[12,18]],[[12,22]],0,0,0,[12],[[12,13],14],[[12,20]],[[12,25]],[12,14],[12,14],[[12,22],14],[12,14],0,[[12,18]],[[12,20]],[[12,20]],[12,13],0,[[12,18]],[[12,18]],[[12,18]],[[]],[[],10],[[],10],[[],10],[[],10],[[],11],[[],11],0,[[12,18]],[[]],[[]],0,0],"p":[[4,"CompilerError"],[3,"Formatter"],[6,"Result"],[3,"LLVMString"],[3,"Error"],[3,"Demand"],[8,"Error"],[4,"Option"],[3,"String"],[4,"Result"],[3,"TypeId"],[3,"Compiler"],[4,"CompilationState"],[6,"CompilerResult"],[15,"str"],[3,"Path"],[3,"Program"],[3,"LibfuncDeclaration"],[15,"bool"],[3,"TypeDeclaration"],[6,"Invocation"],[15,"usize"],[4,"BasicTypeEnum"],[3,"HashMap"],[3,"StructType"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};