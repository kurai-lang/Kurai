; ModuleID = 'main_module'
source_filename = "main_module"

@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@str = private unnamed_addr constant [4 x i8] c"ily\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, ptr @str)
  ret i32 0
}
