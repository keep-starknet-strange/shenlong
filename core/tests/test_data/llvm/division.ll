; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

define i252 @modulo(i503 %0) {
entry:
  %modulus = srem i503 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %res = trunc i503 %modulus to i252
  ret i252 %res
}

define i252 @"felt_const<6>"() {
entry:
  ret i252 6
}

define i252 @"felt_const<2>"() {
entry:
  ret i252 2
}

define i252 @"store_temp<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define i252 @felt_div(i252 %0, i252 %1) {
entry:
  %t = alloca i503, align 8
  %new_t = alloca i503, align 8
  %r = alloca i503, align 8
  %new_r = alloca i503, align 8
  store i503 0, ptr %t, align 4
  store i503 1, ptr %new_t, align 4
  store i503 3618502788666131213697322783095070105623107215331596699973092056135872020481, ptr %r, align 4
  %new_r_extended = sext i252 %1 to i503
  store i503 %new_r_extended, ptr %new_r, align 4
  br label %while

while:                                            ; preds = %body, %entry
  %new_r_value = load i503, ptr %new_r, align 4
  %while_compare = icmp ne i503 %new_r_value, 0
  br i1 %while_compare, label %body, label %exit

body:                                             ; preds = %while
  %r_value = load i503, ptr %r, align 4
  %new_r_value1 = load i503, ptr %new_r, align 4
  %new_quotient_value = sdiv i503 %r_value, %new_r_value1
  %old_t_value = load i503, ptr %t, align 4
  %new_t_value = load i503, ptr %new_t, align 4
  store i503 %new_t_value, ptr %t, align 4
  %quotient_mul_new_t = mul i503 %new_quotient_value, %new_t_value
  %sub_t_res = sub i503 %old_t_value, %quotient_mul_new_t
  store i503 %sub_t_res, ptr %new_t, align 4
  %old_r_value = load i503, ptr %r, align 4
  %new_r_value2 = load i503, ptr %new_r, align 4
  store i503 %new_r_value2, ptr %r, align 4
  %quotient_mul_new_r = mul i503 %new_quotient_value, %new_r_value2
  %sub_r_res = sub i503 %old_r_value, %quotient_mul_new_r
  store i503 %sub_r_res, ptr %new_r, align 4
  br label %while

exit:                                             ; preds = %while
  %extended_a = sext i252 %0 to i503
  %inverse = load i503, ptr %t, align 4
  %res = mul i503 %extended_a, %inverse
  %res3 = call i252 @modulo(i503 %res)
  ret i252 %res3
}

define i252 @"rename<felt>"(i252 %0) {
entry:
  ret i252 %0
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
  %worked = call i32 @print({ i252 } %return_struct_value)
  ret i32 %worked
}
