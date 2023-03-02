; ModuleID = 'root'
source_filename = "root"

declare i32 @printf(ptr, ...)

define void @print_felt(i253 %0) !dbg !3 {
entry:
  %rounded_size_val = zext i253 %0 to i256, !dbg !9
  %shifted = lshr i256 %rounded_size_val, 224, !dbg !9
  %print_value_trunc = trunc i256 %shifted to i32, !dbg !9
  %format = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format, align 1, !dbg !9
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc), !dbg !9
  %shifted1 = lshr i256 %rounded_size_val, 192, !dbg !9
  %print_value_trunc2 = trunc i256 %shifted1 to i32, !dbg !9
  %format3 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format3, align 1, !dbg !9
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2), !dbg !9
  %shifted5 = lshr i256 %rounded_size_val, 160, !dbg !9
  %print_value_trunc6 = trunc i256 %shifted5 to i32, !dbg !9
  %format7 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format7, align 1, !dbg !9
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6), !dbg !9
  %shifted9 = lshr i256 %rounded_size_val, 128, !dbg !9
  %print_value_trunc10 = trunc i256 %shifted9 to i32, !dbg !9
  %format11 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format11, align 1, !dbg !9
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10), !dbg !9
  %shifted13 = lshr i256 %rounded_size_val, 96, !dbg !9
  %print_value_trunc14 = trunc i256 %shifted13 to i32, !dbg !9
  %format15 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format15, align 1, !dbg !9
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14), !dbg !9
  %shifted17 = lshr i256 %rounded_size_val, 64, !dbg !9
  %print_value_trunc18 = trunc i256 %shifted17 to i32, !dbg !9
  %format19 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format19, align 1, !dbg !9
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18), !dbg !9
  %shifted21 = lshr i256 %rounded_size_val, 32, !dbg !9
  %print_value_trunc22 = trunc i256 %shifted21 to i32, !dbg !9
  %format23 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format23, align 1, !dbg !9
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22), !dbg !9
  %shifted25 = lshr i256 %rounded_size_val, 0, !dbg !9
  %print_value_trunc26 = trunc i256 %shifted25 to i32, !dbg !9
  %format27 = alloca [5 x i8], align 1, !dbg !9
  store [5 x i8] c"%08X\00", ptr %format27, align 1, !dbg !9
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26), !dbg !9
  %format29 = alloca [2 x i8], align 1, !dbg !9
  store [2 x i8] c"\0A\00", ptr %format29, align 1, !dbg !9
  %chars_printed30 = call i32 (ptr, ...) @printf(ptr %format29), !dbg !9
  ret void, !dbg !9
}

