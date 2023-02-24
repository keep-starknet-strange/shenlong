; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

define i252 @modulo(i503 %0) {
entry:
  %modulus = srem i503 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %res = trunc i503 %modulus to i252
  ret i252 %res
}

declare i32 @printf(ptr, ...)

define void @print_felt(i252 %0) {
entry:
  %rounded_size_val = sext i252 %0 to i256
  %shifted = lshr i256 %rounded_size_val, 224
  %print_value_trunc = trunc i256 %shifted to i32
  %format = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format, align 1
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc)
  %shifted1 = lshr i256 %rounded_size_val, 192
  %print_value_trunc2 = trunc i256 %shifted1 to i32
  %format3 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format3, align 1
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2)
  %shifted5 = lshr i256 %rounded_size_val, 160
  %print_value_trunc6 = trunc i256 %shifted5 to i32
  %format7 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format7, align 1
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6)
  %shifted9 = lshr i256 %rounded_size_val, 128
  %print_value_trunc10 = trunc i256 %shifted9 to i32
  %format11 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format11, align 1
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10)
  %shifted13 = lshr i256 %rounded_size_val, 96
  %print_value_trunc14 = trunc i256 %shifted13 to i32
  %format15 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format15, align 1
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14)
  %shifted17 = lshr i256 %rounded_size_val, 64
  %print_value_trunc18 = trunc i256 %shifted17 to i32
  %format19 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format19, align 1
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18)
  %shifted21 = lshr i256 %rounded_size_val, 32
  %print_value_trunc22 = trunc i256 %shifted21 to i32
  %format23 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format23, align 1
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22)
  %shifted25 = lshr i256 %rounded_size_val, 0
  %print_value_trunc26 = trunc i256 %shifted25 to i32
  %format27 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format27, align 1
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26)
  %format29 = alloca [1 x i8], align 1
  store [2 x i8] c"\0A\00", ptr %format29, align 1
  %chars_printed30 = call i32 (ptr, ...) @printf(ptr %format29)
  ret void
}

