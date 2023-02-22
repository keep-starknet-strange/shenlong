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

define i252 @"felt_const<2>"() {
entry:
  ret i252 2
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

define i252 @"felt_const<100>"() {
entry:
  ret i252 100
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

define { {} } @"fib_caller::fib_caller::fib_mid"(i252 %0) {
entry:
  %1 = call { i252, i252 } @"dup<felt>"(i252 %0)
  %res_ptr = alloca { i252, i252 }, align 8
  store { i252, i252 } %1, ptr %res_ptr, align 4
  %"0_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 0
  %"0" = load i252, ptr %"0_ptr", align 4
  %"2_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 1
  %"2" = load i252, ptr %"2_ptr", align 4
  %check = icmp eq i252 %"2", 0
  br i1 %check, label %then, label %else

then:                                             ; preds = %entry
  br label %dest

else:                                             ; preds = %entry
  br label %dest1

dest:                                             ; preds = %then
  %2 = call {} @"struct_construct<Unit>"()
  %3 = call {} @"store_temp<Unit>"({} %2)
  %ret_struct_ptr = alloca { {} }, align 8
  %field_0_ptr = getelementptr inbounds { {} }, ptr %ret_struct_ptr, i32 0, i32 0
  store {} %3, ptr %field_0_ptr, align 1
  %return_struct_value = load { {} }, ptr %ret_struct_ptr, align 1
  ret { {} } %return_struct_value

dest1:                                            ; preds = %else
  %4 = call i252 @"felt_const<1>"()
  %5 = call i252 @"felt_const<2>"()
  %6 = call i252 @"felt_const<500>"()
  %7 = call i252 @"store_temp<felt>"(i252 %4)
  %8 = call i252 @"store_temp<felt>"(i252 %5)
  %9 = call i252 @"store_temp<felt>"(i252 %6)
  %10 = call { i252 } @"fib_caller::fib_caller::fib"(i252 %7, i252 %8, i252 %9)
  %res_ptr2 = alloca { i252 }, align 8
  store { i252 } %10, ptr %res_ptr2, align 4
  %"6_ptr" = getelementptr inbounds { i252 }, ptr %res_ptr2, i32 0, i32 0
  %"6" = load i252, ptr %"6_ptr", align 4
  %11 = call i252 @"felt_const<1>"()
  %12 = call i252 @felt_sub(i252 %"0", i252 %11)
  %13 = call i252 @"store_temp<felt>"(i252 %12)
  %14 = call { {} } @"fib_caller::fib_caller::fib_mid"(i252 %13)
  %res_ptr3 = alloca { {} }, align 8
  store { {} } %14, ptr %res_ptr3, align 1
  %"12_ptr" = getelementptr inbounds { {} }, ptr %res_ptr3, i32 0, i32 0
  %"12" = load {}, ptr %"12_ptr", align 1
  %15 = call {} @"struct_construct<Unit>"()
  %16 = call {} @"store_temp<Unit>"({} %15)
  %ret_struct_ptr4 = alloca { {} }, align 8
  %field_0_ptr5 = getelementptr inbounds { {} }, ptr %ret_struct_ptr4, i32 0, i32 0
  store {} %16, ptr %field_0_ptr5, align 1
  %return_struct_value6 = load { {} }, ptr %ret_struct_ptr4, align 1
  ret { {} } %return_struct_value6
}

declare i32 @printf(ptr, ...)

define i32 @print({ {} } %0) {
entry:
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%ld\0A\00", ptr %prefix, align 1
  %worked = call i32 (ptr, ...) @printf(ptr %prefix, { {} } %0)
  ret i32 %worked
}

define i32 @main(i252 %0) {
entry:
  %1 = call i252 @"felt_const<100>"()
  %2 = call i252 @"store_temp<felt>"(i252 %1)
  %3 = call { {} } @"fib_caller::fib_caller::fib_mid"(i252 %2)
  %res_ptr = alloca { {} }, align 8
  store { {} } %3, ptr %res_ptr, align 1
  %"2_ptr" = getelementptr inbounds { {} }, ptr %res_ptr, i32 0, i32 0
  %"2" = load {}, ptr %"2_ptr", align 1
  %4 = call {} @"struct_construct<Unit>"()
  %5 = call {} @"store_temp<Unit>"({} %4)
  %ret_struct_ptr = alloca { {} }, align 8
  %field_0_ptr = getelementptr inbounds { {} }, ptr %ret_struct_ptr, i32 0, i32 0
  store {} %5, ptr %field_0_ptr, align 1
  %return_struct_value = load { {} }, ptr %ret_struct_ptr, align 1
  ret i32 0
}
