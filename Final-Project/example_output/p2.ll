; ModuleID = 'Python compiler'
source_filename = "Python compiler"

define internal void @main() {
entry:
  %z = alloca float
  %y = alloca float
  %x = alloca float
  %b = alloca float
  %a = alloca float
  store float 1.000000e+00, float* %a
  store float 0.000000e+00, float* %b
  store float 7.000000e+00, float* %x
  store float 1.000000e+00, float* %y
  %a1 = load float, float* %a
  %ifcond4 = fcmp one float %a1, 0.000000e+00
  br i1 %ifcond4, label %ifBlock, label %ifContinueBlock5

ifBlock:                                          ; preds = %entry
  store float 5.000000e+00, float* %x
  %b2 = load float, float* %b
  %ifcond = fcmp one float %b2, 0.000000e+00
  br i1 %ifcond, label %ifBlock3, label %elseBlock

ifBlock3:                                         ; preds = %ifBlock
  store float 4.000000e+00, float* %y
  br label %ifContinueBlock

elseBlock:                                        ; preds = %ifBlock
  store float 2.000000e+00, float* %y
  br label %ifContinueBlock

ifContinueBlock:                                  ; preds = %elseBlock, %ifBlock3
  br label %ifContinueBlock5

ifContinueBlock5:                                 ; preds = %entry, %ifContinueBlock
  %x6 = load float, float* %x
  %multmp = fmul float %x6, 3.000000e+00
  %multmp7 = fmul float %multmp, 7.000000e+00
  %y8 = load float, float* %y
  %divtmp = fdiv float %multmp7, %y8
  store float %divtmp, float* %z
  %z9 = load float, float* %z
  %gttmp = fcmp ugt float %z9, 1.000000e+01
  %booltmp = uitofp i1 %gttmp to float
  %ifcond11 = fcmp one float %booltmp, 0.000000e+00
  br i1 %ifcond11, label %ifBlock10, label %ifContinueBlock12

ifBlock10:                                        ; preds = %ifContinueBlock5
  store float 5.000000e+00, float* %y
  br label %ifContinueBlock12

ifContinueBlock12:                                ; preds = %ifContinueBlock5, %ifBlock10
  ret void
}
