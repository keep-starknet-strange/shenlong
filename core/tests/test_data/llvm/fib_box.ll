; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(ptr, ...)

define void @print_felt(i253 %0) !dbg !3 {
entry:
  %rounded_size_val = zext i253 %0 to i256
  %shifted = lshr i256 %rounded_size_val, 224
  %print_value_trunc = trunc i256 %shifted to i32
  %format = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format, align 1
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc)
  %shifted1 = lshr i256 %rounded_size_val, 192
  %print_value_trunc2 = trunc i256 %shifted1 to i32
  %format3 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format3, align 1
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2)
  %shifted5 = lshr i256 %rounded_size_val, 160
  %print_value_trunc6 = trunc i256 %shifted5 to i32
  %format7 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format7, align 1
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6)
  %shifted9 = lshr i256 %rounded_size_val, 128
  %print_value_trunc10 = trunc i256 %shifted9 to i32
  %format11 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format11, align 1
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10)
  %shifted13 = lshr i256 %rounded_size_val, 96
  %print_value_trunc14 = trunc i256 %shifted13 to i32
  %format15 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format15, align 1
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14)
  %shifted17 = lshr i256 %rounded_size_val, 64
  %print_value_trunc18 = trunc i256 %shifted17 to i32
  %format19 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format19, align 1
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18)
  %shifted21 = lshr i256 %rounded_size_val, 32
  %print_value_trunc22 = trunc i256 %shifted21 to i32
  %format23 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format23, align 1
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22)
  %shifted25 = lshr i256 %rounded_size_val, 0
  %print_value_trunc26 = trunc i256 %shifted25 to i32
  %format27 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format27, align 1
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26)
  %format29 = alloca [2 x i8], align 1
  store [2 x i8] c"\0A\00", ptr %format29, align 1
  %chars_printed30 = call i32 (ptr, ...) @printf(ptr %format29)
  ret void
}

