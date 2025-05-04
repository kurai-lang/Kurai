; ModuleID = 'main_module'
source_filename = "main_module"

declare i32 @printf(ptr, ...)

define i32 @main(i32 %0, ptr %1) {
entry:
  %x = alloca i64, align 8
  store i64 3, ptr %x, align 4
  store i64 1, ptr %x, align 4
  ret i32 0
}
