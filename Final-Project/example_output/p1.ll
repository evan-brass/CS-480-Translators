; ModuleID = 'Python compiler'
source_filename = "Python compiler"

define internal void @main() {
entry:
  %sphere_surf_area = alloca float
  %sphere_vol = alloca float
  %circle_circum = alloca float
  %circle_area = alloca float
  %r = alloca float
  %b = alloca float
  %a = alloca float
  %pi = alloca float
  store float 0x400921CAC0000000, float* %pi
  store float 3.000000e+00, float* %a
  store float 2.000000e+00, float* %b
  %a1 = load float, float* %a
  %b2 = load float, float* %b
  %addtmp = fadd float %a1, %b2
  %b3 = load float, float* %b
  %multmp = fmul float %addtmp, %b3
  %b4 = load float, float* %b
  %subtmp = fsub float %multmp, %b4
  store float %subtmp, float* %r
  %pi5 = load float, float* %pi
  %r6 = load float, float* %r
  %multmp7 = fmul float %pi5, %r6
  %r8 = load float, float* %r
  %multmp9 = fmul float %multmp7, %r8
  store float %multmp9, float* %circle_area
  %pi10 = load float, float* %pi
  %multmp11 = fmul float %pi10, 2.000000e+00
  %r12 = load float, float* %r
  %multmp13 = fmul float %multmp11, %r12
  store float %multmp13, float* %circle_circum
  %pi14 = load float, float* %pi
  %multmp15 = fmul float 0x3FF5555560000000, %pi14
  %r16 = load float, float* %r
  %multmp17 = fmul float %multmp15, %r16
  %r18 = load float, float* %r
  %multmp19 = fmul float %multmp17, %r18
  %r20 = load float, float* %r
  %multmp21 = fmul float %multmp19, %r20
  store float %multmp21, float* %sphere_vol
  %pi22 = load float, float* %pi
  %multmp23 = fmul float 4.000000e+00, %pi22
  %r24 = load float, float* %r
  %multmp25 = fmul float %multmp23, %r24
  %r26 = load float, float* %r
  %multmp27 = fmul float %multmp25, %r26
  store float %multmp27, float* %sphere_surf_area
  ret void
}