define void @print_double_felt(i512 %0) !dbg !9 {
entry:
  %shifted = lshr i512 %0, 480, !dbg !13
  %print_value_trunc = trunc i512 %shifted to i32, !dbg !13
  %format = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format, align 1, !dbg !13
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc), !dbg !13
  %shifted1 = lshr i512 %0, 448, !dbg !13
  %print_value_trunc2 = trunc i512 %shifted1 to i32, !dbg !13
  %format3 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format3, align 1, !dbg !13
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2), !dbg !13
  %shifted5 = lshr i512 %0, 416, !dbg !13
  %print_value_trunc6 = trunc i512 %shifted5 to i32, !dbg !13
  %format7 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format7, align 1, !dbg !13
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6), !dbg !13
  %shifted9 = lshr i512 %0, 384, !dbg !13
  %print_value_trunc10 = trunc i512 %shifted9 to i32, !dbg !13
  %format11 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format11, align 1, !dbg !13
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10), !dbg !13
  %shifted13 = lshr i512 %0, 352, !dbg !13
  %print_value_trunc14 = trunc i512 %shifted13 to i32, !dbg !13
  %format15 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format15, align 1, !dbg !13
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14), !dbg !13
  %shifted17 = lshr i512 %0, 320, !dbg !13
  %print_value_trunc18 = trunc i512 %shifted17 to i32, !dbg !13
  %format19 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format19, align 1, !dbg !13
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18), !dbg !13
  %shifted21 = lshr i512 %0, 288, !dbg !13
  %print_value_trunc22 = trunc i512 %shifted21 to i32, !dbg !13
  %format23 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format23, align 1, !dbg !13
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22), !dbg !13
  %shifted25 = lshr i512 %0, 256, !dbg !13
  %print_value_trunc26 = trunc i512 %shifted25 to i32, !dbg !13
  %format27 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format27, align 1, !dbg !13
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26), !dbg !13
  %shifted29 = lshr i512 %0, 224, !dbg !13
  %print_value_trunc30 = trunc i512 %shifted29 to i32, !dbg !13
  %format31 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format31, align 1, !dbg !13
  %chars_printed32 = call i32 (ptr, ...) @printf(ptr %format31, i32 %print_value_trunc30), !dbg !13
  %shifted33 = lshr i512 %0, 192, !dbg !13
  %print_value_trunc34 = trunc i512 %shifted33 to i32, !dbg !13
  %format35 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format35, align 1, !dbg !13
  %chars_printed36 = call i32 (ptr, ...) @printf(ptr %format35, i32 %print_value_trunc34), !dbg !13
  %shifted37 = lshr i512 %0, 160, !dbg !13
  %print_value_trunc38 = trunc i512 %shifted37 to i32, !dbg !13
  %format39 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format39, align 1, !dbg !13
  %chars_printed40 = call i32 (ptr, ...) @printf(ptr %format39, i32 %print_value_trunc38), !dbg !13
  %shifted41 = lshr i512 %0, 128, !dbg !13
  %print_value_trunc42 = trunc i512 %shifted41 to i32, !dbg !13
  %format43 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format43, align 1, !dbg !13
  %chars_printed44 = call i32 (ptr, ...) @printf(ptr %format43, i32 %print_value_trunc42), !dbg !13
  %shifted45 = lshr i512 %0, 96, !dbg !13
  %print_value_trunc46 = trunc i512 %shifted45 to i32, !dbg !13
  %format47 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format47, align 1, !dbg !13
  %chars_printed48 = call i32 (ptr, ...) @printf(ptr %format47, i32 %print_value_trunc46), !dbg !13
  %shifted49 = lshr i512 %0, 64, !dbg !13
  %print_value_trunc50 = trunc i512 %shifted49 to i32, !dbg !13
  %format51 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format51, align 1, !dbg !13
  %chars_printed52 = call i32 (ptr, ...) @printf(ptr %format51, i32 %print_value_trunc50), !dbg !13
  %shifted53 = lshr i512 %0, 32, !dbg !13
  %print_value_trunc54 = trunc i512 %shifted53 to i32, !dbg !13
  %format55 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format55, align 1, !dbg !13
  %chars_printed56 = call i32 (ptr, ...) @printf(ptr %format55, i32 %print_value_trunc54), !dbg !13
  %shifted57 = lshr i512 %0, 0, !dbg !13
  %print_value_trunc58 = trunc i512 %shifted57 to i32, !dbg !13
  %format59 = alloca [5 x i8], align 1, !dbg !13
  store [5 x i8] c"%08X\00", ptr %format59, align 1, !dbg !13
  %chars_printed60 = call i32 (ptr, ...) @printf(ptr %format59, i32 %print_value_trunc58), !dbg !13
  %format61 = alloca [2 x i8], align 1, !dbg !13
  store [2 x i8] c"\0A\00", ptr %format61, align 1, !dbg !13
  %chars_printed62 = call i32 (ptr, ...) @printf(ptr %format61), !dbg !13
  ret void, !dbg !13
}

define i253 @modulo(i512 %0) !dbg !14 {
entry:
  %modulus = srem i512 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481, !dbg !17
  %res = trunc i512 %modulus to i253, !dbg !17
  ret i253 %res, !dbg !17
}

define i253 @"unbox<felt>"(i253 %0) {
entry:
  ret i253 %0, !dbg !17
}

define i253 @"store_temp<felt>"(i253 %0) {
entry:
  ret i253 %0, !dbg !17
}

define { i253, i253 } @"dup<felt>"(i253 %0) {
entry:
  %res_ptr = alloca { i253, i253 }, align 8, !dbg !17
  %tuple_ptr = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0, !dbg !17
  store i253 %0, ptr %tuple_ptr, align 4, !dbg !17
  %tuple_ptr_2 = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1, !dbg !17
  store i253 %0, ptr %tuple_ptr_2, align 4, !dbg !17
  %res = load { i253, i253 }, ptr %res_ptr, align 4, !dbg !17
  ret { i253, i253 } %res, !dbg !17
}

define i253 @"store_temp<Box<felt>>"(i253 %0) {
entry:
  ret i253 %0, !dbg !17
}

