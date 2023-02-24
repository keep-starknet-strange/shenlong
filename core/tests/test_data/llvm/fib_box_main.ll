; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(ptr, ...)

define void @print_felt(i253 %0) {
entry:
  %rounded_size_val = sext i253 %0 to i256
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

define void @print_double_felt(i512 %0) {
entry:
  %shifted = lshr i512 %0, 480
  %print_value_trunc = trunc i512 %shifted to i32
  %format = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format, align 1
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format, i32 %print_value_trunc)
  %shifted1 = lshr i512 %0, 448
  %print_value_trunc2 = trunc i512 %shifted1 to i32
  %format3 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format3, align 1
  %chars_printed4 = call i32 (ptr, ...) @printf(ptr %format3, i32 %print_value_trunc2)
  %shifted5 = lshr i512 %0, 416
  %print_value_trunc6 = trunc i512 %shifted5 to i32
  %format7 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format7, align 1
  %chars_printed8 = call i32 (ptr, ...) @printf(ptr %format7, i32 %print_value_trunc6)
  %shifted9 = lshr i512 %0, 384
  %print_value_trunc10 = trunc i512 %shifted9 to i32
  %format11 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format11, align 1
  %chars_printed12 = call i32 (ptr, ...) @printf(ptr %format11, i32 %print_value_trunc10)
  %shifted13 = lshr i512 %0, 352
  %print_value_trunc14 = trunc i512 %shifted13 to i32
  %format15 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format15, align 1
  %chars_printed16 = call i32 (ptr, ...) @printf(ptr %format15, i32 %print_value_trunc14)
  %shifted17 = lshr i512 %0, 320
  %print_value_trunc18 = trunc i512 %shifted17 to i32
  %format19 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format19, align 1
  %chars_printed20 = call i32 (ptr, ...) @printf(ptr %format19, i32 %print_value_trunc18)
  %shifted21 = lshr i512 %0, 288
  %print_value_trunc22 = trunc i512 %shifted21 to i32
  %format23 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format23, align 1
  %chars_printed24 = call i32 (ptr, ...) @printf(ptr %format23, i32 %print_value_trunc22)
  %shifted25 = lshr i512 %0, 256
  %print_value_trunc26 = trunc i512 %shifted25 to i32
  %format27 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format27, align 1
  %chars_printed28 = call i32 (ptr, ...) @printf(ptr %format27, i32 %print_value_trunc26)
  %shifted29 = lshr i512 %0, 224
  %print_value_trunc30 = trunc i512 %shifted29 to i32
  %format31 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format31, align 1
  %chars_printed32 = call i32 (ptr, ...) @printf(ptr %format31, i32 %print_value_trunc30)
  %shifted33 = lshr i512 %0, 192
  %print_value_trunc34 = trunc i512 %shifted33 to i32
  %format35 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format35, align 1
  %chars_printed36 = call i32 (ptr, ...) @printf(ptr %format35, i32 %print_value_trunc34)
  %shifted37 = lshr i512 %0, 160
  %print_value_trunc38 = trunc i512 %shifted37 to i32
  %format39 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format39, align 1
  %chars_printed40 = call i32 (ptr, ...) @printf(ptr %format39, i32 %print_value_trunc38)
  %shifted41 = lshr i512 %0, 128
  %print_value_trunc42 = trunc i512 %shifted41 to i32
  %format43 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format43, align 1
  %chars_printed44 = call i32 (ptr, ...) @printf(ptr %format43, i32 %print_value_trunc42)
  %shifted45 = lshr i512 %0, 96
  %print_value_trunc46 = trunc i512 %shifted45 to i32
  %format47 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format47, align 1
  %chars_printed48 = call i32 (ptr, ...) @printf(ptr %format47, i32 %print_value_trunc46)
  %shifted49 = lshr i512 %0, 64
  %print_value_trunc50 = trunc i512 %shifted49 to i32
  %format51 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format51, align 1
  %chars_printed52 = call i32 (ptr, ...) @printf(ptr %format51, i32 %print_value_trunc50)
  %shifted53 = lshr i512 %0, 32
  %print_value_trunc54 = trunc i512 %shifted53 to i32
  %format55 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format55, align 1
  %chars_printed56 = call i32 (ptr, ...) @printf(ptr %format55, i32 %print_value_trunc54)
  %shifted57 = lshr i512 %0, 0
  %print_value_trunc58 = trunc i512 %shifted57 to i32
  %format59 = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %format59, align 1
  %chars_printed60 = call i32 (ptr, ...) @printf(ptr %format59, i32 %print_value_trunc58)
  %format61 = alloca [2 x i8], align 1
  store [2 x i8] c"\0A\00", ptr %format61, align 1
  %chars_printed62 = call i32 (ptr, ...) @printf(ptr %format61)
  ret void
}

