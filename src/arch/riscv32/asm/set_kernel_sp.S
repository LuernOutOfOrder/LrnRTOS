.global set_kernel_sp
.type set_kernel_sp, @function
set_kernel_sp:
  mv sp, a0
  j main
