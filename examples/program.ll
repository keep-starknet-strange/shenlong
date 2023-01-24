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

define i252 @"0"(i252 %0, i252 %1) {
entry:
  %res = add i252 %0, %1
  ret i252 %res
}

define i252 @main() {
entry:
  %"0_ptr" = alloca i252
  %call_felt_add = call i252 @"1"()
  store i252 %call_felt_add, i252* %"0_ptr"
  %"1_ptr" = alloca i252
  %call_felt_add1 = call i252 @"2"()
  store i252 %call_felt_add1, i252* %"1_ptr"
  %"0" = load i252, i252* %"0_ptr"
  %"0_ptr2" = alloca i252
  store i252 %"0", i252* %"0_ptr2"
  %"03" = load i252, i252* %"0_ptr2"
  %"1" = load i252, i252* %"1_ptr"
  %"2_ptr" = alloca i252
  %call_felt_add4 = call i252 @"0"(i252 %"03", i252 %"1")
  store i252 %call_felt_add4, i252* %"2_ptr"
  %"2" = load i252, i252* %"2_ptr"
  %"2_ptr5" = alloca i252
  store i252 %"2", i252* %"2_ptr5"
  %"26" = load i252, i252* %"2_ptr5"
  %"3_ptr" = alloca i252
  store i252 %"26", i252* %"3_ptr"
  %"3" = load i252, i252* %"3_ptr"
  ret i252 %"3"
}
