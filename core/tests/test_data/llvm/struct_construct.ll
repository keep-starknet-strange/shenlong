; ModuleID = 'root'
source_filename = "root"
target triple = "x86_64-pc-linux-gnu"

define i252 @modulo(i503 %0) {
entry:
  %modulus = srem i503 %0, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  %res = trunc i503 %modulus to i252
  ret i252 %res
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
