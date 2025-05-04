; ModuleID = 'main_module'
source_filename = "main_module"

declare i32 @printf(ptr, ...)

define i32 @main(i32 %0, ptr %1) {
entry:
  call i32 @printf(i8 5)
  ret i32 0
}

define i32 @a() {
entry:
  ret i32 0
}
