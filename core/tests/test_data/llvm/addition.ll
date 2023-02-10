; ModuleID = 'root'
source_filename = "root"

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
  %res = add i252 %0, %1
  ret i252 %res
}

define i252 @"rename<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define { i252 } @"add::add::main"() {
entry:
  %0 = call i252 @"felt_const<1>"()
  %1 = call i252 @"felt_const<2>"()
  %2 = call i252 @"store_temp<felt>"(i252 %0)
  %3 = call i252 @felt_add(i252 %2, i252 %1)
  %4 = call i252 @"store_temp<felt>"(i252 %3)
  %5 = call i252 @"rename<felt>"(i252 %4)
  %ret_struct_ptr = alloca { i252 }
  %field_0_ptr = getelementptr inbounds { i252 }, { i252 }* %ret_struct_ptr, i32 0, i32 0
  store i252 %5, i252* %field_0_ptr
  %return_struct_value = load { i252 }, { i252 }* %ret_struct_ptr
  ret { i252 } %return_struct_value
}
