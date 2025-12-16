.global kernel_sp
.type kernel_sp, @function
kernel_sp:
  mv sp, a0
  j main
