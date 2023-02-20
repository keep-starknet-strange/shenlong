; ModuleID = 'root'
source_filename = "root"

define i252 @modulo(i503 %0) {
entry:
  %modulus = srem i503 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %res = trunc i503 %modulus to i252
  ret i252 %res
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
  %5 = call i252 @"rename<felt>"(i252 %4)
  %ret_struct_ptr = alloca { i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i252 %5, ptr %field_0_ptr, align 4
  %return_struct_value = load { i252 }, ptr %ret_struct_ptr, align 4
  ret { i252 } %return_struct_value

else:                                             ; preds = %entry
  %6 = call { i252, i252 } @"dup<felt>"(i252 %1)
  %res_ptr1 = alloca { i252, i252 }, align 8
  store { i252, i252 } %6, ptr %res_ptr1, align 4
  %"1_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr1, i32 0, i32 0
  %"1" = load i252, ptr %"1_ptr", align 4
  %"7_ptr" = getelementptr inbounds { i252, i252 }, ptr %res_ptr1, i32 0, i32 1
  %"7" = load i252, ptr %"7_ptr", align 4
  %7 = call i252 @felt_add(i252 %0, i252 %"7")
  %8 = call i252 @"felt_const<1>"()
  %9 = call i252 @felt_sub(i252 %"2", i252 %8)
  %10 = call i252 @"store_temp<felt>"(i252 %"1")
  %11 = call i252 @"store_temp<felt>"(i252 %7)
  %12 = call i252 @"store_temp<felt>"(i252 %9)
  %13 = call { i252 } @"fib_caller::fib_caller::fib"(i252 %10, i252 %11, i252 %12)
  %res_ptr2 = alloca { i252 }, align 8
  store { i252 } %13, ptr %res_ptr2, align 4
  %"10_ptr" = getelementptr inbounds { i252 }, ptr %res_ptr2, i32 0, i32 0
  %"10" = load i252, ptr %"10_ptr", align 4
  %14 = call i252 @"rename<felt>"(i252 %"10")
  %15 = call i252 @"rename<felt>"(i252 %14)
  %ret_struct_ptr3 = alloca { i252 }, align 8
  %field_0_ptr4 = getelementptr inbounds { i252 }, ptr %ret_struct_ptr3, i32 0, i32 0
  store i252 %15, ptr %field_0_ptr4, align 4
  %return_struct_value5 = load { i252 }, ptr %ret_struct_ptr3, align 4
  ret { i252 } %return_struct_value5
}

declare i32 @printf(ptr, ...)

define i32 @print({ {} } %0) {
entry:
  %prefix = alloca [2 x i8], align 1
  store [2 x i8] c"%d", ptr %prefix, align 1
  %worked = call i32 (ptr, ...) @printf(ptr %prefix, { {} } %0)
  ret i32 %worked
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