define { i253, i253 } @"dup<Box<felt>>"(i253 %0) {
entry:
  %res_ptr = alloca { i253, i253 }, align 8, !dbg !17
  %tuple_ptr = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0, !dbg !17
  store i253 %0, ptr %tuple_ptr, align 4, !dbg !17
  %tuple_ptr_2 = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1, !dbg !17
  store i253 %0, ptr %tuple_ptr_2, align 4, !dbg !17
  %res = load { i253, i253 }, ptr %res_ptr, align 4, !dbg !17
  ret { i253, i253 } %res, !dbg !17
}

define i253 @felt_add(i253 %0, i253 %1) !dbg !18 {
entry:
  %extended_a = sext i253 %0 to i512, !dbg !21
  %extended_b = sext i253 %1 to i512, !dbg !21
  %res = add i512 %extended_a, %extended_b, !dbg !21
  %res1 = call i253 @modulo(i512 %res), !dbg !21
  ret i253 %res1, !dbg !21
}

define i253 @"into_box<felt>"(i253 %0) {
entry:
  ret i253 %0, !dbg !21
}

define i253 @"felt_const<1>"() !dbg !22 {
entry:
  ret i253 1, !dbg !25
}

define i253 @felt_sub(i253 %0, i253 %1) !dbg !26 {
entry:
  %res = sub i253 %0, %1, !dbg !27
  %arg = sext i253 %res to i512, !dbg !27
  %res1 = call i253 @modulo(i512 %arg), !dbg !27
  ret i253 %res1, !dbg !27
}

define i253 @"rename<Box<felt>>"(i253 %0) {
entry:
  ret i253 %0, !dbg !27
}