define void @print_double_felt(i512 %0) !dbg !10 {
entry:
  %shifted = lshr i512 %0, 480, !dbg !16
  %print_value_trunc = trunc i512 %shifted to i32, !dbg !16
  %format = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format, align 1, !dbg !16
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc), !dbg !16
  %shifted1 = lshr i512 %0, 448, !dbg !16
  %print_value_trunc2 = trunc i512 %shifted1 to i32, !dbg !16
  %format3 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format3, align 1, !dbg !16
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2), !dbg !16
  %shifted5 = lshr i512 %0, 416, !dbg !16
  %print_value_trunc6 = trunc i512 %shifted5 to i32, !dbg !16
  %format7 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format7, align 1, !dbg !16
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6), !dbg !16
  %shifted9 = lshr i512 %0, 384, !dbg !16
  %print_value_trunc10 = trunc i512 %shifted9 to i32, !dbg !16
  %format11 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format11, align 1, !dbg !16
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10), !dbg !16
  %shifted13 = lshr i512 %0, 352, !dbg !16
  %print_value_trunc14 = trunc i512 %shifted13 to i32, !dbg !16
  %format15 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format15, align 1, !dbg !16
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14), !dbg !16
  %shifted17 = lshr i512 %0, 320, !dbg !16
  %print_value_trunc18 = trunc i512 %shifted17 to i32, !dbg !16
  %format19 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format19, align 1, !dbg !16
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18), !dbg !16
  %shifted21 = lshr i512 %0, 288, !dbg !16
  %print_value_trunc22 = trunc i512 %shifted21 to i32, !dbg !16
  %format23 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format23, align 1, !dbg !16
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22), !dbg !16
  %shifted25 = lshr i512 %0, 256, !dbg !16
  %print_value_trunc26 = trunc i512 %shifted25 to i32, !dbg !16
  %format27 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format27, align 1, !dbg !16
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26), !dbg !16
  %shifted29 = lshr i512 %0, 224, !dbg !16
  %print_value_trunc30 = trunc i512 %shifted29 to i32, !dbg !16
  %format31 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format31, align 1, !dbg !16
  %chars_printed32 = call i32 (ptr, ...) @printf(ptr %format31, i32 %print_value_trunc30), !dbg !16
  %shifted33 = lshr i512 %0, 192, !dbg !16
  %print_value_trunc34 = trunc i512 %shifted33 to i32, !dbg !16
  %format35 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format35, align 1, !dbg !16
  %chars_printed36 = call i32 (ptr, ...) @printf(ptr %format35, i32 %print_value_trunc34), !dbg !16
  %shifted37 = lshr i512 %0, 160, !dbg !16
  %print_value_trunc38 = trunc i512 %shifted37 to i32, !dbg !16
  %format39 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format39, align 1, !dbg !16
  %chars_printed40 = call i32 (ptr, ...) @printf(ptr %format39, i32 %print_value_trunc38), !dbg !16
  %shifted41 = lshr i512 %0, 128, !dbg !16
  %print_value_trunc42 = trunc i512 %shifted41 to i32, !dbg !16
  %format43 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format43, align 1, !dbg !16
  %chars_printed44 = call i32 (ptr, ...) @printf(ptr %format43, i32 %print_value_trunc42), !dbg !16
  %shifted45 = lshr i512 %0, 96, !dbg !16
  %print_value_trunc46 = trunc i512 %shifted45 to i32, !dbg !16
  %format47 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format47, align 1, !dbg !16
  %chars_printed48 = call i32 (ptr, ...) @printf(ptr %format47, i32 %print_value_trunc46), !dbg !16
  %shifted49 = lshr i512 %0, 64, !dbg !16
  %print_value_trunc50 = trunc i512 %shifted49 to i32, !dbg !16
  %format51 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format51, align 1, !dbg !16
  %chars_printed52 = call i32 (ptr, ...) @printf(ptr %format51, i32 %print_value_trunc50), !dbg !16
  %shifted53 = lshr i512 %0, 32, !dbg !16
  %print_value_trunc54 = trunc i512 %shifted53 to i32, !dbg !16
  %format55 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format55, align 1, !dbg !16
  %chars_printed56 = call i32 (ptr, ...) @printf(ptr %format55, i32 %print_value_trunc54), !dbg !16
  %shifted57 = lshr i512 %0, 0, !dbg !16
  %print_value_trunc58 = trunc i512 %shifted57 to i32, !dbg !16
  %format59 = alloca [5 x i8], align 1, !dbg !16
  store [5 x i8] c"%08X\00", ptr %format59, align 1, !dbg !16
  %chars_printed60 = call i32 (ptr, ...) @printf(ptr %format59, i32 %print_value_trunc58), !dbg !16
  %format61 = alloca [2 x i8], align 1, !dbg !16
  store [2 x i8] c"\0A\00", ptr %format61, align 1, !dbg !16
  %chars_printed62 = call i32 (ptr, ...) @printf(ptr %format61), !dbg !16
  ret void, !dbg !16
}

define i253 @modulo(i512 %0) !dbg !17 {
entry:
  %modulus = srem i512 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481, !dbg !22
  %res = trunc i512 %modulus to i253, !dbg !22
  ret i253 %res, !dbg !22
}

define i253 @"felt_const<1>"() !dbg !23 {
entry:
  ret i253 1, !dbg !27
}

define i253 @"felt_const<2>"() !dbg !28 {
entry:
  ret i253 2, !dbg !29
}

define i253 @"store_temp<felt>"(i253 %0) !dbg !30 {
entry:
  ret i253 %0, !dbg !35
}

define i253 @felt_add(i253 %0, i253 %1) !dbg !36 {
entry:
  %extended_a = sext i253 %0 to i512, !dbg !42
  %extended_b = sext i253 %1 to i512, !dbg !42
  %res = add i512 %extended_a, %extended_b, !dbg !42
  %res1 = call i253 @modulo(i512 %res), !dbg !42
  ret i253 %res1, !dbg !42
}

