; ModuleID = 'Python compiler'
source_filename = "Python compiler"

define internal void @main() {
entry:
  %f = alloca float
  %fi = alloca float
  %i = alloca float
  %f1 = alloca float
  %f0 = alloca float
  %n = alloca float
  store float 6.000000e+00, float* %n
  store float 0.000000e+00, float* %f0
  store float 1.000000e+00, float* %f1
  store float 0.000000e+00, float* %i
  br label %whileCondBlock

whileCondBlock:                                   ; preds = %ifContinueBlock, %entry
  br i1 true, label %whileBlock, label %whileContinueBlock

whileBlock:                                       ; preds = %whileCondBlock
  %f01 = load float, float* %f0
  %f12 = load float, float* %f1
  %addtmp = fadd float %f01, %f12
  store float %addtmp, float* %fi
  %f13 = load float, float* %f1
  store float %f13, float* %f0
  %fi4 = load float, float* %fi
  store float %fi4, float* %f1
  %i5 = load float, float* %i
  %addtmp6 = fadd float %i5, 1.000000e+00
  store float %addtmp6, float* %i
  %i7 = load float, float* %i
  %n8 = load float, float* %n
  %gtetmp = fcmp uge float %i7, %n8
  %booltmp = uitofp i1 %gtetmp to float
  %ifcond = fcmp one float %booltmp, 0.000000e+00
  br i1 %ifcond, label %ifBlock, label %ifContinueBlock

ifBlock:                                          ; preds = %whileBlock
  br label %whileContinueBlock

ifContinueBlock:                                  ; preds = %whileBlock
  br label %whileCondBlock

whileContinueBlock:                               ; preds = %ifBlock, %whileCondBlock
  %f09 = load float, float* %f0
  store float %f09, float* %f
  ret void
}
