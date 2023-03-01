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

define i252 @"felt_const<6>"() {
entry:
  ret i252 9
}

define i252 @"felt_const<2>"() {
entry:
  ret i252 3
}

define i252 @"store_temp<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define i252 @felt_div(i252 %0, i252 %1) {
entry:
  %x = alloca i503, align 8
  %y = alloca i503, align 8
  %r = alloca i503, align 8
  %s = alloca i503, align 8
  store i503 0, ptr %x, align 4
  store i503 1, ptr %y, align 4
  store i503 170141183460469231731687303715884105728, ptr %r, align 4
  %s1 = sext i252 %1 to i503
  store i503 %s1, ptr %s, align 4
  br label %while

while:                                            ; preds = %body, %entry
  %s_check = load i503, ptr %s, align 4
  %while_compare = icmp ne i503 %s_check, 0
  br i1 %while_compare, label %body, label %exit

body:                                             ; preds = %while
  %r2 = load i503, ptr %r, align 4
  %s3 = load i503, ptr %s, align 4
  %q = sdiv i503 %r2, %s3
  %worked = call i32 @print_double_felt(i503 %q)
  %q_mul_s = mul i503 %q, %s3
  %q_mul_s_mod = srem i503 %q_mul_s, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %new_s = sub i503 %r2, %q_mul_s_mod
  %new_s_mod = srem i503 %new_s, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %new_r = load i503, ptr %s, align 4
  store i503 %new_s_mod, ptr %s, align 4
  store i503 %new_r, ptr %r, align 4
  %x4 = load i503, ptr %x, align 4
  %y5 = load i503, ptr %y, align 4
  %q_mul_y = mul i503 %q, %y5
  %q_mul_y_mod = srem i503 %q_mul_y, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %new_y = sub i503 %x4, %q_mul_y_mod
  %new_x = load i503, ptr %y, align 4
  store i503 %new_y, ptr %y, align 4
  store i503 %new_x, ptr %x, align 4
  br label %while

exit:                                             ; preds = %while
  %extended_a = sext i252 %0 to i503
  %inverse = load i503, ptr %x, align 4
  %res = mul i503 %extended_a, %inverse
  %res6 = call i252 @modulo(i503 %res)
  ret i252 %res6
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
  %0 = call i252 @"felt_const<6>"()
  %1 = call i252 @"felt_const<2>"()
  %2 = call i252 @"store_temp<felt>"(i252 %0)
  %3 = call i252 @felt_div(i252 %2, i252 %1)
  %4 = call i252 @"store_temp<felt>"(i252 %3)
  %5 = call i252 @"rename<felt>"(i252 %4)
  %ret_struct_ptr = alloca { i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i252 %5, ptr %field_0_ptr, align 4
  %return_struct_value = load { i252 }, ptr %ret_struct_ptr, align 4
  %worked = call i32 @print_return({ i252 } %return_struct_value)
  ret i32 %worked
}
