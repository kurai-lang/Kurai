; ModuleID = 'main_module'
source_filename = "main_module"

@fmt_0 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str_0_0 = private unnamed_addr constant [10 x i8] c"YES I DO!\00", align 1
@fmt_1 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str_1_0 = private unnamed_addr constant [11 x i8] c"Nah i dont\00", align 1
@fmt_2 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str_2_0 = private unnamed_addr constant [18 x i8] c"Do you like sara?\00", align 1

declare i32 @printf(ptr, ...)

define i32 @check() {
entry:
  %do_i_like_sara = alloca i64, align 8
  store i64 10, ptr %do_i_like_sara, align 4
  %load_do_i_like_sara = load i64, ptr %do_i_like_sara, align 4
  %cmp = icmp sgt i64 %load_do_i_like_sara, 10
  br i1 %cmp, label %then_0, label %next_0

merge:                                            ; preds = %else, %then_0
  ret i32 0

then_0:                                           ; preds = %entry
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @fmt_0, ptr @str_0_0)
  br label %merge

next_0:                                           ; preds = %entry
  br label %else

else:                                             ; preds = %next_0
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @fmt_1, ptr @str_1_0)
  br label %merge
}

define i32 @main() {
entry:
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @fmt_2, ptr @str_2_0)
  %check = call i32 @check()
  ret i32 0
}
