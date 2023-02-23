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

define i252 @"store_temp<felt>"(i252 %0) {
entry:
  ret i252 %0
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

define i252 @"rename<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define i252 @"felt_const<0>"() {
entry:
  ret i252 0
}

define i252 @"felt_const<500>"() {
entry:
  ret i252 500
}

define {} @"struct_construct<Unit>"() {
entry:
  %res_ptr = alloca {}, align 8
  %res = load {}, ptr %res_ptr, align 1
  ret {} %res
}

define {} @"store_temp<Unit>"({} %0) {
entry:
  ret {} %0
}

define { i252 } @"fib_caller::fib_caller::fib"(i252 %0, i252 %1, i252 %2) {
entry:
  %3 = call { i252, i252 } @"dup<felt>"(i252 %2)
  %res_ptr = alloca { i252, i252 }, align 8
  store { i252, i252 } %3, ptr %res_ptr, align 4
  %"2_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 0
  %"2" = load i252, ptr %"2_ptr", align 4
  %"4_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 1
  %"4" = load i252, ptr %"4_ptr", align 4
  %check = icmp eq i252 %"4", 0
  br i1 %check, label %then, label %else

then:                                             ; preds = %entry
  %4 = call i252 @"store_temp<felt>"(i252 %0)
  br label %dest

else:                                             ; preds = %entry
  br label %dest1

dest:                                             ; preds = %then
  %5 = call i252 @"rename<felt>"(i252 %4)
  %ret_struct_ptr = alloca { i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i252 %5, ptr %field_0_ptr, align 4
  %return_struct_value = load { i252 }, ptr %ret_struct_ptr, align 4
  ret { i252 } %return_struct_value

dest1:                                            ; preds = %else
  %6 = call { i252, i252 } @"dup<felt>"(i252 %1)
  %res_ptr2 = alloca { i252, i252 }, align 8
  store { i252, i252 } %6, ptr %res_ptr2, align 4
  %"1_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr2, i32 0, i32 0
  %"1" = load i252, ptr %"1_ptr", align 4
  %"7_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr2, i32 0, i32 1
  %"7" = load i252, ptr %"7_ptr", align 4
  %7 = call i252 @felt_add(i252 %0, i252 %"7")
  %8 = call i252 @"felt_const<1>"()
  %9 = call i252 @felt_sub(i252 %"2", i252 %8)
  %10 = call i252 @"store_temp<felt>"(i252 %"1")
  %11 = call i252 @"store_temp<felt>"(i252 %7)
  %12 = call i252 @"store_temp<felt>"(i252 %9)
  %13 = call { i252 } @"fib_caller::fib_caller::fib"(i252 %10, i252 %11, i252 %12)
  %res_ptr3 = alloca { i252 }, align 8
  store { i252 } %13, ptr %res_ptr3, align 4
  %"10_ptr" = getelementptr inbounds { i252 }, ptr %res_ptr3, i32 0, i32 0
  %"10" = load i252, ptr %"10_ptr", align 4
  %14 = call i252 @"rename<felt>"(i252 %"10")
  br label %dest4

dest4:                                            ; preds = %dest1
  %15 = call i252 @"rename<felt>"(i252 %14)
  %ret_struct_ptr5 = alloca { i252 }, align 8
  %field_0_ptr6 = getelementptr inbounds { i252 }, ptr %ret_struct_ptr5, i32 0, i32 0
  store i252 %15, ptr %field_0_ptr6, align 4
  %return_struct_value7 = load { i252 }, ptr %ret_struct_ptr5, align 4
  ret { i252 } %return_struct_value7
}

define i32 @main(i252 %0) {
entry:
  %1 = call i252 @"felt_const<0>"()
  %2 = call i252 @"felt_const<1>"()
  %3 = call i252 @"felt_const<500>"()
  %4 = call i252 @"store_temp<felt>"(i252 %1)
  %5 = call i252 @"store_temp<felt>"(i252 %2)
  %6 = call i252 @"store_temp<felt>"(i252 %3)
  %7 = call { i252 } @"fib_caller::fib_caller::fib"(i252 %4, i252 %5, i252 %6)
  %res_ptr = alloca { i252 }, align 8
  store { i252 } %7, ptr %res_ptr, align 4
  %"4_ptr" = getelementptr inbounds { i252 }, ptr %res_ptr, i32 0, i32 0
  %"4" = load i252, ptr %"4_ptr", align 4
  %8 = call {} @"struct_construct<Unit>"()
  %9 = call {} @"store_temp<Unit>"({} %8)
  %ret_struct_ptr = alloca { {} }, align 8
  %field_0_ptr = getelementptr inbounds { {} }, ptr %ret_struct_ptr, i32 0, i32 0
  store {} %9, ptr %field_0_ptr, align 1
  %return_struct_value = load { {} }, ptr %ret_struct_ptr, align 1
  ret i32 0
}
