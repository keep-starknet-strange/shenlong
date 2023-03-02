; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(ptr, ...)

define void @print_felt(i253 %0) {
entry:
  %rounded_size_val = zext i253 %0 to i256
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

define i253 @"felt_const<6>"() {
entry:
  ret i253 7
}

define i253 @"felt_const<2>"() {
entry:
  ret i253 2
}

define i253 @"store_temp<felt>"(i253 %0) {
entry:
  ret i253 %0
}

define i253 @felt_div(i253 %0, i253 %1) {
entry:
  %x = alloca i512, align 8
  %y = alloca i512, align 8
  %r = alloca i512, align 8
  %s = alloca i512, align 8
  store i512 0, ptr %x, align 4
  store i512 1, ptr %y, align 4
  store i512 3618502788666131213697322783095070105623107215331596699973092056135872020481, ptr %r, align 4
  %s1 = sext i253 %1 to i512
  store i512 %s1, ptr %s, align 4
  br label %while

while:                                            ; preds = %body, %entry
  %s_check = load i512, ptr %s, align 4
  %while_compare = icmp ne i512 %s_check, 0
  br i1 %while_compare, label %body, label %exit

body:                                             ; preds = %while
  %r2 = load i512, ptr %r, align 4
  %s3 = load i512, ptr %s, align 4
  %q = sdiv i512 %r2, %s3 ; valid
  %q_mul_s = mul i512 %q, %s3
  %q_mul_s_mod = srem i512 %q_mul_s, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %new_s = sub i512 %r2, %q_mul_s_mod ; valid
  %new_s_mod = srem i512 %new_s, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  
  %new_r = load i512, ptr %s, align 4 ;valid

  store i512 %new_s_mod, ptr %s, align 4
  store i512 %new_r, ptr %r, align 4
  %x4 = load i512, ptr %x, align 4
  %y5 = load i512, ptr %y, align 4
  %q_mul_y = mul i512 %q, %y5
  %q_mul_y_mod = srem i512 %q_mul_y, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %new_y = sub i512 %x4, %q_mul_y_mod
  %new_x = load i512, ptr %y, align 4
  store i512 %new_y, ptr %y, align 4
  store i512 %new_x, ptr %x, align 4
  br label %while

exit:                                             ; preds = %while
  %extended_a = sext i253 %0 to i512
  %inverse = load i512, ptr %x, align 4
  %inv_mod = srem i512 %inverse, 3618502788666131213697322783095070105623107215331596699973092056135872020481 ; valid
  %format1 = alloca [10 x i8], align 1
  %res = mul i512 %extended_a, %inv_mod
  %format2 = alloca [6 x i8], align 1
  store [10 x i8] c"Res:     \00", ptr %format, align 1
  call i32 (ptr, ...) @printf(ptr %format)
  call void @print_double_felt(i512 %res)

  %res6 = call i253 @modulo(i512 %res)
  %format3 = alloca [10 x i8], align 1
  store [10 x i8] c"Res mod: \00", ptr %format1, align 1
  call i32 (ptr, ...) @printf(ptr %format1)
  call void @print_felt(i253 %res6)
  ret i253 %res6
}

define i253 @"rename<felt>"(i253 %0) {
entry:
  ret i253 %0
}

define void @print_return(i253 %0) {
entry:
  %rounded_size_val = zext i253 %0 to i256
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
  %0 = call i253 @"felt_const<6>"()
  %1 = call i253 @"felt_const<2>"()
  %2 = call i253 @"store_temp<felt>"(i253 %0)
  %3 = call i253 @felt_div(i253 %2, i253 %1)
  %4 = call i253 @"store_temp<felt>"(i253 %3)
  %5 = call i253 @"rename<felt>"(i253 %4)
  %ret_struct_ptr = alloca { i253 }, align 8
  %field_0_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0
  store i253 %5, ptr %field_0_ptr, align 4
  %return_struct_value = load { i253 }, ptr %ret_struct_ptr, align 4
  %return_value_ptr = getelementptr inbounds { i253 }, ptr %ret_struct_ptr, i32 0, i32 0
  %return_value = load i253, ptr %return_value_ptr, align 4
  %format = alloca [15 x i8], align 1
  store [15 x i8] c"Return value: \00", ptr %format, align 1
  %chars_printed = call i32 (ptr, ...) @printf(ptr %format)
  call void @print_return(i253 %return_value)
  ret i32 0
}
