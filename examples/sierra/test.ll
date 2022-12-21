; ModuleID = 'dumb'
source_filename = "dumb"

define i32 @a_1() {
entry:
  ret i32 62
}

define i32 @a_2() {
entry:
  ret i32 1
}

define i32 @a_0(i32 %0, i32 %1) {
entry:
  %sum = add i32 %0, %1
  ret i32 %sum
}

define i32 @a_4(i32 %0) {
entry:
  ret i32 %0
}

define i32 @main(i32 %0) {
entry:
  %ptr1 = alloca i32
  %val1 = call i32 @a_1()
  store i32 %val1, i32* %ptr1
  %ptr2 = alloca i32
  %val2 = call i32 @a_2()
  store i32 %val2, i32* %ptr2
  %"2" = load i32, i32* %ptr2
  %ptr3 = alloca i32
  %val3 = call i32 @a_0(i32 %0, i32 %"2")
  store i32 %val3, i32* %ptr3
  %"3" = load i32, i32* %ptr3
  %"1" = load i32, i32* %ptr1
  %ptr4 = alloca i32
  %val4 = call i32 @a_0(i32 %"3", i32 %"1")
  store i32 %val4, i32* %ptr4
  %"4" = load i32, i32* %ptr4
  %ptr5 = alloca i32
  %val5 = call i32 @a_4(i32 %"4")
  store i32 %val5, i32* %ptr5
  %"5" = load i32, i32* %ptr5
  ret i32 %"5"
}
