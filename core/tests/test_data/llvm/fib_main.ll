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

define { i253, i253 } @"dup<felt>"(i253 %0) !dbg !18 {
entry:
  %res_ptr = alloca { i253, i253 }, align 8, !dbg !23
  %tuple_ptr = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0, !dbg !23
  store i253 %0, ptr %tuple_ptr, align 4, !dbg !23
  %tuple_ptr_2 = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1, !dbg !23
  store i253 %0, ptr %tuple_ptr_2, align 4, !dbg !23
  %res = load { i253, i253 }, ptr %res_ptr, align 4, !dbg !23
  ret { i253, i253 } %res, !dbg !23
}

define i253 @"store_temp<felt>"(i253 %0) !dbg !24 {
entry:
  ret i253 %0, !dbg !27
}

define i253 @felt_add(i253 %0, i253 %1) !dbg !28 {
entry:
  %extended_a = sext i253 %0 to i512, !dbg !31
  %extended_b = sext i253 %1 to i512, !dbg !31
  %res = add i512 %extended_a, %extended_b, !dbg !31
  %res1 = call i253 @modulo(i512 %res), !dbg !31
  ret i253 %res1, !dbg !31
}

define i253 @"felt_const<1>"() !dbg !32 {
entry:
  ret i253 1, !dbg !34
}

define i253 @felt_sub(i253 %0, i253 %1) !dbg !35 {
entry:
  %res = sub i253 %0, %1, !dbg !36
  %arg = sext i253 %res to i512, !dbg !36
  %res1 = call i253 @modulo(i512 %arg), !dbg !36
  ret i253 %res1, !dbg !36
}

define i253 @"rename<felt>"(i253 %0) !dbg !37 {
entry:
  ret i253 %0, !dbg !38
}

define i253 @"felt_const<0>"() !dbg !39 {
entry:
  ret i253 0, !dbg !40
}

define i253 @"felt_const<30>"() !dbg !41 {
entry:
  ret i253 30, !dbg !42
}

define i253 @"fib_caller::fib_caller::fib"(i253 %0, i253 %1, i253 %2) !dbg !43 {
entry:
  %3 = call { i253, i253 } @"dup<felt>"(i253 %2), !dbg !46
  %res_ptr = alloca { i253, i253 }, align 8, !dbg !46
  store { i253, i253 } %3, ptr %res_ptr, align 4, !dbg !46
  %"2_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0, !dbg !46
  %"2" = load i253, ptr %"2_ptr", align 4, !dbg !46
  %"4_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1, !dbg !46
  %"4" = load i253, ptr %"4_ptr", align 4, !dbg !46
  %check = icmp eq i253 %"4", 0, !dbg !46
  br i1 %check, label %then, label %else, !dbg !46

then:                                             ; preds = %entry
  %4 = call i253 @"store_temp<felt>"(i253 %0), !dbg !46
  br label %dest, !dbg !46

else:                                             ; preds = %entry
  br label %dest1, !dbg !46

dest:                                             ; preds = %then
  %5 = call i253 @"rename<felt>"(i253 %4), !dbg !46
  %ret_struct_ptr = alloca { i253 }, align 8, !dbg !46
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0, !dbg !46
  store i253 %5, ptr %field_0_ptr, align 4, !dbg !46
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4, !dbg !46
  ret { i253 } %return_struct_value, !dbg !46

dest1:                                            ; preds = %else
  %6 = call { i253, i253 } @"dup<felt>"(i253 %1), !dbg !46
  %res_ptr2 = alloca { i253, i253 }, align 8, !dbg !46
  store { i253, i253 } %6, ptr %res_ptr2, align 4, !dbg !46
  %"1_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr2, i32 0, i32 0, !dbg !46
  %"1" = load i253, ptr %"1_ptr", align 4, !dbg !46
  %"7_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr2, i32 0, i32 1, !dbg !46
  %"7" = load i253, ptr %"7_ptr", align 4, !dbg !46
  %7 = call i253 @felt_add(i253 %0, i253 %"7"), !dbg !46
  %8 = call i253 @"felt_const<1>"(), !dbg !46
  %9 = call i253 @felt_sub(i253 %"2", i253 %8), !dbg !46
  %10 = call i253 @"store_temp<felt>"(i253 %"1"), !dbg !46
  %11 = call i253 @"store_temp<felt>"(i253 %7), !dbg !46
  %12 = call i253 @"store_temp<felt>"(i253 %9), !dbg !46
  %13 = call i253 @"fib_caller::fib_caller::fib"(i253 %10, i253 %11, i253 %12), !dbg !46
  %14 = call i253 @"rename<felt>"(i253 %13), !dbg !46
  br label %dest3, !dbg !46

dest3:                                            ; preds = %dest1
  %15 = call i253 @"rename<felt>"(i253 %14), !dbg !46
  %ret_struct_ptr4 = alloca { i253 }, align 8, !dbg !46
  %field_0_ptr5 = getelementptr inbounds { i253 }, ptr %ret_struct_ptr4, i32 0, i32 0, !dbg !46
  store i253 %15, ptr %field_0_ptr5, align 4, !dbg !46
  %return_struct_value6 = load { i253 }, ptr %ret_struct_ptr4, align 4, !dbg !46
  ret { i253 } %return_struct_value6, !dbg !46
}

