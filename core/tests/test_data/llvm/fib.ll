; ModuleID = 'root'
source_filename = "root"

define { i252, i252 } @"dup<felt>"(i252 %0) {
entry:
  %res_ptr = alloca { i252, i252 }
  %tuple_ptr = getelementptr inbounds { i252, i252 }, { i252, i252 }* %res_ptr, i32 0, i32 0
  store i252 %0, i252* %tuple_ptr
  %tuple_ptr_2 = getelementptr inbounds { i252, i252 }, { i252, i252 }* %res_ptr, i32 0, i32 1
  store i252 %0, i252* %tuple_ptr_2
  %res = load { i252, i252 }, { i252, i252 }* %res_ptr
  ret { i252, i252 } %res
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

define i252 @"felt_const<1>"() {
entry:
  ret i252 1
}

define i252 @felt_sub(i252 %0, i252 %1) {
entry:
  %res = sub i252 %0, %1
  ret i252 %res
}

define i252 @"rename<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define { i252 } @"fib::fib::fib"(i252 %0, i252 %1, i252 %2) {
entry:
  %3 = call { i252, i252 } @"dup<felt>"(i252 %2)
  %res_ptr = alloca { i252, i252 }
  store { i252, i252 } %3, { i252, i252 }* %res_ptr
  %"2_ptr" = getelementptr inbounds { i252, i252 }, { i252, i252 }* %res_ptr, i32 0, i32 0
  %"2" = load i252, i252* %"2_ptr"
  %"3_ptr" = getelementptr inbounds { i252, i252 }, { i252, i252 }* %res_ptr, i32 0, i32 1
  %"3" = load i252, i252* %"3_ptr"
  %check = icmp eq i252 %"3", 0
  br i1 %check, label %then, label %else

then:                                             ; preds = %entry
  %4 = call i252 @"store_temp<felt>"(i252 %0)
  %5 = call i252 @"rename<felt>"(i252 %4)
  %ret_struct_ptr = alloca { i252 }
  %field_0_ptr = getelementptr inbounds { i252 }, { i252 }* %ret_struct_ptr, i32 0, i32 0
  store i252 %5, i252* %field_0_ptr
  %return_struct_value = load { i252 }, { i252 }* %ret_struct_ptr
  ret { i252 } %return_struct_value

else:                                             ; preds = %entry
  %6 = call { i252, i252 } @"dup<felt>"(i252 %1)
  %res_ptr1 = alloca { i252, i252 }
  store { i252, i252 } %6, { i252, i252 }* %res_ptr1
  %"1_ptr" = getelementptr inbounds { i252, i252 }, { i252, i252 }* %res_ptr1, i32 0, i32 0
  %"1" = load i252, i252* %"1_ptr"
  %"7_ptr" = getelementptr inbounds { i252, i252 }, { i252, i252 }* %res_ptr1, i32 0, i32 1
  %"7" = load i252, i252* %"7_ptr"
  %7 = call i252 @felt_add(i252 %0, i252 %"7")
  %8 = call i252 @"felt_const<1>"()
  %9 = call i252 @felt_sub(i252 %"2", i252 %8)
  %10 = call i252 @"store_temp<felt>"(i252 %"1")
  %11 = call i252 @"store_temp<felt>"(i252 %7)
  %12 = call i252 @"rename<felt>"(i252 %11)
  %13 = call i252 @"store_temp<felt>"(i252 %9)
  %14 = call i252 @"rename<felt>"(i252 %13)
  %15 = call { i252 } @"fib::fib::fib"(i252 %10, i252 %12, i252 %14)
  %res_ptr2 = alloca { i252 }
  store { i252 } %15, { i252 }* %res_ptr2
  %"10_ptr" = getelementptr inbounds { i252 }, { i252 }* %res_ptr2, i32 0, i32 0
  %"10" = load i252, i252* %"10_ptr"
  %16 = call i252 @"rename<felt>"(i252 %"10")
  %17 = call i252 @"rename<felt>"(i252 %16)
  %ret_struct_ptr3 = alloca { i252 }
  %field_0_ptr4 = getelementptr inbounds { i252 }, { i252 }* %ret_struct_ptr3, i32 0, i32 0
  store i252 %17, i252* %field_0_ptr4
  %return_struct_value5 = load { i252 }, { i252 }* %ret_struct_ptr3
  ret { i252 } %return_struct_value5
}
