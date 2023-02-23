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

define i252 @"felt_const<2>"() {
entry:
  ret i252 2
}

define i252 @"felt_const<4>"() {
entry:
  ret i252 4
}

define { i252, i252 } @"struct_construct<Tuple<felt, felt>>"(i252 %0, i252 %1) {
entry:
  %res_ptr = alloca { i252, i252 }, align 8
  %field_0_ptr = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 0
  store i252 %0, ptr %field_0_ptr, align 4
  %field_1_ptr = getelementptr inbounds { i252, i252 }, ptr %res_ptr, i32 0, i32 1
  store i252 %1, ptr %field_1_ptr, align 4
  %res = load { i252, i252 }, ptr %res_ptr, align 4
  ret { i252, i252 } %res
}

define { i252, i252 } @"store_temp<Tuple<felt, felt>>"({ i252, i252 } %0) {
entry:
  ret { i252, i252 } %0
}

define { { i252, i252 } } @"struct_construct::struct_construct::complex_type"() {
entry:
  %0 = call i252 @"felt_const<2>"()
  %1 = call i252 @"felt_const<4>"()
  %2 = call { i252, i252 } @"struct_construct<Tuple<felt, felt>>"(i252 %0, i252 %1)
  %3 = call { i252, i252 } @"store_temp<Tuple<felt, felt>>"({ i252, i252 } %2)
  %ret_struct_ptr = alloca { { i252, i252 } }, align 8
  %field_0_ptr = getelementptr inbounds { { i252, i252 } }, ptr %ret_struct_ptr, i32 0, i32 0
  store { i252, i252 } %3, ptr %field_0_ptr, align 4
  %return_struct_value = load { { i252, i252 } }, ptr %ret_struct_ptr, align 4
  ret { { i252, i252 } } %return_struct_value
}