define void @print_double_felt(i503 %0) {
entry:
  %rounded_size_val = sext i503 %0 to i512
  %shifted = lshr i512 %rounded_size_val, 480
  %print_value_trunc = trunc i512 %shifted to i32
  %format = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format, align 1
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc)
  %shifted1 = lshr i512 %rounded_size_val, 448
  %print_value_trunc2 = trunc i512 %shifted1 to i32
  %format3 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format3, align 1
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2)
  %shifted5 = lshr i512 %rounded_size_val, 416
  %print_value_trunc6 = trunc i512 %shifted5 to i32
  %format7 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format7, align 1
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6)
  %shifted9 = lshr i512 %rounded_size_val, 384
  %print_value_trunc10 = trunc i512 %shifted9 to i32
  %format11 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format11, align 1
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10)
  %shifted13 = lshr i512 %rounded_size_val, 352
  %print_value_trunc14 = trunc i512 %shifted13 to i32
  %format15 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format15, align 1
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14)
  %shifted17 = lshr i512 %rounded_size_val, 320
  %print_value_trunc18 = trunc i512 %shifted17 to i32
  %format19 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format19, align 1
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18)
  %shifted21 = lshr i512 %rounded_size_val, 288
  %print_value_trunc22 = trunc i512 %shifted21 to i32
  %format23 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format23, align 1
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22)
  %shifted25 = lshr i512 %rounded_size_val, 256
  %print_value_trunc26 = trunc i512 %shifted25 to i32
  %format27 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format27, align 1
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26)
  %shifted29 = lshr i512 %rounded_size_val, 224
  %print_value_trunc30 = trunc i512 %shifted29 to i32
  %format31 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format31, align 1
  %chars_printed32 = call i32 (ptr, ...) @printf(ptr %format31, i32 %print_value_trunc30)
  %shifted33 = lshr i512 %rounded_size_val, 192
  %print_value_trunc34 = trunc i512 %shifted33 to i32
  %format35 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format35, align 1
  %chars_printed36 = call i32 (ptr, ...) @printf(ptr %format35, i32 %print_value_trunc34)
  %shifted37 = lshr i512 %rounded_size_val, 160
  %print_value_trunc38 = trunc i512 %shifted37 to i32
  %format39 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format39, align 1
  %chars_printed40 = call i32 (ptr, ...) @printf(ptr %format39, i32 %print_value_trunc38)
  %shifted41 = lshr i512 %rounded_size_val, 128
  %print_value_trunc42 = trunc i512 %shifted41 to i32
  %format43 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format43, align 1
  %chars_printed44 = call i32 (ptr, ...) @printf(ptr %format43, i32 %print_value_trunc42)
  %shifted45 = lshr i512 %rounded_size_val, 96
  %print_value_trunc46 = trunc i512 %shifted45 to i32
  %format47 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format47, align 1
  %chars_printed48 = call i32 (ptr, ...) @printf(ptr %format47, i32 %print_value_trunc46)
  %shifted49 = lshr i512 %rounded_size_val, 64
  %print_value_trunc50 = trunc i512 %shifted49 to i32
  %format51 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format51, align 1
  %chars_printed52 = call i32 (ptr, ...) @printf(ptr %format51, i32 %print_value_trunc50)
  %shifted53 = lshr i512 %rounded_size_val, 32
  %print_value_trunc54 = trunc i512 %shifted53 to i32
  %format55 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format55, align 1
  %chars_printed56 = call i32 (ptr, ...) @printf(ptr %format55, i32 %print_value_trunc54)
  %shifted57 = lshr i512 %rounded_size_val, 0
  %print_value_trunc58 = trunc i512 %shifted57 to i32
  %format59 = alloca [4 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format59, align 1
  %chars_printed60 = call i32 (ptr, ...) @printf(ptr %format59, i32 %print_value_trunc58)
  %format61 = alloca [1 x i8], align 1
  store [2 x i8] c"\0A\00", ptr %format61, align 1
  %chars_printed62 = call i32 (ptr, ...) @printf(ptr %format61)
  ret void
}

define i252 @"unbox<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define i252 @"store_temp<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define { i252, i252 } @"dup<felt>"(i252 %0) {
entry:
  %res_ptr = alloca { i252, i252 }, align 8
  %tuple_ptr = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 0
  store i252 %0, ptr %tuple_ptr, align 4
  %tuple_ptr_2 = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 1
  store i252 %0, ptr %tuple_ptr_2, align 4
  %res = load { i252, i252 }, ptr %res_ptr, align 4
  ret { i252, i252 } %res
}

define i252 @"store_temp<Box<felt>>"(i252 %0) {
entry:
  ret i252 %0
}

define { i252, i252 } @"dup<Box<felt>>"(i252 %0) {
entry:
  %res_ptr = alloca { i252, i252 }, align 8
  %tuple_ptr = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 0
  store i252 %0, ptr %tuple_ptr, align 4
  %tuple_ptr_2 = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 1
  store i252 %0, ptr %tuple_ptr_2, align 4
  %res = load { i252, i252 }, ptr %res_ptr, align 4
  ret { i252, i252 } %res
}

define i252 @felt_add(i252 %0, i252 %1) {
entry:
  %extended_a = sext i252 %0 to i253
  %extended_b = sext i252 %1 to i253
  %res = add i253 %extended_a, %extended_b
  %arg = sext i253 %res to i503
  %res1 = call i252 @modulo(i503 %arg)
  ret i252 %res1
}

define i252 @"into_box<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define i252 @"felt_const<1>"() {
entry:
  ret i252 1
}

define i252 @felt_sub(i252 %0, i252 %1) {
entry:
  %res = sub i252 %0, %1
  %arg = sext i252 %res to i503
  %res1 = call i252 @modulo(i503 %arg)
  ret i252 %res1
}

