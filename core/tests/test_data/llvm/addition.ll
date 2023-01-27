; ModuleID = 'root'
source_filename = "root"

define i252 @"1"() {
entry:
  ret i252 1
}

define i252 @"2"() {
entry:
  ret i252 2
}

define i252 @"3"(i252 %0) {
entry:
  ret i252 %0
}

define i252 @"0"(i252 %0, i252 %1) {
entry:
  %res = add i252 %0, %1
  ret i252 %res
}

define i252 @main() {
entry:
  %"0_ptr" = alloca i252
  %0 = call i252 @"1"()
  store i252 %0, i252* %"0_ptr"
  %"1_ptr" = alloca i252
  %1 = call i252 @"2"()
  store i252 %1, i252* %"1_ptr"
  %"0" = load i252, i252* %"0_ptr"
  %"0_ptr1" = alloca i252
  %2 = call i252 @"3"(i252 %"0")
  store i252 %2, i252* %"0_ptr1"
  %"02" = load i252, i252* %"0_ptr1"
  %"1" = load i252, i252* %"1_ptr"
  %"2_ptr" = alloca i252
  %3 = call i252 @"0"(i252 %"02", i252 %"1")
  store i252 %3, i252* %"2_ptr"
  %"2" = load i252, i252* %"2_ptr"
  %"2_ptr3" = alloca i252
  %4 = call i252 @"3"(i252 %"2")
  store i252 %4, i252* %"2_ptr3"
  %"24" = load i252, i252* %"2_ptr3"
  %"3_ptr" = alloca i252
  store i252 %"24", i252* %"3_ptr"
  %"3" = load i252, i252* %"3_ptr"
  %"4_ptr" = alloca i252
  store i252 %"3", i252* %"4_ptr"
  %"4" = load i252, i252* %"4_ptr"
  ret i252 %"4"
}
