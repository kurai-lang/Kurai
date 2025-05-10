; ModuleID = 'main_module'
source_filename = "main_module"

@fmt = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %printf_call = call i32 (ptr, ...) @printf(ptr @fmt, i64 6942000)
  ret i32 0
}