define i253 @"rename<felt>"(i253 %0) !dbg !43 {
entry:
  ret i253 %0, !dbg !46
}

define void @print_return(i253 %0) !dbg !47 {
entry:
  %rounded_size_val = zext i253 %0 to i256, !dbg !50
  %shifted = lshr i256 %rounded_size_val, 224, !dbg !50
  %print_value_trunc = trunc i256 %shifted to i32, !dbg !50
  %format = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format, align 1, !dbg !50
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc), !dbg !50
  %shifted1 = lshr i256 %rounded_size_val, 192, !dbg !50
  %print_value_trunc2 = trunc i256 %shifted1 to i32, !dbg !50
  %format3 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format3, align 1, !dbg !50
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2), !dbg !50
  %shifted5 = lshr i256 %rounded_size_val, 160, !dbg !50
  %print_value_trunc6 = trunc i256 %shifted5 to i32, !dbg !50
  %format7 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format7, align 1, !dbg !50
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6), !dbg !50
  %shifted9 = lshr i256 %rounded_size_val, 128, !dbg !50
  %print_value_trunc10 = trunc i256 %shifted9 to i32, !dbg !50
  %format11 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format11, align 1, !dbg !50
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10), !dbg !50
  %shifted13 = lshr i256 %rounded_size_val, 96, !dbg !50
  %print_value_trunc14 = trunc i256 %shifted13 to i32, !dbg !50
  %format15 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format15, align 1, !dbg !50
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14), !dbg !50
  %shifted17 = lshr i256 %rounded_size_val, 64, !dbg !50
  %print_value_trunc18 = trunc i256 %shifted17 to i32, !dbg !50
  %format19 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format19, align 1, !dbg !50
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18), !dbg !50
  %shifted21 = lshr i256 %rounded_size_val, 32, !dbg !50
  %print_value_trunc22 = trunc i256 %shifted21 to i32, !dbg !50
  %format23 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format23, align 1, !dbg !50
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22), !dbg !50
  %shifted25 = lshr i256 %rounded_size_val, 0, !dbg !50
  %print_value_trunc26 = trunc i256 %shifted25 to i32, !dbg !50
  %format27 = alloca [5 x i8], align 1, !dbg !50
  store [5 x i8] c"%08X\00", ptr %format27, align 1, !dbg !50
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26), !dbg !50
  %format29 = alloca [2 x i8], align 1, !dbg !50
  store [2 x i8] c"\0A\00", ptr %format29, align 1, !dbg !50
  %chars_printed30 = call i32 (ptr, ...) @printf(ptr %format29), !dbg !50
  ret void, !dbg !50
}

