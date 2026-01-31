; LLVM IR Example

; Module-level declarations
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Global variables
@.str = private unnamed_addr constant [14 x i8] c"Hello, World!\00", align 1
@global_var = global i32 42, align 4

; External function declarations
declare i32 @printf(i8* nocapture readonly, ...) nounwind
declare i8* @malloc(i64) nounwind
declare void @free(i8*) nounwind

; Function definition with attributes
define i32 @factorial(i32 %n) #0 {
entry:
  %cmp = icmp sle i32 %n, 1
  br i1 %cmp, label %base_case, label %recursive_case

base_case:
  ret i32 1

recursive_case:
  %sub = sub nsw i32 %n, 1
  %call = call i32 @factorial(i32 %sub)
  %mul = mul nsw i32 %n, %call
  ret i32 %mul
}

; Main function
define i32 @main(i32 %argc, i8** %argv) #0 {
entry:
  ; Allocate local variable
  %result = alloca i32, align 4

  ; Store initial value
  store i32 0, i32* %result, align 4

  ; Call printf
  %str_ptr = getelementptr inbounds [14 x i8], [14 x i8]* @.str, i64 0, i64 0
  %call = call i32 (i8*, ...) @printf(i8* %str_ptr)

  ; Compute factorial
  %fact = call i32 @factorial(i32 5)
  store i32 %fact, i32* %result, align 4

  ; Load and return
  %ret_val = load i32, i32* %result, align 4
  ret i32 %ret_val
}

; Struct type definition
%struct.Point = type { double, double }

; Function with struct parameter
define double @distance(%struct.Point* %p1, %struct.Point* %p2) {
entry:
  %x1_ptr = getelementptr inbounds %struct.Point, %struct.Point* %p1, i32 0, i32 0
  %x1 = load double, double* %x1_ptr
  %y1_ptr = getelementptr inbounds %struct.Point, %struct.Point* %p1, i32 0, i32 1
  %y1 = load double, double* %y1_ptr

  %x2_ptr = getelementptr inbounds %struct.Point, %struct.Point* %p2, i32 0, i32 0
  %x2 = load double, double* %x2_ptr
  %y2_ptr = getelementptr inbounds %struct.Point, %struct.Point* %p2, i32 0, i32 1
  %y2 = load double, double* %y2_ptr

  %dx = fsub double %x2, %x1
  %dy = fsub double %y2, %y1
  %dx2 = fmul double %dx, %dx
  %dy2 = fmul double %dy, %dy
  %sum = fadd double %dx2, %dy2
  %dist = call double @llvm.sqrt.f64(double %sum)
  ret double %dist
}

; LLVM intrinsic declaration
declare double @llvm.sqrt.f64(double)

; Function attributes
attributes #0 = { nounwind uwtable "frame-pointer"="all" }
