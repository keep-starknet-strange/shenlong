; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

define i252 @modulo(i503 %0) {
entry:
  %modulus = srem i503 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %res = trunc i503 %modulus to i252
  ret i252 %res
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

define i252 @"felt_const<0>"() {
entry:
  ret i252 0
}

define i252 @"felt_const<30>"() {
entry:
  ret i252 30
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

declare i32 @printf(ptr, ...)

define i32 @print({ i252 } %0) {
entry:
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%ld\0A\00", ptr %prefix, align 1
  %worked = call i32 (ptr, ...) @printf(ptr %prefix, { i252 } %0)
  ret i32 %worked
}

define i32 @main() {
entry:
  %0 = call i252 @"felt_const<0>"()
  %1 = call i252 @"store_temp<felt>"(i252 %0)
  %2 = call i252 @"into_box<felt>"(i252 %1)
  %3 = call i252 @"felt_const<1>"()
  %4 = call i252 @"store_temp<felt>"(i252 %3)
  %5 = call i252 @"into_box<felt>"(i252 %4)
  %6 = call i252 @"felt_const<30>"()
  %7 = call i252 @"store_temp<felt>"(i252 %6)
  %8 = call i252 @"into_box<felt>"(i252 %7)
  %9 = call i252 @"store_temp<Box<felt>>"(i252 %2)
  %10 = call i252 @"store_temp<Box<felt>>"(i252 %5)
  %11 = call i252 @"store_temp<Box<felt>>"(i252 %8)
  %12 = call { i252 } @"fib_box::fib_box::fib"(i252 %9, i252 %10, i252 %11)
  %res_ptr = alloca { i252 }, align 8
  store { i252 } %12, ptr %res_ptr, align 4
  %"6_ptr" = getelementptr inbounds { i252 }, ptr %res_ptr, i32 0, i32 0
  %"6" = load i252, ptr %"6_ptr", align 4
  %13 = call i252 @"unbox<felt>"(i252 %"6")
  %14 = call i252 @"store_temp<felt>"(i252 %13)
  %ret_struct_ptr = alloca { i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i252 %14, ptr %field_0_ptr, align 4
  %return_struct_value = load { i252 }, ptr %ret_struct_ptr, align 4
  %worked = call i32 @print({ i252 } %return_struct_value)
  ret i32 %worked
}
