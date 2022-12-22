window.SIDEBAR_ITEMS = {"enum":[["CompilationState","Compilation state. This is used to keep track of the current compilation state. The reason is that the compilation process is split into multiple steps. The state will be used to implement a state machine that will keep track of the current compilation step. Goal is to ensure consistency in the order of the compilation steps. This is important because the compilation steps are not independent. For example, the type declarations must be processed before the statements. The state machine will ensure that the compilation steps are executed in the correct order. The state machine will also ensure that the compilation steps are executed only once."]],"struct":[["Compiler","Compiler is the main entry point for the LLVM backend. It is responsible for compiling a Sierra program to LLVM IR."]],"type":[["CompilationStateTransition",""]]};