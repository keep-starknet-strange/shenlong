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
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %prefix, align 1
  %rounded_size_val = sext i252 %0 to i256
  %shifted = lshr i256 %rounded_size_val, 224
  %print_value_trunc = trunc i256 %shifted to i32
  %printed_chars = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc)
  %shifted1 = lshr i256 %rounded_size_val, 192
  %print_value_trunc2 = trunc i256 %shifted1 to i32
  %printed_chars3 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc2)
  %shifted4 = lshr i256 %rounded_size_val, 160
  %print_value_trunc5 = trunc i256 %shifted4 to i32
  %printed_chars6 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc5)
  %shifted7 = lshr i256 %rounded_size_val, 128
  %print_value_trunc8 = trunc i256 %shifted7 to i32
  %printed_chars9 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc8)
  %shifted10 = lshr i256 %rounded_size_val, 96
  %print_value_trunc11 = trunc i256 %shifted10 to i32
  %printed_chars12 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc11)
  %shifted13 = lshr i256 %rounded_size_val, 64
  %print_value_trunc14 = trunc i256 %shifted13 to i32
  %printed_chars15 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc14)
  %shifted16 = lshr i256 %rounded_size_val, 32
  %print_value_trunc17 = trunc i256 %shifted16 to i32
  %printed_chars18 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc17)
  %shifted19 = lshr i256 %rounded_size_val, 0
  %print_value_trunc20 = trunc i256 %shifted19 to i32
  %printed_chars21 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc20)
  %prefix_newline = alloca [2 x i8], align 1
  store [2 x i8] c"\0A\00", ptr %prefix_newline, align 1
  %printed_chars22 = call i32 (ptr, ...) @printf(ptr %prefix_newline)
  ret void
}

define void @print_double_felt(i503 %0) {
entry:
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%08X\00", ptr %prefix, align 1
  %rounded_size_val = sext i503 %0 to i512
  %shifted = lshr i512 %rounded_size_val, 480
  %print_value_trunc = trunc i512 %shifted to i32
  %printed_chars = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc)
  %shifted1 = lshr i512 %rounded_size_val, 448
  %print_value_trunc2 = trunc i512 %shifted1 to i32
  %printed_chars3 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc2)
  %shifted4 = lshr i512 %rounded_size_val, 416
  %print_value_trunc5 = trunc i512 %shifted4 to i32
  %printed_chars6 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc5)
  %shifted7 = lshr i512 %rounded_size_val, 384
  %print_value_trunc8 = trunc i512 %shifted7 to i32
  %printed_chars9 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc8)
  %shifted10 = lshr i512 %rounded_size_val, 352
  %print_value_trunc11 = trunc i512 %shifted10 to i32
  %printed_chars12 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc11)
  %shifted13 = lshr i512 %rounded_size_val, 320
  %print_value_trunc14 = trunc i512 %shifted13 to i32
  %printed_chars15 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc14)
  %shifted16 = lshr i512 %rounded_size_val, 288
  %print_value_trunc17 = trunc i512 %shifted16 to i32
  %printed_chars18 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc17)
  %shifted19 = lshr i512 %rounded_size_val, 256
  %print_value_trunc20 = trunc i512 %shifted19 to i32
  %printed_chars21 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc20)
  %shifted22 = lshr i512 %rounded_size_val, 224
  %print_value_trunc23 = trunc i512 %shifted22 to i32
  %printed_chars24 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc23)
  %shifted25 = lshr i512 %rounded_size_val, 192
  %print_value_trunc26 = trunc i512 %shifted25 to i32
  %printed_chars27 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc26)
  %shifted28 = lshr i512 %rounded_size_val, 160
  %print_value_trunc29 = trunc i512 %shifted28 to i32
  %printed_chars30 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc29)
  %shifted31 = lshr i512 %rounded_size_val, 128
  %print_value_trunc32 = trunc i512 %shifted31 to i32
  %printed_chars33 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc32)
  %shifted34 = lshr i512 %rounded_size_val, 96
  %print_value_trunc35 = trunc i512 %shifted34 to i32
  %printed_chars36 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc35)
  %shifted37 = lshr i512 %rounded_size_val, 64
  %print_value_trunc38 = trunc i512 %shifted37 to i32
  %printed_chars39 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc38)
  %shifted40 = lshr i512 %rounded_size_val, 32
  %print_value_trunc41 = trunc i512 %shifted40 to i32
  %printed_chars42 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc41)
  %shifted43 = lshr i512 %rounded_size_val, 0
  %print_value_trunc44 = trunc i512 %shifted43 to i32
  %printed_chars45 = call i32 (ptr, ...) @printf(ptr %prefix, i32 %print_value_trunc44)
  %prefix_newline = alloca [2 x i8], align 1
  store [2 x i8] c"\0A\00", ptr %prefix_newline, align 1
  %printed_chars46 = call i32 (ptr, ...) @printf(ptr %prefix_newline)
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
