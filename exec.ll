; ModuleID = 'main_module'
source_filename = "main_module"

@fmt_0 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str_0_0 = private unnamed_addr constant [4 x i8] c"yo\0A\00", align 1
@fmt_1 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str_1_0 = private unnamed_addr constant [5 x i8] c"ily\0A\00", align 1
@fmt_2 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str_2_0 = private unnamed_addr constant [20 x i8] c"will you marry me?\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0_0)
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1_0)
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_2_0)
  ret i32 0
}