define i253 @modulo(i512 %0) {
entry:
  %modulus = srem i512 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %res = trunc i512 %modulus to i253
  ret i253 %res
}

define i253 @"unbox<felt>"(i253 %0) {
entry:
  ret i253 %0
}

define i253 @"store_temp<felt>"(i253 %0) {
entry:
  ret i253 %0
}

define { i253, i253 } @"dup<felt>"(i253 %0) {
entry:
  %res_ptr = alloca { i253, i253 }, align 8
  %tuple_ptr = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0
  store i253 %0, ptr %tuple_ptr, align 4
  %tuple_ptr_2 = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1
  store i253 %0, ptr %tuple_ptr_2, align 4
  %res = load { i253, i253 }, ptr %res_ptr, align 4
  ret { i253, i253 } %res
}

define i253 @"store_temp<Box<felt>>"(i253 %0) {
entry:
  ret i253 %0
}

define { i253, i253 } @"dup<Box<felt>>"(i253 %0) {
entry:
  %res_ptr = alloca { i253, i253 }, align 8
  %tuple_ptr = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0
  store i253 %0, ptr %tuple_ptr, align 4
  %tuple_ptr_2 = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1
  store i253 %0, ptr %tuple_ptr_2, align 4
  %res = load { i253, i253 }, ptr %res_ptr, align 4
  ret { i253, i253 } %res
}

define i253 @felt_add(i253 %0, i253 %1) {
entry:
  %extended_a = sext i253 %0 to i512
  %extended_b = sext i253 %1 to i512
  %res = add i512 %extended_a, %extended_b
  %res1 = call i253 @modulo(i512 %res)
  ret i253 %res1
}

define i253 @"into_box<felt>"(i253 %0) {
entry:
  ret i253 %0
}

define i253 @"felt_const<1>"() {
entry:
  ret i253 1
}

define i253 @felt_sub(i253 %0, i253 %1) {
entry:
  %res = sub i253 %0, %1
  %arg = sext i253 %res to i512
  %res1 = call i253 @modulo(i512 %arg)
  ret i253 %res1
}

define i253 @"rename<Box<felt>>"(i253 %0) {
entry:
  ret i253 %0
}

define i253 @"felt_const<0>"() {
entry:
  ret i253 0
}

define i253 @"felt_const<30>"() {
entry:
  ret i253 30
}

