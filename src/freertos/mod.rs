//基本数据类型
pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_long = i64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type ssize_t = isize;

pub type gid_t = u32;
pub type uid_t = u32;
pub type size_t = usize;
pub type mode_t = u32;

//以下数据类型主要用于栈展开
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_NUM: u32 = 8;
pub const PT_LOOS: u32 = 0x60000000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;
pub const PT_LOSUNW: u32 = 0x6ffffffa;
pub const PT_SUNWBSS: u32 = 0x6ffffffa;
pub const PT_SUNWSTACK: u32 = 0x6ffffffb;
pub const PT_HISUNW: u32 = 0x6fffffff;
pub const PT_HIOS: u32 = 0x6fffffff;
pub const PT_LOPROC: u32 = 0x70000000;
pub const PT_HIPROC: u32 = 0x7fffffff;

pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;

pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

pub type Elf32_Section = u16;
pub type Elf64_Section = u16;

s!{
    pub struct Elf32_Phdr {
        pub p_type: Elf32_Word,
        pub p_offset: Elf32_Off,
        pub p_vaddr: Elf32_Addr,
        pub p_paddr: Elf32_Addr,
        pub p_filesz: Elf32_Word,
        pub p_memsz: Elf32_Word,
        pub p_flags: Elf32_Word,
        pub p_align: Elf32_Word,
    }

    pub struct Elf64_Phdr {
        pub p_type: Elf64_Word,
        pub p_flags: Elf64_Word,
        pub p_offset: Elf64_Off,
        pub p_vaddr: Elf64_Addr,
        pub p_paddr: Elf64_Addr,
        pub p_filesz: Elf64_Xword,
        pub p_memsz: Elf64_Xword,
        pub p_align: Elf64_Xword,
    }

    pub struct dl_phdr_info {
        #[cfg(target_pointer_width = "64")]
        pub dlpi_addr: Elf64_Addr,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_addr: Elf32_Addr,

        pub dlpi_name: *const ::c_char,

        #[cfg(target_pointer_width = "64")]
        pub dlpi_phdr: *const Elf64_Phdr,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_phdr: *const Elf32_Phdr,

        #[cfg(target_pointer_width = "64")]
        pub dlpi_phnum: Elf64_Half,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_phnum: Elf32_Half,

        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_adds: ::c_ulonglong,
        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_subs: ::c_ulonglong,
        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_tls_modid: ::size_t,
        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_tls_data: *mut ::c_void,
    }
}

pub enum FILE {}
impl ::Copy for FILE {}
impl ::Clone for FILE {
    fn clone(&self) -> FILE {
        *self
    }
}

//libc memory interface
extern "C" {
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
}

//libc fs interface
extern "C"{
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn rmdir(path: *const c_char) -> ::c_int;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> ::c_int;
    pub fn stat(path: *const c_char, buf: *mut stat) -> ::c_int;
    pub fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t;
    pub fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int;
    pub fn read(fd: ::c_int, buf: *mut ::c_void, count: ::size_t) -> ::ssize_t;
    pub fn unlink(c: *const c_char) -> ::c_int;
    pub fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t) -> ::ssize_t;
    pub fn open(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn close(fd: ::c_int) -> ::c_int;
}


extern "C"{
    pub fn dl_iterate_phdr(
        callback: ::Option<
            unsafe extern "C" fn(
                info: *mut ::dl_phdr_info,
                size: ::size_t,
                data: *mut ::c_void,
            ) -> ::c_int,
        >,
        data: *mut ::c_void,
    ) -> ::c_int;
}

mod sipeed;
mod newlib;
pub use self::newlib::*;
pub use self::sipeed::*;

cfg_if! {
    if #[cfg(libc_core_cvoid)] {
        pub use ::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}