define void @print_return(i253 %0) !dbg !47 {
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

define i32 @main(i253 %0) !dbg !49 {
entry:
  %1 = call i253 @"felt_const<0>"(), !dbg !50
  %2 = call i253 @"felt_const<1>"(), !dbg !50
  %3 = call i253 @"felt_const<30>"(), !dbg !50
  %4 = call i253 @"store_temp<felt>"(i253 %1), !dbg !50
  %5 = call i253 @"store_temp<felt>"(i253 %2), !dbg !50
  %6 = call i253 @"store_temp<felt>"(i253 %3), !dbg !50
  %7 = call i253 @"fib_caller::fib_caller::fib"(i253 %4, i253 %5, i253 %6), !dbg !50
  %8 = call i253 @"rename<felt>"(i253 %7), !dbg !50
  %ret_struct_ptr = alloca { i253 }, align 8, !dbg !50
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0, !dbg !50
  store i253 %8, ptr %field_0_ptr, align 4, !dbg !50
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4, !dbg !50
  %return_value_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0, !dbg !50
  %return_value = load i253, ptr %return_value_ptr, align 4, !dbg !50
  %format = alloca [15 x i8], align 1, !dbg !50
  store [15 x i8] c"Return value: \00", ptr %format, align 1, !dbg !50
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format), !dbg !50
  call void @print_return(i253 %return_value), !dbg !50
  ret i32 0, !dbg !50
}

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 2, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus, file: !2, producer: "shenlong", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "sierra/fib_main.sierra", directory: "sierra")
!3 = distinct !DISubprogram(name: "print_felt", linkageName: "print_felt", scope: null, file: !2, line: 3, type: !4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{null, !6}
!6 = !DIBasicType(name: "felt", size: 253, flags: DIFlagPublic)
!7 = !{}
!8 = !DILocation(line: 3, scope: !3)
!9 = distinct !DISubprogram(name: "print_double_felt", linkageName: "print_double_felt", scope: null, file: !2, line: 3, type: !10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!10 = !DISubroutineType(flags: DIFlagPublic, types: !11)
!11 = !{null, !12}
!12 = !DIBasicType(name: "double_felt", size: 512, flags: DIFlagPublic)
!13 = !DILocation(line: 3, scope: !9)
!14 = distinct !DISubprogram(name: "modulo", linkageName: "modulo", scope: null, file: !2, line: 3, type: !15, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!15 = !DISubroutineType(flags: DIFlagPublic, types: !16)
!16 = !{!6, !12}
!17 = !DILocation(line: 3, scope: !14)
!18 = distinct !DISubprogram(name: "dup<felt>", linkageName: "dup<felt>", scope: null, file: !2, line: 5, type: !19, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!19 = !DISubroutineType(flags: DIFlagPublic, types: !20)
!20 = !{!21, !6}
!21 = !DICompositeType(tag: DW_TAG_structure_type, name: "return_type_dup<felt>", file: !2, line: 5, size: 253, align: 64, flags: DIFlagPublic, elements: !22, identifier: "return_type_dup<felt>")
!22 = !{!6}
!23 = !DILocation(line: 5, scope: !18)
!24 = distinct !DISubprogram(name: "store_temp<felt>", linkageName: "store_temp<felt>", scope: null, file: !2, line: 9, type: !25, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!25 = !DISubroutineType(flags: DIFlagPublic, types: !26)
!26 = !{!6, !6}
!27 = !DILocation(line: 9, scope: !24)
!28 = distinct !DISubprogram(name: "felt_add", linkageName: "felt_add", scope: null, file: !2, line: 12, type: !29, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!29 = !DISubroutineType(flags: DIFlagPublic, types: !30)
!30 = !{!6, !6, !6}
!31 = !DILocation(line: 12, scope: !28)
!32 = distinct !DISubprogram(name: "felt_const<1>", linkageName: "felt_const<1>", scope: null, file: !2, line: 13, type: !33, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!33 = !DISubroutineType(flags: DIFlagPublic, types: !22)
!34 = !DILocation(line: 13, scope: !32)
!35 = distinct !DISubprogram(name: "felt_sub", linkageName: "felt_sub", scope: null, file: !2, line: 14, type: !29, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!36 = !DILocation(line: 14, scope: !35)
!37 = distinct !DISubprogram(name: "rename<felt>", linkageName: "rename<felt>", scope: null, file: !2, line: 16, type: !25, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!38 = !DILocation(line: 16, scope: !37)
!39 = distinct !DISubprogram(name: "felt_const<0>", linkageName: "felt_const<0>", scope: null, file: !2, line: 17, type: !33, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!40 = !DILocation(line: 17, scope: !39)
!41 = distinct !DISubprogram(name: "felt_const<30>", linkageName: "felt_const<30>", scope: null, file: !2, line: 18, type: !33, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!42 = !DILocation(line: 18, scope: !41)
!43 = distinct !DISubprogram(name: "fib_caller::fib_caller::fib", linkageName: "fib_caller::fib_caller::fib", scope: null, file: !2, line: 20, type: !44, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!44 = !DISubroutineType(flags: DIFlagPublic, types: !45)
!45 = !{!6, !6, !6, !6}
!46 = !DILocation(line: 20, scope: !43)
!47 = distinct !DISubprogram(name: "print_return", linkageName: "print_return", scope: null, file: !2, line: 21, type: !4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!48 = !DILocation(line: 21, scope: !47)
!49 = distinct !DISubprogram(name: "fib_caller::fib_caller::main", linkageName: "fib_caller::fib_caller::main", scope: null, file: !2, line: 21, type: !25, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, retainedNodes: !7)
!50 = !DILocation(line: 21, scope: !49)