define { i253 } @"fib_box::fib_box::fib"(i253 %0, i253 %1, i253 %2) {
entry:
  %3 = call i253 @"unbox<felt>"(i253 %2)
  %4 = call i253 @"store_temp<felt>"(i253 %3)
  %5 = call { i253, i253 } @"dup<felt>"(i253 %4)
  %res_ptr = alloca { i253, i253 }, align 8
  store { i253, i253 } %5, ptr %res_ptr, align 4
  %"3_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 0
  %"3" = load i253, ptr %"3_ptr", align 4
  %"5_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr, i32 0, i32 1
  %"5" = load i253, ptr %"5_ptr", align 4
  %check = icmp eq i253 %"5", 0
  br i1 %check, label %then, label %else

then:                                             ; preds = %entry
  %6 = call i253 @"store_temp<Box<felt>>"(i253 %0)
  br label %dest

else:                                             ; preds = %entry
  br label %dest1

dest:                                             ; preds = %then
  %7 = call i253 @"rename<Box<felt>>"(i253 %6)
  %ret_struct_ptr = alloca { i253 }, align 8
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i253 %7, ptr %field_0_ptr, align 4
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4
  ret { i253 } %return_struct_value

dest1:                                            ; preds = %else
  %8 = call i253 @"unbox<felt>"(i253 %0)
  %9 = call { i253, i253 } @"dup<Box<felt>>"(i253 %1)
  %res_ptr2 = alloca { i253, i253 }, align 8
  store { i253, i253 } %9, ptr %res_ptr2, align 4
  %"1_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr2, i32 0, i32 0
  %"1" = load i253, ptr %"1_ptr", align 4
  %"9_ptr" = getelementptr inbounds { i253, i253 }, ptr %res_ptr2, i32 0, i32 1
  %"9" = load i253, ptr %"9_ptr", align 4
  %10 = call i253 @"unbox<felt>"(i253 %"9")
  %11 = call i253 @"store_temp<felt>"(i253 %8)
  %12 = call i253 @"store_temp<felt>"(i253 %10)
  %13 = call i253 @felt_add(i253 %11, i253 %12)
  %14 = call i253 @"store_temp<felt>"(i253 %13)
  %15 = call i253 @"into_box<felt>"(i253 %14)
  %16 = call i253 @"felt_const<1>"()
  %17 = call i253 @felt_sub(i253 %"3", i253 %16)
  %18 = call i253 @"store_temp<felt>"(i253 %17)
  %19 = call i253 @"into_box<felt>"(i253 %18)
  %20 = call i253 @"store_temp<Box<felt>>"(i253 %"1")
  %21 = call i253 @"store_temp<Box<felt>>"(i253 %15)
  %22 = call i253 @"store_temp<Box<felt>>"(i253 %19)
  %23 = call { i253 } @"fib_box::fib_box::fib"(i253 %20, i253 %21, i253 %22)
  %res_ptr3 = alloca { i253 }, align 8
  store { i253 } %23, ptr %res_ptr3, align 4
  %"15_ptr" = getelementptr inbounds { i253 }, ptr %res_ptr3, i32 0, i32 0
  %"15" = load i253, ptr %"15_ptr", align 4
  %24 = call i253 @"rename<Box<felt>>"(i253 %"15")
  br label %dest4

dest4:                                            ; preds = %dest1
  %25 = call i253 @"rename<Box<felt>>"(i253 %24)
  %ret_struct_ptr5 = alloca { i253 }, align 8
  %field_0_ptr6 = getelementptr inbounds { i253 }, ptr %ret_struct_ptr5, i32 0, i32 0
  store i253 %25, ptr %field_0_ptr6, align 4
  %return_struct_value7 = load { i253 }, ptr %ret_struct_ptr5, align 4
  ret { i253 } %return_struct_value7
}

define void @print_return(i253 %0) {
entry:
  %rounded_size_val = sext i253 %0 to i256
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

define i32 @main() {
entry:
  %0 = call i253 @"felt_const<0>"()
  %1 = call i253 @"store_temp<felt>"(i253 %0)
  %2 = call i253 @"into_box<felt>"(i253 %1)
  %3 = call i253 @"felt_const<1>"()
  %4 = call i253 @"store_temp<felt>"(i253 %3)
  %5 = call i253 @"into_box<felt>"(i253 %4)
  %6 = call i253 @"felt_const<30>"()
  %7 = call i253 @"store_temp<felt>"(i253 %6)
  %8 = call i253 @"into_box<felt>"(i253 %7)
  %9 = call i253 @"store_temp<Box<felt>>"(i253 %2)
  %10 = call i253 @"store_temp<Box<felt>>"(i253 %5)
  %11 = call i253 @"store_temp<Box<felt>>"(i253 %8)
  %12 = call { i253 } @"fib_box::fib_box::fib"(i253 %9, i253 %10, i253 %11)
  %res_ptr = alloca { i253 }, align 8
  store { i253 } %12, ptr %res_ptr, align 4
  %"6_ptr" = getelementptr inbounds { i253 }, ptr %res_ptr, i32 0, i32 0
  %"6" = load i253, ptr %"6_ptr", align 4
  %13 = call i253 @"unbox<felt>"(i253 %"6")
  %14 = call i253 @"store_temp<felt>"(i253 %13)
  %ret_struct_ptr = alloca { i253 }, align 8
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i253 %14, ptr %field_0_ptr, align 4
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4
  %return_value_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0
  %return_value = load i253, ptr %return_value_ptr, align 4
  %format = alloca [15 x i8], align 1
  store [15 x i8] c"Return value: \00", ptr %format, align 1
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format)
  call void @print_return(i253 %return_value)
  ret i32 0
}