define { i253 } @"fib_box::fib_box::fib"(i253 %0, i253 %1, i253 %2) {
entry:
  %3 = call i253 @"unbox<felt>"(i253 %2), !dbg !27
  %4 = call i253 @"store_temp<felt>"(i253 %3), !dbg !27
  %5 = call { i253, i253 } @"dup<felt>"(i253 %4), !dbg !27
  %res_ptr = alloca { i253, i253 }, align 8, !dbg !27
  store { i253, i253 } %5, ptr %res_ptr, align 4, !dbg !27
  %"3_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0, !dbg !27
  %"3" = load i253, ptr %"3_ptr", align 4, !dbg !27
  %"5_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1, !dbg !27
  %"5" = load i253, ptr %"5_ptr", align 4, !dbg !27
  %check = icmp eq i253 %"5", 0, !dbg !27
  br i1 %check, label %then, label %else, !dbg !27

then:                                             ; preds = %entry
  %6 = call i253 @"store_temp<Box<felt>>"(i253 %0), !dbg !27
  br label %dest, !dbg !27

else:                                             ; preds = %entry
  br label %dest1, !dbg !27

dest:                                             ; preds = %then
  %7 = call i253 @"rename<Box<felt>>"(i253 %6), !dbg !27
  %ret_struct_ptr = alloca { i253 }, align 8, !dbg !27
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0, !dbg !27
  store i253 %7, ptr %field_0_ptr, align 4, !dbg !27
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4, !dbg !27
  ret { i253 } %return_struct_value, !dbg !27

dest1:                                            ; preds = %else
  %8 = call i253 @"unbox<felt>"(i253 %0), !dbg !27
  %9 = call { i253, i253 } @"dup<Box<felt>>"(i253 %1), !dbg !27
  %res_ptr2 = alloca { i253, i253 }, align 8, !dbg !27
  store { i253, i253 } %9, ptr %res_ptr2, align 4, !dbg !27
  %"1_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr2, i32 0, i32 0, !dbg !27
  %"1" = load i253, ptr %"1_ptr", align 4, !dbg !27
  %"9_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr2, i32 0, i32 1, !dbg !27
  %"9" = load i253, ptr %"9_ptr", align 4, !dbg !27
  %10 = call i253 @"unbox<felt>"(i253 %"9"), !dbg !27
  %11 = call i253 @"store_temp<felt>"(i253 %8), !dbg !27
  %12 = call i253 @"store_temp<felt>"(i253 %10), !dbg !27
  %13 = call i253 @felt_add(i253 %11, i253 %12), !dbg !27
  %14 = call i253 @"store_temp<felt>"(i253 %13), !dbg !27
  %15 = call i253 @"into_box<felt>"(i253 %14), !dbg !27
  %16 = call i253 @"felt_const<1>"(), !dbg !27
  %17 = call i253 @felt_sub(i253 %"3", i253 %16), !dbg !27
  %18 = call i253 @"store_temp<felt>"(i253 %17), !dbg !27
  %19 = call i253 @"into_box<felt>"(i253 %18), !dbg !27
  %20 = call i253 @"store_temp<Box<felt>>"(i253 %"1"), !dbg !27
  %21 = call i253 @"store_temp<Box<felt>>"(i253 %15), !dbg !27
  %22 = call i253 @"store_temp<Box<felt>>"(i253 %19), !dbg !27
  %23 = call { i253 } @"fib_box::fib_box::fib"(i253 %20, i253 %21, i253 %22), !dbg !27
  %res_ptr3 = alloca { i253 }, align 8, !dbg !27
  store { i253 } %23, ptr %res_ptr3, align 4, !dbg !27
  %"15_ptr" = getelementptr inbounds { i253 }, ptr %res_ptr3, i32 0, i32 0, !dbg !27
  %"15" = load i253, ptr %"15_ptr", align 4, !dbg !27
  %24 = call i253 @"rename<Box<felt>>"(i253 %"15"), !dbg !27
  br label %dest4, !dbg !27

dest4:                                            ; preds = %dest1
  %25 = call i253 @"rename<Box<felt>>"(i253 %24), !dbg !27
  %ret_struct_ptr5 = alloca { i253 }, align 8, !dbg !27
  %field_0_ptr6 = getelementptr inbounds { i253 }, ptr %ret_struct_ptr5, i32 0, i32 0, !dbg !27
  store i253 %25, ptr %field_0_ptr6, align 4, !dbg !27
  %return_struct_value7 = load { i253 }, ptr %ret_struct_ptr5, align 4, !dbg !27
  ret { i253 } %return_struct_value7, !dbg !27
}

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 2, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus, file: !2, producer: "shenlong", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "sierra/fib_box.sierra", directory: "sierra")
!3 = distinct !DISubprogram(name: "print_felt", linkageName: "print_felt", scope: null, file: !2, line: 4, type: !4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{null, !6}
!6 = !DIBasicType(name: "felt", size: 253, flags: DIFlagPublic)
!7 = !{}
!8 = !DILocation(line: 4, scope: !3)
!9 = distinct !DISubprogram(name: "print_double_felt", linkageName: "print_double_felt", scope: null, file: !2, line: 4, type: !10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!10 = !DISubroutineType(flags: DIFlagPublic, types: !11)
!11 = !{null, !12}
!12 = !DIBasicType(name: "double_felt", size: 512, flags: DIFlagPublic)
!13 = !DILocation(line: 4, scope: !9)
!14 = distinct !DISubprogram(name: "modulo", linkageName: "modulo", scope: null, file: !2, line: 4, type: !15, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!15 = !DISubroutineType(flags: DIFlagPublic, types: !16)
!16 = !{!6, !12}
!17 = !DILocation(line: 4, scope: !14)
!18 = distinct !DISubprogram(name: "felt_add", linkageName: "felt_add", scope: null, file: !2, line: 17, type: !19, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!19 = !DISubroutineType(flags: DIFlagPublic, types: !20)
!20 = !{!6, !6, !6}
!21 = !DILocation(line: 17, scope: !1)
!22 = distinct !DISubprogram(name: "felt_const<1>", linkageName: "felt_const<1>", scope: null, file: !2, line: 19, type: !23, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!23 = !DISubroutineType(flags: DIFlagPublic, types: !24)
!24 = !{!6}
!25 = !DILocation(line: 19, scope: !22)
!26 = distinct !DISubprogram(name: "felt_sub", linkageName: "felt_sub", scope: null, file: !2, line: 20, type: !19, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!27 = !DILocation(line: 20, scope: !26)
