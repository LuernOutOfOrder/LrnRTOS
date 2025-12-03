// Architecture module, import all architecture specific module, like arm or riscv and only use the
// module for actual target. Allow to import or use specific function without importing it from a
// specific arch. 
// Exemple:
//
// arch::interrupt::halt();
//
// We don't need to specify the arch. But this forces to always keep the same functions, or module
// names accross all arch.
#[cfg(target_arch = "riscv32")]
pub mod riscv32;

#[cfg(target_arch = "riscv32")]
pub use self::riscv32::*;