define i32 @main() !dbg !51 {
entry:
  %0 = call i253 @"felt_const<1>"(), !dbg !52
  %1 = call i253 @"felt_const<2>"(), !dbg !53
  %2 = call i253 @"store_temp<felt>"(i253 %0), !dbg !54
  %3 = call i253 @felt_add(i253 %2, i253 %1), !dbg !55
  %4 = call i253 @"store_temp<felt>"(i253 %3), !dbg !56
  %5 = call i253 @"rename<felt>"(i253 %4), !dbg !57
  %ret_struct_ptr = alloca { i253 }, align 8, !dbg !58
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0, !dbg !58
  store i253 %5, ptr %field_0_ptr, align 4, !dbg !58
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4, !dbg !58
  %return_value_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0, !dbg !58
  %return_value = load i253, ptr %return_value_ptr, align 4, !dbg !58
  %format = alloca [15 x i8], align 1, !dbg !58
  store [15 x i8] c"Return value: \00", ptr %format, align 1, !dbg !58
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format), !dbg !58
  call void @print_return(i253 %return_value), !dbg !58
  ret i32 0, !dbg !58
}

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 2, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus, file: !2, producer: "shenlong", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "../examples/program.sierra", directory: "../examples")
!3 = distinct !DISubprogram(name: "print_felt", linkageName: "print_felt", scope: null, file: !2, line: 2, type: !4, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{null, !6}
!6 = !DIBasicType(name: "felt", size: 253, flags: DIFlagPublic)
!7 = !{!8}
!8 = !DILocalVariable(name: "0", arg: 1, scope: !3, file: !2, line: 2, type: !6, flags: DIFlagPublic)
!9 = !DILocation(line: 2, scope: !3)
!10 = distinct !DISubprogram(name: "print_double_felt", linkageName: "print_double_felt", scope: null, file: !2, line: 2, type: !11, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !14)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{null, !13}
!13 = !DIBasicType(name: "double_felt", size: 512, flags: DIFlagPublic)
!14 = !{!15}
!15 = !DILocalVariable(name: "0", arg: 1, scope: !10, file: !2, line: 2, type: !13, flags: DIFlagPublic)
!16 = !DILocation(line: 2, scope: !10)
!17 = distinct !DISubprogram(name: "modulo", linkageName: "modulo", scope: null, file: !2, line: 2, type: !18, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !20)
!18 = !DISubroutineType(flags: DIFlagPublic, types: !19)
!19 = !{!6, !13}
!20 = !{!21}
!21 = !DILocalVariable(name: "0", arg: 1, scope: !17, file: !2, line: 2, type: !13, flags: DIFlagPublic)
!22 = !DILocation(line: 2, scope: !17)
!23 = distinct !DISubprogram(name: "felt_const<1>", linkageName: "felt_const<1>", scope: null, file: !2, line: 3, type: !24, scopeLine: 3, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !26)
!24 = !DISubroutineType(flags: DIFlagPublic, types: !25)
!25 = !{!6}
!26 = !{}
!27 = !DILocation(line: 3, scope: !23)
!28 = distinct !DISubprogram(name: "felt_const<2>", linkageName: "felt_const<2>", scope: null, file: !2, line: 4, type: !24, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !26)
!29 = !DILocation(line: 4, scope: !28)
!30 = distinct !DISubprogram(name: "store_temp<felt>", linkageName: "store_temp<felt>", scope: null, file: !2, line: 5, type: !31, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !33)
!31 = !DISubroutineType(flags: DIFlagPublic, types: !32)
!32 = !{!6, !6}
!33 = !{!34}
!34 = !DILocalVariable(name: "0", arg: 1, scope: !30, file: !2, line: 5, type: !6, flags: DIFlagPublic)
!35 = !DILocation(line: 5, scope: !30)
!36 = distinct !DISubprogram(name: "felt_add", linkageName: "felt_add", scope: null, file: !2, line: 6, type: !37, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !39)
!37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
!38 = !{!6, !6, !6}
!39 = !{!40, !41}
!40 = !DILocalVariable(name: "0", arg: 1, scope: !36, file: !2, line: 6, type: !6, flags: DIFlagPublic)
!41 = !DILocalVariable(name: "1", arg: 2, scope: !36, file: !2, line: 6, type: !6, flags: DIFlagPublic)
!42 = !DILocation(line: 6, scope: !36)
!43 = distinct !DISubprogram(name: "rename<felt>", linkageName: "rename<felt>", scope: null, file: !2, line: 7, type: !31, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !44)
!44 = !{!45}
!45 = !DILocalVariable(name: "0", arg: 1, scope: !43, file: !2, line: 7, type: !6, flags: DIFlagPublic)
!46 = !DILocation(line: 7, scope: !43)
!47 = distinct !DISubprogram(name: "print_return", linkageName: "print_return", scope: null, file: !2, line: 8, type: !4, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !48)
!48 = !{!49}
!49 = !DILocalVariable(name: "0", arg: 1, scope: !47, file: !2, line: 8, type: !6, flags: DIFlagPublic)
!50 = !DILocation(line: 8, scope: !47)
!51 = distinct !DISubprogram(name: "main", linkageName: "add::add::main", scope: null, file: !2, line: 8, type: !24, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !26)
!52 = !DILocation(line: 8, scope: !51)
!53 = !DILocation(line: 9, scope: !51)
!54 = !DILocation(line: 10, scope: !51)
!55 = !DILocation(line: 11, scope: !51)
!56 = !DILocation(line: 12, scope: !51)
!57 = !DILocation(line: 13, scope: !51)
!58 = !DILocation(line: 14, scope: !51)
