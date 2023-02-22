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

define i32 @print_felt(i252 %0) {
entry:
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%ld\0A\00", ptr %prefix, align 1
  %printed_characters_n = call i32 (ptr, ...) @printf(ptr %prefix, i252 %0)
  ret i32 %printed_characters_n
}

define i32 @print_double_felt(i503 %0) {
entry:
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%ld\0A\00", ptr %prefix, align 1
  %printed_characters_n = call i32 (ptr, ...) @printf(ptr %prefix, i503 %0)
  ret i32 %printed_characters_n
}

define i252 @"felt_const<1>"() {
entry:
  ret i252 1
}

define i252 @"felt_const<2>"() {
entry:
  ret i252 2
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

define i252 @"rename<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define i32 @print_return({ i252 } %0) {
entry:
  %prefix = alloca [5 x i8], align 1
  store [5 x i8] c"%ld\0A\00", ptr %prefix, align 1
  %printed_characters_n = call i32 (ptr, ...) @printf(ptr %prefix, { i252 } %0)
  ret i32 %printed_characters_n
}

define i32 @main() {
entry:
  %0 = call i252 @"felt_const<1>"()
  %1 = call i252 @"felt_const<2>"()
  %2 = call i252 @"store_temp<felt>"(i252 %0)
  %3 = call i252 @felt_add(i252 %2, i252 %1)
  %4 = call i252 @"store_temp<felt>"(i252 %3)
  %5 = call i252 @"rename<felt>"(i252 %4)
  %ret_struct_ptr = alloca { i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i252 %5, ptr %field_0_ptr, align 4
  %return_struct_value = load { i252 }, ptr %ret_struct_ptr, align 4
  %worked = call i32 @print_return({ i252 } %return_struct_value)
  ret i32 %worked
}