define i252 @"rename<Box<felt>>"(i252 %0) {
entry:
  ret i252 %0
}

define { i252 } @"fib_box::fib_box::fib"(i252 %0, i252 %1, i252 %2) {
entry:
  %3 = call i252 @"unbox<felt>"(i252 %2)
  %4 = call i252 @"store_temp<felt>"(i252 %3)
  %5 = call { i252, i252 } @"dup<felt>"(i252 %4)
  %res_ptr = alloca { i252, i252 }, align 8
  store { i252, i252 } %5, ptr %res_ptr, align 4
  %"3_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 0
  %"3" = load i252, ptr %"3_ptr", align 4
  %"5_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 1
  %"5" = load i252, ptr %"5_ptr", align 4
  %check = icmp eq i252 %"5", 0
  br i1 %check, label %then, label %else

then:                                             ; preds = %entry
  %6 = call i252 @"store_temp<Box<felt>>"(i252 %0)
  br label %dest

else:                                             ; preds = %entry
  br label %dest1

dest:                                             ; preds = %then
  %7 = call i252 @"rename<Box<felt>>"(i252 %6)
  %ret_struct_ptr = alloca { i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i252 %7, ptr %field_0_ptr, align 4
  %return_struct_value = load { i252 }, ptr %ret_struct_ptr, align 4
  ret { i252 } %return_struct_value

dest1:                                            ; preds = %else
  %8 = call i252 @"unbox<felt>"(i252 %0)
  %9 = call { i252, i252 } @"dup<Box<felt>>"(i252 %1)
  %res_ptr2 = alloca { i252, i252 }, align 8
  store { i252, i252 } %9, ptr %res_ptr2, align 4
  %"1_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr2, i32 0, i32 0
  %"1" = load i252, ptr %"1_ptr", align 4
  %"9_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr2, i32 0, i32 1
  %"9" = load i252, ptr %"9_ptr", align 4
  %10 = call i252 @"unbox<felt>"(i252 %"9")
  %11 = call i252 @"store_temp<felt>"(i252 %8)
  %12 = call i252 @"store_temp<felt>"(i252 %10)
  %13 = call i252 @felt_add(i252 %11, i252 %12)
  %14 = call i252 @"store_temp<felt>"(i252 %13)
  %15 = call i252 @"into_box<felt>"(i252 %14)
  %16 = call i252 @"felt_const<1>"()
  %17 = call i252 @felt_sub(i252 %"3", i252 %16)
  %18 = call i252 @"store_temp<felt>"(i252 %17)
  %19 = call i252 @"into_box<felt>"(i252 %18)
  %20 = call i252 @"store_temp<Box<felt>>"(i252 %"1")
  %21 = call i252 @"store_temp<Box<felt>>"(i252 %15)
  %22 = call i252 @"store_temp<Box<felt>>"(i252 %19)
  %23 = call { i252 } @"fib_box::fib_box::fib"(i252 %20, i252 %21, i252 %22)
  %res_ptr3 = alloca { i252 }, align 8
  store { i252 } %23, ptr %res_ptr3, align 4
  %"15_ptr" = getelementptr inbounds { i252 }, ptr %res_ptr3, i32 0, i32 0
  %"15" = load i252, ptr %"15_ptr", align 4
  %24 = call i252 @"rename<Box<felt>>"(i252 %"15")
  br label %dest4

dest4:                                            ; preds = %dest1
  %25 = call i252 @"rename<Box<felt>>"(i252 %24)
  %ret_struct_ptr5 = alloca { i252 }, align 8
  %field_0_ptr6 = getelementptr inbounds { i252 }, ptr %ret_struct_ptr5, i32 0, i32 0
  store i252 %25, ptr %field_0_ptr6, align 4
  %return_struct_value7 = load { i252 }, ptr %ret_struct_ptr5, align 4
  ret { i252 } %return_struct_value7
}
