; ModuleID = 'root'
source_filename = "root"

define i252 @modulo(i503 %0) {
entry:
  %val_ptr = alloca i503
  store i503 %0, i503* %val_ptr
  br label %start

start:                                            ; preds = %body, %entry
  %val = load i503, i503* %val_ptr
  %compare = icmp ult i503 3618502788666131213697322783095070105623107215331596699973092056135872020481, %val
  br i1 %compare, label %body, label %end

body:                                             ; preds = %start
  %value = load i503, i503* %val_ptr
  %sub = sub i503 %value, 3618502788666131213697322783095070105623107215331596699973092056135872020481
  store i503 %sub, i503* %val_ptr
  br label %start

end:                                              ; preds = %start
  %val1 = load i503, i503* %val_ptr
  %res = trunc i503 %val1 to i252
  ret i252 %res
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
  %res = add i252 %0, %1
  %arg = zext i252 %res to i503
  %res1 = call i252 @modulo(i503 %arg)
  ret i252 %res1
}

define i252 @"rename<felt>"(i252 %0) {
entry:
  ret i252 %0
}

define { i252 } @main() {
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
