#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}
pub const BSON_BYTE_ORDER: u32 = 1234;
pub const BSON_HAVE_STDBOOL_H: u32 = 1;
pub const BSON_OS: u32 = 1;
pub const BSON_HAVE_ATOMIC_32_ADD_AND_FETCH: u32 = 1;
pub const BSON_HAVE_ATOMIC_64_ADD_AND_FETCH: u32 = 1;
pub const BSON_HAVE_CLOCK_GETTIME: u32 = 1;
pub const BSON_HAVE_STRINGS_H: u32 = 1;
pub const BSON_HAVE_STRNLEN: u32 = 1;
pub const BSON_HAVE_SNPRINTF: u32 = 1;
pub const BSON_HAVE_GMTIME_R: u32 = 1;
pub const BSON_HAVE_REALLOCF: u32 = 0;
pub const BSON_HAVE_TIMESPEC: u32 = 1;
pub const BSON_EXTRA_ALIGN: u32 = 1;
pub const BSON_HAVE_SYSCALL_TID: u32 = 1;
pub const BSON_HAVE_RAND_R: u32 = 1;
pub const _STDIO_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 27;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const _BITS_LIBIO_H: u32 = 1;
pub const _BITS_G_CONFIG_H: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _G_HAVE_MMAP: u32 = 1;
pub const _G_HAVE_MREMAP: u32 = 1;
pub const _G_IO_IO_FILE_VERSION: u32 = 131073;
pub const _G_BUFSIZ: u32 = 8192;
pub const _IO_BUFSIZ: u32 = 8192;
pub const __GNUC_VA_LIST: u32 = 1;
pub const _IO_UNIFIED_JUMPTABLES: u32 = 1;
pub const EOF: i32 = -1;
pub const _IOS_INPUT: u32 = 1;
pub const _IOS_OUTPUT: u32 = 2;
pub const _IOS_ATEND: u32 = 4;
pub const _IOS_APPEND: u32 = 8;
pub const _IOS_TRUNC: u32 = 16;
pub const _IOS_NOCREATE: u32 = 32;
pub const _IOS_NOREPLACE: u32 = 64;
pub const _IOS_BIN: u32 = 128;
pub const _IO_MAGIC: u32 = 4222418944;
pub const _OLD_STDIO_MAGIC: u32 = 4206624768;
pub const _IO_MAGIC_MASK: u32 = 4294901760;
pub const _IO_USER_BUF: u32 = 1;
pub const _IO_UNBUFFERED: u32 = 2;
pub const _IO_NO_READS: u32 = 4;
pub const _IO_NO_WRITES: u32 = 8;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_DELETE_DONT_CLOSE: u32 = 64;
pub const _IO_LINKED: u32 = 128;
pub const _IO_IN_BACKUP: u32 = 256;
pub const _IO_LINE_BUF: u32 = 512;
pub const _IO_TIED_PUT_GET: u32 = 1024;
pub const _IO_CURRENTLY_PUTTING: u32 = 2048;
pub const _IO_IS_APPENDING: u32 = 4096;
pub const _IO_IS_FILEBUF: u32 = 8192;
pub const _IO_BAD_SEEN: u32 = 16384;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IO_FLAGS2_MMAP: u32 = 1;
pub const _IO_FLAGS2_NOTCANCEL: u32 = 2;
pub const _IO_FLAGS2_USER_WBUF: u32 = 8;
pub const _IO_SKIPWS: u32 = 1;
pub const _IO_LEFT: u32 = 2;
pub const _IO_RIGHT: u32 = 4;
pub const _IO_INTERNAL: u32 = 8;
pub const _IO_DEC: u32 = 16;
pub const _IO_OCT: u32 = 32;
pub const _IO_HEX: u32 = 64;
pub const _IO_SHOWBASE: u32 = 128;
pub const _IO_SHOWPOINT: u32 = 256;
pub const _IO_UPPERCASE: u32 = 512;
pub const _IO_SHOWPOS: u32 = 1024;
pub const _IO_SCIENTIFIC: u32 = 2048;
pub const _IO_FIXED: u32 = 4096;
pub const _IO_UNITBUF: u32 = 8192;
pub const _IO_STDIO: u32 = 16384;
pub const _IO_DONT_CLOSE: u32 = 32768;
pub const _IO_BOOLALPHA: u32 = 65536;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const BSON_WORD_SIZE: u32 = 64;
pub const _UNISTD_H: u32 = 1;
pub const _POSIX_VERSION: u32 = 200809;
pub const __POSIX2_THIS_VERSION: u32 = 200809;
pub const _POSIX2_VERSION: u32 = 200809;
pub const _POSIX2_C_VERSION: u32 = 200809;
pub const _POSIX2_C_BIND: u32 = 200809;
pub const _POSIX2_C_DEV: u32 = 200809;
pub const _POSIX2_SW_DEV: u32 = 200809;
pub const _POSIX2_LOCALEDEF: u32 = 200809;
pub const _XOPEN_VERSION: u32 = 700;
pub const _XOPEN_XCU_VERSION: u32 = 4;
pub const _XOPEN_XPG2: u32 = 1;
pub const _XOPEN_XPG3: u32 = 1;
pub const _XOPEN_XPG4: u32 = 1;
pub const _XOPEN_UNIX: u32 = 1;
pub const _XOPEN_CRYPT: u32 = 1;
pub const _XOPEN_ENH_I18N: u32 = 1;
pub const _XOPEN_LEGACY: u32 = 1;
pub const _BITS_POSIX_OPT_H: u32 = 1;
pub const _POSIX_JOB_CONTROL: u32 = 1;
pub const _POSIX_SAVED_IDS: u32 = 1;
pub const _POSIX_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_SYNCHRONIZED_IO: u32 = 200809;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_CHOWN_RESTRICTED: u32 = 0;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_NO_TRUNC: u32 = 1;
pub const _XOPEN_REALTIME: u32 = 1;
pub const _XOPEN_REALTIME_THREADS: u32 = 1;
pub const _XOPEN_SHM: u32 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u32 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: i32 = -1;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: u32 = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 200809;
pub const _POSIX_ASYNC_IO: u32 = 1;
pub const _LFS_ASYNCHRONOUS_IO: u32 = 1;
pub const _POSIX_PRIORITIZED_IO: u32 = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: u32 = 1;
pub const _LFS_LARGEFILE: u32 = 1;
pub const _LFS64_LARGEFILE: u32 = 1;
pub const _LFS64_STDIO: u32 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u32 = 0;
pub const _POSIX_THREAD_CPUTIME: u32 = 0;
pub const _POSIX_REGEXP: u32 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u32 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 200809;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: u32 = 200809;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 0;
pub const _POSIX_CLOCK_SELECTION: u32 = 200809;
pub const _POSIX_ADVISORY_INFO: u32 = 200809;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_TRACE: i32 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i32 = -1;
pub const _POSIX_TRACE_INHERIT: i32 = -1;
pub const _POSIX_TRACE_LOG: i32 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i32 = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: i32 = -1;
pub const _XBS5_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V7_LP64_OFF64: u32 = 1;
pub const _POSIX_V6_LP64_OFF64: u32 = 1;
pub const _XBS5_LP64_OFF64: u32 = 1;
pub const __ILP32_OFF32_CFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFF32_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFFBIG_CFLAGS: &'static [u8; 48usize] =
    b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0";
pub const __ILP32_OFFBIG_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __LP64_OFF64_CFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const __LP64_OFF64_LDFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;
pub const R_OK: u32 = 4;
pub const W_OK: u32 = 2;
pub const X_OK: u32 = 1;
pub const F_OK: u32 = 0;
pub const L_SET: u32 = 0;
pub const L_INCR: u32 = 1;
pub const L_XTND: u32 = 2;
pub const _GETOPT_POSIX_H: u32 = 1;
pub const _GETOPT_CORE_H: u32 = 1;
pub const F_ULOCK: u32 = 0;
pub const F_LOCK: u32 = 1;
pub const F_TLOCK: u32 = 2;
pub const F_TEST: u32 = 3;
pub const _SYS_TIME_H: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timespec_defined: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _ERRNO_H: u32 = 1;
pub const _BITS_ERRNO_H: u32 = 1;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11;
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;
pub const EDEADLOCK: u32 = 35;
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EMULTIHOP: u32 = 72;
pub const EDOTDOT: u32 = 73;
pub const EBADMSG: u32 = 74;
pub const EOVERFLOW: u32 = 75;
pub const ENOTUNIQ: u32 = 76;
pub const EBADFD: u32 = 77;
pub const EREMCHG: u32 = 78;
pub const ELIBACC: u32 = 79;
pub const ELIBBAD: u32 = 80;
pub const ELIBSCN: u32 = 81;
pub const ELIBMAX: u32 = 82;
pub const ELIBEXEC: u32 = 83;
pub const EILSEQ: u32 = 84;
pub const ERESTART: u32 = 85;
pub const ESTRPIPE: u32 = 86;
pub const EUSERS: u32 = 87;
pub const ENOTSOCK: u32 = 88;
pub const EDESTADDRREQ: u32 = 89;
pub const EMSGSIZE: u32 = 90;
pub const EPROTOTYPE: u32 = 91;
pub const ENOPROTOOPT: u32 = 92;
pub const EPROTONOSUPPORT: u32 = 93;
pub const ESOCKTNOSUPPORT: u32 = 94;
pub const EOPNOTSUPP: u32 = 95;
pub const EPFNOSUPPORT: u32 = 96;
pub const EAFNOSUPPORT: u32 = 97;
pub const EADDRINUSE: u32 = 98;
pub const EADDRNOTAVAIL: u32 = 99;
pub const ENETDOWN: u32 = 100;
pub const ENETUNREACH: u32 = 101;
pub const ENETRESET: u32 = 102;
pub const ECONNABORTED: u32 = 103;
pub const ECONNRESET: u32 = 104;
pub const ENOBUFS: u32 = 105;
pub const EISCONN: u32 = 106;
pub const ENOTCONN: u32 = 107;
pub const ESHUTDOWN: u32 = 108;
pub const ETOOMANYREFS: u32 = 109;
pub const ETIMEDOUT: u32 = 110;
pub const ECONNREFUSED: u32 = 111;
pub const EHOSTDOWN: u32 = 112;
pub const EHOSTUNREACH: u32 = 113;
pub const EALREADY: u32 = 114;
pub const EINPROGRESS: u32 = 115;
pub const ESTALE: u32 = 116;
pub const EUCLEAN: u32 = 117;
pub const ENOTNAM: u32 = 118;
pub const ENAVAIL: u32 = 119;
pub const EISNAM: u32 = 120;
pub const EREMOTEIO: u32 = 121;
pub const EDQUOT: u32 = 122;
pub const ENOMEDIUM: u32 = 123;
pub const EMEDIUMTYPE: u32 = 124;
pub const ECANCELED: u32 = 125;
pub const ENOKEY: u32 = 126;
pub const EKEYEXPIRED: u32 = 127;
pub const EKEYREVOKED: u32 = 128;
pub const EKEYREJECTED: u32 = 129;
pub const EOWNERDEAD: u32 = 130;
pub const ENOTRECOVERABLE: u32 = 131;
pub const ERFKILL: u32 = 132;
pub const EHWPOISON: u32 = 133;
pub const ENOTSUP: u32 = 95;
pub const _CTYPE_H: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _LIBC_LIMITS_H_: u32 = 1;
pub const MB_LEN_MAX: u32 = 16;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const _FCNTL_H: u32 = 1;
pub const __O_LARGEFILE: u32 = 0;
pub const F_GETLK64: u32 = 5;
pub const F_SETLK64: u32 = 6;
pub const F_SETLKW64: u32 = 7;
pub const O_ACCMODE: u32 = 3;
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 64;
pub const O_EXCL: u32 = 128;
pub const O_NOCTTY: u32 = 256;
pub const O_TRUNC: u32 = 512;
pub const O_APPEND: u32 = 1024;
pub const O_NONBLOCK: u32 = 2048;
pub const O_NDELAY: u32 = 2048;
pub const O_SYNC: u32 = 1052672;
pub const O_FSYNC: u32 = 1052672;
pub const O_ASYNC: u32 = 8192;
pub const __O_DIRECTORY: u32 = 65536;
pub const __O_NOFOLLOW: u32 = 131072;
pub const __O_CLOEXEC: u32 = 524288;
pub const __O_DIRECT: u32 = 16384;
pub const __O_NOATIME: u32 = 262144;
pub const __O_PATH: u32 = 2097152;
pub const __O_DSYNC: u32 = 4096;
pub const __O_TMPFILE: u32 = 4259840;
pub const F_GETLK: u32 = 5;
pub const F_SETLK: u32 = 6;
pub const F_SETLKW: u32 = 7;
pub const O_DIRECTORY: u32 = 65536;
pub const O_NOFOLLOW: u32 = 131072;
pub const O_CLOEXEC: u32 = 524288;
pub const O_DSYNC: u32 = 4096;
pub const O_RSYNC: u32 = 1052672;
pub const F_DUPFD: u32 = 0;
pub const F_GETFD: u32 = 1;
pub const F_SETFD: u32 = 2;
pub const F_GETFL: u32 = 3;
pub const F_SETFL: u32 = 4;
pub const __F_SETOWN: u32 = 8;
pub const __F_GETOWN: u32 = 9;
pub const F_SETOWN: u32 = 8;
pub const F_GETOWN: u32 = 9;
pub const __F_SETSIG: u32 = 10;
pub const __F_GETSIG: u32 = 11;
pub const __F_SETOWN_EX: u32 = 15;
pub const __F_GETOWN_EX: u32 = 16;
pub const F_DUPFD_CLOEXEC: u32 = 1030;
pub const FD_CLOEXEC: u32 = 1;
pub const F_RDLCK: u32 = 0;
pub const F_WRLCK: u32 = 1;
pub const F_UNLCK: u32 = 2;
pub const F_EXLCK: u32 = 4;
pub const F_SHLCK: u32 = 8;
pub const LOCK_SH: u32 = 1;
pub const LOCK_EX: u32 = 2;
pub const LOCK_NB: u32 = 4;
pub const LOCK_UN: u32 = 8;
pub const FAPPEND: u32 = 1024;
pub const FFSYNC: u32 = 1052672;
pub const FASYNC: u32 = 8192;
pub const FNONBLOCK: u32 = 2048;
pub const FNDELAY: u32 = 2048;
pub const __POSIX_FADV_DONTNEED: u32 = 4;
pub const __POSIX_FADV_NOREUSE: u32 = 5;
pub const POSIX_FADV_NORMAL: u32 = 0;
pub const POSIX_FADV_RANDOM: u32 = 1;
pub const POSIX_FADV_SEQUENTIAL: u32 = 2;
pub const POSIX_FADV_WILLNEED: u32 = 3;
pub const POSIX_FADV_DONTNEED: u32 = 4;
pub const POSIX_FADV_NOREUSE: u32 = 5;
pub const AT_FDCWD: i32 = -100;
pub const AT_SYMLINK_NOFOLLOW: u32 = 256;
pub const AT_REMOVEDIR: u32 = 512;
pub const AT_SYMLINK_FOLLOW: u32 = 1024;
pub const AT_EACCESS: u32 = 512;
pub const _BITS_STAT_H: u32 = 1;
pub const _STAT_VER_KERNEL: u32 = 0;
pub const _STAT_VER_LINUX: u32 = 1;
pub const _MKNOD_VER_LINUX: u32 = 0;
pub const _STAT_VER: u32 = 1;
pub const __S_IFMT: u32 = 61440;
pub const __S_IFDIR: u32 = 16384;
pub const __S_IFCHR: u32 = 8192;
pub const __S_IFBLK: u32 = 24576;
pub const __S_IFREG: u32 = 32768;
pub const __S_IFIFO: u32 = 4096;
pub const __S_IFLNK: u32 = 40960;
pub const __S_IFSOCK: u32 = 49152;
pub const __S_ISUID: u32 = 2048;
pub const __S_ISGID: u32 = 1024;
pub const __S_ISVTX: u32 = 512;
pub const __S_IREAD: u32 = 256;
pub const __S_IWRITE: u32 = 128;
pub const __S_IEXEC: u32 = 64;
pub const UTIME_NOW: u32 = 1073741823;
pub const UTIME_OMIT: u32 = 1073741822;
pub const S_IFMT: u32 = 61440;
pub const S_IFDIR: u32 = 16384;
pub const S_IFCHR: u32 = 8192;
pub const S_IFBLK: u32 = 24576;
pub const S_IFREG: u32 = 32768;
pub const S_IFIFO: u32 = 4096;
pub const S_IFLNK: u32 = 40960;
pub const S_IFSOCK: u32 = 49152;
pub const S_ISUID: u32 = 2048;
pub const S_ISGID: u32 = 1024;
pub const S_ISVTX: u32 = 512;
pub const S_IRUSR: u32 = 256;
pub const S_IWUSR: u32 = 128;
pub const S_IXUSR: u32 = 64;
pub const S_IRWXU: u32 = 448;
pub const S_IRGRP: u32 = 32;
pub const S_IWGRP: u32 = 16;
pub const S_IXGRP: u32 = 8;
pub const S_IRWXG: u32 = 56;
pub const S_IROTH: u32 = 4;
pub const S_IWOTH: u32 = 2;
pub const S_IXOTH: u32 = 1;
pub const S_IRWXO: u32 = 7;
pub const _SYS_STAT_H: u32 = 1;
pub const S_IREAD: u32 = 256;
pub const S_IWRITE: u32 = 128;
pub const S_IEXEC: u32 = 64;
pub const ACCESSPERMS: u32 = 511;
pub const ALLPERMS: u32 = 4095;
pub const DEFFILEMODE: u32 = 438;
pub const S_BLKSIZE: u32 = 512;
pub const _MKNOD_VER: u32 = 0;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __ENUM_IDTYPE_T: u32 = 1;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _SYS_SYSMACROS_H: u32 = 1;
pub const _BITS_SYSMACROS_H: u32 = 1;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const _STRING_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const _TIME_H: u32 = 1;
pub const _BITS_TIME_H: u32 = 1;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_TAI: u32 = 11;
pub const TIMER_ABSTIME: u32 = 1;
pub const __struct_tm_defined: u32 = 1;
pub const __itimerspec_defined: u32 = 1;
pub const TIME_UTC: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const _INTTYPES_H: u32 = 1;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 3usize] = b"li\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 3usize] = b"li\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const BSON_BIG_ENDIAN: u32 = 4321;
pub const BSON_LITTLE_ENDIAN: u32 = 1234;
pub const BSON_ERROR_BUFFER_SIZE: u32 = 504;
pub const BSON_DECIMAL128_STRING: u32 = 43;
pub const BSON_DECIMAL128_INF: &'static [u8; 9usize] = b"Infinity\0";
pub const BSON_DECIMAL128_NAN: &'static [u8; 4usize] = b"NaN\0";
pub const BSON_ERROR_JSON: u32 = 1;
pub const BSON_ERROR_READER: u32 = 2;
pub const BSON_ERROR_INVALID: u32 = 3;
pub const BSON_ERROR_READER_BADFD: u32 = 1;
pub const BSON_MAJOR_VERSION: u32 = 1;
pub const BSON_MINOR_VERSION: u32 = 13;
pub const BSON_MICRO_VERSION: u32 = 0;
pub const BSON_VERSION_S: &'static [u8; 7usize] = b"1.13.0\0";
pub const BSON_VERSION_HEX: u32 = 17629184;
pub const BCON_STACK_MAX: u32 = 100;
pub const BSON_HOST_NAME_MAX: u32 = 255;
pub const MONGOC_USER_SET_CFLAGS: &'static [u8; 1usize] = b"\0";
pub const MONGOC_USER_SET_LDFLAGS: &'static [u8; 1usize] = b"\0";
pub const MONGOC_CC: &'static [u8; 12usize] = b"/usr/bin/cc\0";
pub const MONGOC_ENABLE_SSL_SECURE_CHANNEL: u32 = 0;
pub const MONGOC_ENABLE_CRYPTO_CNG: u32 = 0;
pub const MONGOC_ENABLE_SSL_SECURE_TRANSPORT: u32 = 0;
pub const MONGOC_ENABLE_CRYPTO_COMMON_CRYPTO: u32 = 0;
pub const MONGOC_ENABLE_SSL_LIBRESSL: u32 = 0;
pub const MONGOC_ENABLE_SSL_OPENSSL: u32 = 1;
pub const MONGOC_ENABLE_CRYPTO_LIBCRYPTO: u32 = 1;
pub const MONGOC_ENABLE_SSL: u32 = 1;
pub const MONGOC_ENABLE_CRYPTO: u32 = 1;
pub const MONGOC_ENABLE_CRYPTO_SYSTEM_PROFILE: u32 = 0;
pub const MONGOC_HAVE_ASN1_STRING_GET0_DATA: u32 = 0;
pub const MONGOC_ENABLE_SASL: u32 = 1;
pub const MONGOC_ENABLE_SASL_CYRUS: u32 = 1;
pub const MONGOC_ENABLE_SASL_SSPI: u32 = 0;
pub const MONGOC_ENABLE_SASL_GSSAPI: u32 = 0;
pub const MONGOC_HAVE_SASL_CLIENT_DONE: u32 = 0;
pub const MONGOC_NO_AUTOMATIC_GLOBALS: u32 = 1;
pub const MONGOC_HAVE_SOCKLEN: u32 = 1;
pub const MONGOC_HAVE_DNSAPI: u32 = 0;
pub const MONGOC_HAVE_RES_NSEARCH: u32 = 1;
pub const MONGOC_HAVE_RES_NDESTROY: u32 = 0;
pub const MONGOC_HAVE_RES_NCLOSE: u32 = 1;
pub const MONGOC_HAVE_RES_SEARCH: u32 = 0;
pub const MONGOC_ENABLE_COMPRESSION: u32 = 1;
pub const MONGOC_ENABLE_COMPRESSION_SNAPPY: u32 = 0;
pub const MONGOC_ENABLE_COMPRESSION_ZLIB: u32 = 1;
pub const MONGOC_ENABLE_SHM_COUNTERS: u32 = 1;
pub const MONGOC_ENABLE_RDTSCP: u32 = 0;
pub const MONGOC_HAVE_SCHED_GETCPU: u32 = 0;
pub const MONGOC_TRACE: u32 = 0;
pub const MONGOC_ENABLE_ICU: u32 = 0;
pub const MONGOC_NO_MAX_STALENESS: i32 = -1;
pub const MONGOC_SMALLEST_MAX_STALENESS_SECONDS: u32 = 90;
pub const MONGOC_WRITE_CONCERN_W_UNACKNOWLEDGED: u32 = 0;
pub const MONGOC_WRITE_CONCERN_W_ERRORS_IGNORED: i32 = -1;
pub const MONGOC_WRITE_CONCERN_W_DEFAULT: i32 = -2;
pub const MONGOC_WRITE_CONCERN_W_MAJORITY: i32 = -3;
pub const MONGOC_WRITE_CONCERN_W_TAG: i32 = -4;
pub const MONGOC_INSERT_NO_VALIDATE: u32 = 2147483648;
pub const MONGOC_UPDATE_NO_VALIDATE: u32 = 2147483648;
pub const MONGOC_READ_CONCERN_LEVEL_AVAILABLE: &'static [u8; 10usize] = b"available\0";
pub const MONGOC_READ_CONCERN_LEVEL_LOCAL: &'static [u8; 6usize] = b"local\0";
pub const MONGOC_READ_CONCERN_LEVEL_MAJORITY: &'static [u8; 9usize] = b"majority\0";
pub const MONGOC_READ_CONCERN_LEVEL_LINEARIZABLE: &'static [u8; 13usize] = b"linearizable\0";
pub const MONGOC_READ_CONCERN_LEVEL_SNAPSHOT: &'static [u8; 9usize] = b"snapshot\0";
pub const _SYS_UIO_H: u32 = 1;
pub const __iovec_defined: u32 = 1;
pub const _BITS_UIO_LIM_H: u32 = 1;
pub const __IOV_MAX: u32 = 1024;
pub const UIO_MAXIOV: u32 = 1024;
pub const _ARPA_INET_H: u32 = 1;
pub const _NETINET_IN_H: u32 = 1;
pub const _SYS_SOCKET_H: u32 = 1;
pub const PF_UNSPEC: u32 = 0;
pub const PF_LOCAL: u32 = 1;
pub const PF_UNIX: u32 = 1;
pub const PF_FILE: u32 = 1;
pub const PF_INET: u32 = 2;
pub const PF_AX25: u32 = 3;
pub const PF_IPX: u32 = 4;
pub const PF_APPLETALK: u32 = 5;
pub const PF_NETROM: u32 = 6;
pub const PF_BRIDGE: u32 = 7;
pub const PF_ATMPVC: u32 = 8;
pub const PF_X25: u32 = 9;
pub const PF_INET6: u32 = 10;
pub const PF_ROSE: u32 = 11;
pub const PF_DECnet: u32 = 12;
pub const PF_NETBEUI: u32 = 13;
pub const PF_SECURITY: u32 = 14;
pub const PF_KEY: u32 = 15;
pub const PF_NETLINK: u32 = 16;
pub const PF_ROUTE: u32 = 16;
pub const PF_PACKET: u32 = 17;
pub const PF_ASH: u32 = 18;
pub const PF_ECONET: u32 = 19;
pub const PF_ATMSVC: u32 = 20;
pub const PF_RDS: u32 = 21;
pub const PF_SNA: u32 = 22;
pub const PF_IRDA: u32 = 23;
pub const PF_PPPOX: u32 = 24;
pub const PF_WANPIPE: u32 = 25;
pub const PF_LLC: u32 = 26;
pub const PF_IB: u32 = 27;
pub const PF_MPLS: u32 = 28;
pub const PF_CAN: u32 = 29;
pub const PF_TIPC: u32 = 30;
pub const PF_BLUETOOTH: u32 = 31;
pub const PF_IUCV: u32 = 32;
pub const PF_RXRPC: u32 = 33;
pub const PF_ISDN: u32 = 34;
pub const PF_PHONET: u32 = 35;
pub const PF_IEEE802154: u32 = 36;
pub const PF_CAIF: u32 = 37;
pub const PF_ALG: u32 = 38;
pub const PF_NFC: u32 = 39;
pub const PF_VSOCK: u32 = 40;
pub const PF_KCM: u32 = 41;
pub const PF_QIPCRTR: u32 = 42;
pub const PF_SMC: u32 = 43;
pub const PF_MAX: u32 = 44;
pub const AF_UNSPEC: u32 = 0;
pub const AF_LOCAL: u32 = 1;
pub const AF_UNIX: u32 = 1;
pub const AF_FILE: u32 = 1;
pub const AF_INET: u32 = 2;
pub const AF_AX25: u32 = 3;
pub const AF_IPX: u32 = 4;
pub const AF_APPLETALK: u32 = 5;
pub const AF_NETROM: u32 = 6;
pub const AF_BRIDGE: u32 = 7;
pub const AF_ATMPVC: u32 = 8;
pub const AF_X25: u32 = 9;
pub const AF_INET6: u32 = 10;
pub const AF_ROSE: u32 = 11;
pub const AF_DECnet: u32 = 12;
pub const AF_NETBEUI: u32 = 13;
pub const AF_SECURITY: u32 = 14;
pub const AF_KEY: u32 = 15;
pub const AF_NETLINK: u32 = 16;
pub const AF_ROUTE: u32 = 16;
pub const AF_PACKET: u32 = 17;
pub const AF_ASH: u32 = 18;
pub const AF_ECONET: u32 = 19;
pub const AF_ATMSVC: u32 = 20;
pub const AF_RDS: u32 = 21;
pub const AF_SNA: u32 = 22;
pub const AF_IRDA: u32 = 23;
pub const AF_PPPOX: u32 = 24;
pub const AF_WANPIPE: u32 = 25;
pub const AF_LLC: u32 = 26;
pub const AF_IB: u32 = 27;
pub const AF_MPLS: u32 = 28;
pub const AF_CAN: u32 = 29;
pub const AF_TIPC: u32 = 30;
pub const AF_BLUETOOTH: u32 = 31;
pub const AF_IUCV: u32 = 32;
pub const AF_RXRPC: u32 = 33;
pub const AF_ISDN: u32 = 34;
pub const AF_PHONET: u32 = 35;
pub const AF_IEEE802154: u32 = 36;
pub const AF_CAIF: u32 = 37;
pub const AF_ALG: u32 = 38;
pub const AF_NFC: u32 = 39;
pub const AF_VSOCK: u32 = 40;
pub const AF_KCM: u32 = 41;
pub const AF_QIPCRTR: u32 = 42;
pub const AF_SMC: u32 = 43;
pub const AF_MAX: u32 = 44;
pub const SOL_RAW: u32 = 255;
pub const SOL_DECNET: u32 = 261;
pub const SOL_X25: u32 = 262;
pub const SOL_PACKET: u32 = 263;
pub const SOL_ATM: u32 = 264;
pub const SOL_AAL: u32 = 265;
pub const SOL_IRDA: u32 = 266;
pub const SOL_NETBEUI: u32 = 267;
pub const SOL_LLC: u32 = 268;
pub const SOL_DCCP: u32 = 269;
pub const SOL_NETLINK: u32 = 270;
pub const SOL_TIPC: u32 = 271;
pub const SOL_RXRPC: u32 = 272;
pub const SOL_PPPOL2TP: u32 = 273;
pub const SOL_BLUETOOTH: u32 = 274;
pub const SOL_PNPIPE: u32 = 275;
pub const SOL_RDS: u32 = 276;
pub const SOL_IUCV: u32 = 277;
pub const SOL_CAIF: u32 = 278;
pub const SOL_ALG: u32 = 279;
pub const SOL_NFC: u32 = 280;
pub const SOL_KCM: u32 = 281;
pub const SOL_TLS: u32 = 282;
pub const SOMAXCONN: u32 = 128;
pub const _BITS_SOCKADDR_H: u32 = 1;
pub const _SS_SIZE: u32 = 128;
pub const FIOSETOWN: u32 = 35073;
pub const SIOCSPGRP: u32 = 35074;
pub const FIOGETOWN: u32 = 35075;
pub const SIOCGPGRP: u32 = 35076;
pub const SIOCATMARK: u32 = 35077;
pub const SIOCGSTAMP: u32 = 35078;
pub const SIOCGSTAMPNS: u32 = 35079;
pub const SOL_SOCKET: u32 = 1;
pub const SO_DEBUG: u32 = 1;
pub const SO_REUSEADDR: u32 = 2;
pub const SO_TYPE: u32 = 3;
pub const SO_ERROR: u32 = 4;
pub const SO_DONTROUTE: u32 = 5;
pub const SO_BROADCAST: u32 = 6;
pub const SO_SNDBUF: u32 = 7;
pub const SO_RCVBUF: u32 = 8;
pub const SO_SNDBUFFORCE: u32 = 32;
pub const SO_RCVBUFFORCE: u32 = 33;
pub const SO_KEEPALIVE: u32 = 9;
pub const SO_OOBINLINE: u32 = 10;
pub const SO_NO_CHECK: u32 = 11;
pub const SO_PRIORITY: u32 = 12;
pub const SO_LINGER: u32 = 13;
pub const SO_BSDCOMPAT: u32 = 14;
pub const SO_REUSEPORT: u32 = 15;
pub const SO_PASSCRED: u32 = 16;
pub const SO_PEERCRED: u32 = 17;
pub const SO_RCVLOWAT: u32 = 18;
pub const SO_SNDLOWAT: u32 = 19;
pub const SO_RCVTIMEO: u32 = 20;
pub const SO_SNDTIMEO: u32 = 21;
pub const SO_SECURITY_AUTHENTICATION: u32 = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: u32 = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: u32 = 24;
pub const SO_BINDTODEVICE: u32 = 25;
pub const SO_ATTACH_FILTER: u32 = 26;
pub const SO_DETACH_FILTER: u32 = 27;
pub const SO_GET_FILTER: u32 = 26;
pub const SO_PEERNAME: u32 = 28;
pub const SO_TIMESTAMP: u32 = 29;
pub const SCM_TIMESTAMP: u32 = 29;
pub const SO_ACCEPTCONN: u32 = 30;
pub const SO_PEERSEC: u32 = 31;
pub const SO_PASSSEC: u32 = 34;
pub const SO_TIMESTAMPNS: u32 = 35;
pub const SCM_TIMESTAMPNS: u32 = 35;
pub const SO_MARK: u32 = 36;
pub const SO_TIMESTAMPING: u32 = 37;
pub const SCM_TIMESTAMPING: u32 = 37;
pub const SO_PROTOCOL: u32 = 38;
pub const SO_DOMAIN: u32 = 39;
pub const SO_RXQ_OVFL: u32 = 40;
pub const SO_WIFI_STATUS: u32 = 41;
pub const SCM_WIFI_STATUS: u32 = 41;
pub const SO_PEEK_OFF: u32 = 42;
pub const SO_NOFCS: u32 = 43;
pub const SO_LOCK_FILTER: u32 = 44;
pub const SO_SELECT_ERR_QUEUE: u32 = 45;
pub const SO_BUSY_POLL: u32 = 46;
pub const SO_MAX_PACING_RATE: u32 = 47;
pub const SO_BPF_EXTENSIONS: u32 = 48;
pub const SO_INCOMING_CPU: u32 = 49;
pub const SO_ATTACH_BPF: u32 = 50;
pub const SO_DETACH_BPF: u32 = 27;
pub const SO_ATTACH_REUSEPORT_CBPF: u32 = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: u32 = 52;
pub const SO_CNX_ADVICE: u32 = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: u32 = 54;
pub const SO_MEMINFO: u32 = 55;
pub const SO_INCOMING_NAPI_ID: u32 = 56;
pub const SO_COOKIE: u32 = 57;
pub const SCM_TIMESTAMPING_PKTINFO: u32 = 58;
pub const SO_PEERGROUPS: u32 = 59;
pub const SO_ZEROCOPY: u32 = 60;
pub const __osockaddr_defined: u32 = 1;
pub const __USE_KERNEL_IPV6_DEFS: u32 = 0;
pub const IP_OPTIONS: u32 = 4;
pub const IP_HDRINCL: u32 = 3;
pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;
pub const IP_RECVOPTS: u32 = 6;
pub const IP_RETOPTS: u32 = 7;
pub const IP_MULTICAST_IF: u32 = 32;
pub const IP_MULTICAST_TTL: u32 = 33;
pub const IP_MULTICAST_LOOP: u32 = 34;
pub const IP_ADD_MEMBERSHIP: u32 = 35;
pub const IP_DROP_MEMBERSHIP: u32 = 36;
pub const IP_UNBLOCK_SOURCE: u32 = 37;
pub const IP_BLOCK_SOURCE: u32 = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 40;
pub const IP_MSFILTER: u32 = 41;
pub const MCAST_JOIN_GROUP: u32 = 42;
pub const MCAST_BLOCK_SOURCE: u32 = 43;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44;
pub const MCAST_LEAVE_GROUP: u32 = 45;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 47;
pub const MCAST_MSFILTER: u32 = 48;
pub const IP_MULTICAST_ALL: u32 = 49;
pub const IP_UNICAST_IF: u32 = 50;
pub const MCAST_EXCLUDE: u32 = 0;
pub const MCAST_INCLUDE: u32 = 1;
pub const IP_ROUTER_ALERT: u32 = 5;
pub const IP_PKTINFO: u32 = 8;
pub const IP_PKTOPTIONS: u32 = 9;
pub const IP_PMTUDISC: u32 = 10;
pub const IP_MTU_DISCOVER: u32 = 10;
pub const IP_RECVERR: u32 = 11;
pub const IP_RECVTTL: u32 = 12;
pub const IP_RECVTOS: u32 = 13;
pub const IP_MTU: u32 = 14;
pub const IP_FREEBIND: u32 = 15;
pub const IP_IPSEC_POLICY: u32 = 16;
pub const IP_XFRM_POLICY: u32 = 17;
pub const IP_PASSSEC: u32 = 18;
pub const IP_TRANSPARENT: u32 = 19;
pub const IP_ORIGDSTADDR: u32 = 20;
pub const IP_RECVORIGDSTADDR: u32 = 20;
pub const IP_MINTTL: u32 = 21;
pub const IP_NODEFRAG: u32 = 22;
pub const IP_CHECKSUM: u32 = 23;
pub const IP_BIND_ADDRESS_NO_PORT: u32 = 24;
pub const IP_RECVFRAGSIZE: u32 = 25;
pub const IP_PMTUDISC_DONT: u32 = 0;
pub const IP_PMTUDISC_WANT: u32 = 1;
pub const IP_PMTUDISC_DO: u32 = 2;
pub const IP_PMTUDISC_PROBE: u32 = 3;
pub const IP_PMTUDISC_INTERFACE: u32 = 4;
pub const IP_PMTUDISC_OMIT: u32 = 5;
pub const SOL_IP: u32 = 0;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1;
pub const IP_MAX_MEMBERSHIPS: u32 = 20;
pub const IPV6_ADDRFORM: u32 = 1;
pub const IPV6_2292PKTINFO: u32 = 2;
pub const IPV6_2292HOPOPTS: u32 = 3;
pub const IPV6_2292DSTOPTS: u32 = 4;
pub const IPV6_2292RTHDR: u32 = 5;
pub const IPV6_2292PKTOPTIONS: u32 = 6;
pub const IPV6_CHECKSUM: u32 = 7;
pub const IPV6_2292HOPLIMIT: u32 = 8;
pub const IPV6_NEXTHOP: u32 = 9;
pub const IPV6_AUTHHDR: u32 = 10;
pub const IPV6_UNICAST_HOPS: u32 = 16;
pub const IPV6_MULTICAST_IF: u32 = 17;
pub const IPV6_MULTICAST_HOPS: u32 = 18;
pub const IPV6_MULTICAST_LOOP: u32 = 19;
pub const IPV6_JOIN_GROUP: u32 = 20;
pub const IPV6_LEAVE_GROUP: u32 = 21;
pub const IPV6_ROUTER_ALERT: u32 = 22;
pub const IPV6_MTU_DISCOVER: u32 = 23;
pub const IPV6_MTU: u32 = 24;
pub const IPV6_RECVERR: u32 = 25;
pub const IPV6_V6ONLY: u32 = 26;
pub const IPV6_JOIN_ANYCAST: u32 = 27;
pub const IPV6_LEAVE_ANYCAST: u32 = 28;
pub const IPV6_IPSEC_POLICY: u32 = 34;
pub const IPV6_XFRM_POLICY: u32 = 35;
pub const IPV6_HDRINCL: u32 = 36;
pub const IPV6_RECVPKTINFO: u32 = 49;
pub const IPV6_PKTINFO: u32 = 50;
pub const IPV6_RECVHOPLIMIT: u32 = 51;
pub const IPV6_HOPLIMIT: u32 = 52;
pub const IPV6_RECVHOPOPTS: u32 = 53;
pub const IPV6_HOPOPTS: u32 = 54;
pub const IPV6_RTHDRDSTOPTS: u32 = 55;
pub const IPV6_RECVRTHDR: u32 = 56;
pub const IPV6_RTHDR: u32 = 57;
pub const IPV6_RECVDSTOPTS: u32 = 58;
pub const IPV6_DSTOPTS: u32 = 59;
pub const IPV6_RECVPATHMTU: u32 = 60;
pub const IPV6_PATHMTU: u32 = 61;
pub const IPV6_DONTFRAG: u32 = 62;
pub const IPV6_RECVTCLASS: u32 = 66;
pub const IPV6_TCLASS: u32 = 67;
pub const IPV6_AUTOFLOWLABEL: u32 = 70;
pub const IPV6_ADDR_PREFERENCES: u32 = 72;
pub const IPV6_MINHOPCOUNT: u32 = 73;
pub const IPV6_ORIGDSTADDR: u32 = 74;
pub const IPV6_RECVORIGDSTADDR: u32 = 74;
pub const IPV6_TRANSPARENT: u32 = 75;
pub const IPV6_UNICAST_IF: u32 = 76;
pub const IPV6_RECVFRAGSIZE: u32 = 77;
pub const IPV6_ADD_MEMBERSHIP: u32 = 20;
pub const IPV6_DROP_MEMBERSHIP: u32 = 21;
pub const IPV6_RXHOPOPTS: u32 = 54;
pub const IPV6_RXDSTOPTS: u32 = 59;
pub const IPV6_PMTUDISC_DONT: u32 = 0;
pub const IPV6_PMTUDISC_WANT: u32 = 1;
pub const IPV6_PMTUDISC_DO: u32 = 2;
pub const IPV6_PMTUDISC_PROBE: u32 = 3;
pub const IPV6_PMTUDISC_INTERFACE: u32 = 4;
pub const IPV6_PMTUDISC_OMIT: u32 = 5;
pub const SOL_IPV6: u32 = 41;
pub const SOL_ICMPV6: u32 = 58;
pub const IPV6_RTHDR_LOOSE: u32 = 0;
pub const IPV6_RTHDR_STRICT: u32 = 1;
pub const IPV6_RTHDR_TYPE_0: u32 = 0;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSA_HOST: u32 = 16777215;
pub const IN_CLASSA_MAX: u32 = 128;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSB_HOST: u32 = 65535;
pub const IN_CLASSB_MAX: u32 = 65536;
pub const IN_CLASSC_NET: u32 = 4294967040;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSC_HOST: u32 = 255;
pub const IN_LOOPBACKNET: u32 = 127;
pub const INET_ADDRSTRLEN: u32 = 16;
pub const INET6_ADDRSTRLEN: u32 = 46;
pub const _SYS_POLL_H: u32 = 1;
pub const POLLIN: u32 = 1;
pub const POLLPRI: u32 = 2;
pub const POLLOUT: u32 = 4;
pub const POLLRDNORM: u32 = 64;
pub const POLLRDBAND: u32 = 128;
pub const POLLWRNORM: u32 = 256;
pub const POLLWRBAND: u32 = 512;
pub const POLLERR: u32 = 8;
pub const POLLHUP: u32 = 16;
pub const POLLNVAL: u32 = 32;
pub const _NETDB_H: u32 = 1;
pub const _RPC_NETDB_H: u32 = 1;
pub const _PATH_HEQUIV: &'static [u8; 17usize] = b"/etc/hosts.equiv\0";
pub const _PATH_HOSTS: &'static [u8; 11usize] = b"/etc/hosts\0";
pub const _PATH_NETWORKS: &'static [u8; 14usize] = b"/etc/networks\0";
pub const _PATH_NSSWITCH_CONF: &'static [u8; 19usize] = b"/etc/nsswitch.conf\0";
pub const _PATH_PROTOCOLS: &'static [u8; 15usize] = b"/etc/protocols\0";
pub const _PATH_SERVICES: &'static [u8; 14usize] = b"/etc/services\0";
pub const HOST_NOT_FOUND: u32 = 1;
pub const TRY_AGAIN: u32 = 2;
pub const NO_RECOVERY: u32 = 3;
pub const NO_DATA: u32 = 4;
pub const NETDB_INTERNAL: i32 = -1;
pub const NETDB_SUCCESS: u32 = 0;
pub const NO_ADDRESS: u32 = 4;
pub const AI_PASSIVE: u32 = 1;
pub const AI_CANONNAME: u32 = 2;
pub const AI_NUMERICHOST: u32 = 4;
pub const AI_V4MAPPED: u32 = 8;
pub const AI_ALL: u32 = 16;
pub const AI_ADDRCONFIG: u32 = 32;
pub const AI_NUMERICSERV: u32 = 1024;
pub const EAI_BADFLAGS: i32 = -1;
pub const EAI_NONAME: i32 = -2;
pub const EAI_AGAIN: i32 = -3;
pub const EAI_FAIL: i32 = -4;
pub const EAI_FAMILY: i32 = -6;
pub const EAI_SOCKTYPE: i32 = -7;
pub const EAI_SERVICE: i32 = -8;
pub const EAI_MEMORY: i32 = -10;
pub const EAI_SYSTEM: i32 = -11;
pub const EAI_OVERFLOW: i32 = -12;
pub const NI_MAXHOST: u32 = 1025;
pub const NI_MAXSERV: u32 = 32;
pub const NI_NUMERICHOST: u32 = 1;
pub const NI_NUMERICSERV: u32 = 2;
pub const NI_NOFQDN: u32 = 4;
pub const NI_NAMEREQD: u32 = 8;
pub const NI_DGRAM: u32 = 16;
pub const _NETINET_TCP_H: u32 = 1;
pub const TCP_NODELAY: u32 = 1;
pub const TCP_MAXSEG: u32 = 2;
pub const TCP_CORK: u32 = 3;
pub const TCP_KEEPIDLE: u32 = 4;
pub const TCP_KEEPINTVL: u32 = 5;
pub const TCP_KEEPCNT: u32 = 6;
pub const TCP_SYNCNT: u32 = 7;
pub const TCP_LINGER2: u32 = 8;
pub const TCP_DEFER_ACCEPT: u32 = 9;
pub const TCP_WINDOW_CLAMP: u32 = 10;
pub const TCP_INFO: u32 = 11;
pub const TCP_QUICKACK: u32 = 12;
pub const TCP_CONGESTION: u32 = 13;
pub const TCP_MD5SIG: u32 = 14;
pub const TCP_COOKIE_TRANSACTIONS: u32 = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: u32 = 16;
pub const TCP_THIN_DUPACK: u32 = 17;
pub const TCP_USER_TIMEOUT: u32 = 18;
pub const TCP_REPAIR: u32 = 19;
pub const TCP_REPAIR_QUEUE: u32 = 20;
pub const TCP_QUEUE_SEQ: u32 = 21;
pub const TCP_REPAIR_OPTIONS: u32 = 22;
pub const TCP_FASTOPEN: u32 = 23;
pub const TCP_TIMESTAMP: u32 = 24;
pub const TCP_NOTSENT_LOWAT: u32 = 25;
pub const TCP_CC_INFO: u32 = 26;
pub const TCP_SAVE_SYN: u32 = 27;
pub const TCP_SAVED_SYN: u32 = 28;
pub const TCP_REPAIR_WINDOW: u32 = 29;
pub const TCP_FASTOPEN_CONNECT: u32 = 30;
pub const TCP_ULP: u32 = 31;
pub const TCP_MD5SIG_EXT: u32 = 32;
pub const TH_FIN: u32 = 1;
pub const TH_SYN: u32 = 2;
pub const TH_RST: u32 = 4;
pub const TH_PUSH: u32 = 8;
pub const TH_ACK: u32 = 16;
pub const TH_URG: u32 = 32;
pub const TCPOPT_EOL: u32 = 0;
pub const TCPOPT_NOP: u32 = 1;
pub const TCPOPT_MAXSEG: u32 = 2;
pub const TCPOLEN_MAXSEG: u32 = 4;
pub const TCPOPT_WINDOW: u32 = 3;
pub const TCPOLEN_WINDOW: u32 = 3;
pub const TCPOPT_SACK_PERMITTED: u32 = 4;
pub const TCPOLEN_SACK_PERMITTED: u32 = 2;
pub const TCPOPT_SACK: u32 = 5;
pub const TCPOPT_TIMESTAMP: u32 = 8;
pub const TCPOLEN_TIMESTAMP: u32 = 10;
pub const TCPOLEN_TSTAMP_APPA: u32 = 12;
pub const TCPOPT_TSTAMP_HDR: u32 = 16844810;
pub const TCP_MSS: u32 = 512;
pub const TCP_MAXWIN: u32 = 65535;
pub const TCP_MAX_WINSHIFT: u32 = 14;
pub const SOL_TCP: u32 = 6;
pub const TCPI_OPT_TIMESTAMPS: u32 = 1;
pub const TCPI_OPT_SACK: u32 = 2;
pub const TCPI_OPT_WSCALE: u32 = 4;
pub const TCPI_OPT_ECN: u32 = 8;
pub const TCPI_OPT_ECN_SEEN: u32 = 16;
pub const TCPI_OPT_SYN_DATA: u32 = 32;
pub const TCP_MD5SIG_MAXKEYLEN: u32 = 80;
pub const TCP_MD5SIG_FLAG_PREFIX: u32 = 1;
pub const TCP_COOKIE_MIN: u32 = 8;
pub const TCP_COOKIE_MAX: u32 = 16;
pub const TCP_COOKIE_PAIR_SIZE: u32 = 32;
pub const TCP_COOKIE_IN_ALWAYS: u32 = 1;
pub const TCP_COOKIE_OUT_NEVER: u32 = 2;
pub const TCP_S_DATA_IN: u32 = 4;
pub const TCP_S_DATA_OUT: u32 = 8;
pub const TCP_MSS_DEFAULT: u32 = 536;
pub const TCP_MSS_DESIRED: u32 = 1220;
pub const _SYS_UN_H: u32 = 1;
pub const MONGOC_DEFAULT_PORT: u32 = 27017;
pub const MONGOC_URI_APPNAME: &'static [u8; 8usize] = b"appname\0";
pub const MONGOC_URI_AUTHMECHANISM: &'static [u8; 14usize] = b"authmechanism\0";
pub const MONGOC_URI_AUTHMECHANISMPROPERTIES: &'static [u8; 24usize] = b"authmechanismproperties\0";
pub const MONGOC_URI_AUTHSOURCE: &'static [u8; 11usize] = b"authsource\0";
pub const MONGOC_URI_CANONICALIZEHOSTNAME: &'static [u8; 21usize] = b"canonicalizehostname\0";
pub const MONGOC_URI_CONNECTTIMEOUTMS: &'static [u8; 17usize] = b"connecttimeoutms\0";
pub const MONGOC_URI_COMPRESSORS: &'static [u8; 12usize] = b"compressors\0";
pub const MONGOC_URI_GSSAPISERVICENAME: &'static [u8; 18usize] = b"gssapiservicename\0";
pub const MONGOC_URI_HEARTBEATFREQUENCYMS: &'static [u8; 21usize] = b"heartbeatfrequencyms\0";
pub const MONGOC_URI_JOURNAL: &'static [u8; 8usize] = b"journal\0";
pub const MONGOC_URI_LOCALTHRESHOLDMS: &'static [u8; 17usize] = b"localthresholdms\0";
pub const MONGOC_URI_MAXIDLETIMEMS: &'static [u8; 14usize] = b"maxidletimems\0";
pub const MONGOC_URI_MAXPOOLSIZE: &'static [u8; 12usize] = b"maxpoolsize\0";
pub const MONGOC_URI_MAXSTALENESSSECONDS: &'static [u8; 20usize] = b"maxstalenessseconds\0";
pub const MONGOC_URI_MINPOOLSIZE: &'static [u8; 12usize] = b"minpoolsize\0";
pub const MONGOC_URI_READCONCERNLEVEL: &'static [u8; 17usize] = b"readconcernlevel\0";
pub const MONGOC_URI_READPREFERENCE: &'static [u8; 15usize] = b"readpreference\0";
pub const MONGOC_URI_READPREFERENCETAGS: &'static [u8; 19usize] = b"readpreferencetags\0";
pub const MONGOC_URI_REPLICASET: &'static [u8; 11usize] = b"replicaset\0";
pub const MONGOC_URI_RETRYWRITES: &'static [u8; 12usize] = b"retrywrites\0";
pub const MONGOC_URI_SAFE: &'static [u8; 5usize] = b"safe\0";
pub const MONGOC_URI_SERVERSELECTIONTIMEOUTMS: &'static [u8; 25usize] =
    b"serverselectiontimeoutms\0";
pub const MONGOC_URI_SERVERSELECTIONTRYONCE: &'static [u8; 23usize] = b"serverselectiontryonce\0";
pub const MONGOC_URI_SLAVEOK: &'static [u8; 8usize] = b"slaveok\0";
pub const MONGOC_URI_SOCKETCHECKINTERVALMS: &'static [u8; 22usize] = b"socketcheckintervalms\0";
pub const MONGOC_URI_SOCKETTIMEOUTMS: &'static [u8; 16usize] = b"sockettimeoutms\0";
pub const MONGOC_URI_SSL: &'static [u8; 4usize] = b"ssl\0";
pub const MONGOC_URI_SSLCLIENTCERTIFICATEKEYFILE: &'static [u8; 28usize] =
    b"sslclientcertificatekeyfile\0";
pub const MONGOC_URI_SSLCLIENTCERTIFICATEKEYPASSWORD: &'static [u8; 32usize] =
    b"sslclientcertificatekeypassword\0";
pub const MONGOC_URI_SSLCERTIFICATEAUTHORITYFILE: &'static [u8; 28usize] =
    b"sslcertificateauthorityfile\0";
pub const MONGOC_URI_SSLALLOWINVALIDCERTIFICATES: &'static [u8; 28usize] =
    b"sslallowinvalidcertificates\0";
pub const MONGOC_URI_SSLALLOWINVALIDHOSTNAMES: &'static [u8; 25usize] =
    b"sslallowinvalidhostnames\0";
pub const MONGOC_URI_W: &'static [u8; 2usize] = b"w\0";
pub const MONGOC_URI_WAITQUEUEMULTIPLE: &'static [u8; 18usize] = b"waitqueuemultiple\0";
pub const MONGOC_URI_WAITQUEUETIMEOUTMS: &'static [u8; 19usize] = b"waitqueuetimeoutms\0";
pub const MONGOC_URI_WTIMEOUTMS: &'static [u8; 11usize] = b"wtimeoutms\0";
pub const MONGOC_URI_ZLIBCOMPRESSIONLEVEL: &'static [u8; 21usize] = b"zlibcompressionlevel\0";
pub const MONGOC_NAMESPACE_MAX: u32 = 128;
pub const MONGOC_DEFAULT_CONNECTTIMEOUTMS: u32 = 10000;
pub const MONGOC_DEFAULT_SOCKETTIMEOUTMS: u32 = 300000;
pub const MONGOC_ERROR_API_VERSION_LEGACY: u32 = 1;
pub const MONGOC_ERROR_API_VERSION_2: u32 = 2;
pub const MONGOC_HANDSHAKE_APPNAME_MAX: u32 = 128;
pub const MONGOC_LOG_DOMAIN: &'static [u8; 7usize] = b"mongoc\0";
pub const MONGOC_MAJOR_VERSION: u32 = 1;
pub const MONGOC_MINOR_VERSION: u32 = 13;
pub const MONGOC_MICRO_VERSION: u32 = 0;
pub const MONGOC_VERSION_S: &'static [u8; 7usize] = b"1.13.0\0";
pub const MONGOC_VERSION_HEX: u32 = 17629184;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wch)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wchb as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wchb)
        )
    );
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        8usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__value as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__value)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos_t() {
    assert_eq!(
        ::std::mem::size_of::<_G_fpos_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos_t>())).__pos as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos_t>())).__state as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__state)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos64_t() {
    assert_eq!(
        ::std::mem::size_of::<_G_fpos64_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos64_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos64_t>())).__pos as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos64_t>())).__state as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__state)
        )
    );
}
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_jump_t {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__IO_marker() {
    assert_eq!(
        ::std::mem::size_of::<_IO_marker>(),
        24usize,
        concat!("Size of: ", stringify!(_IO_marker))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_marker>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_marker))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._sbuf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_sbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._pos as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_pos)
        )
    );
}
pub const __codecvt_result___codecvt_ok: __codecvt_result = 0;
pub const __codecvt_result___codecvt_partial: __codecvt_result = 1;
pub const __codecvt_result___codecvt_error: __codecvt_result = 2;
pub const __codecvt_result___codecvt_noconv: __codecvt_result = 3;
pub type __codecvt_result = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::std::os::raw::c_void,
    pub __pad2: *mut ::std::os::raw::c_void,
    pub __pad3: *mut ::std::os::raw::c_void,
    pub __pad4: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad1 as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad2 as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad3 as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad4 as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE_plus {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_IO_2_1_stdin_"]
    pub static mut _IO_2_1_stdin_: _IO_FILE_plus;
}
extern "C" {
    #[link_name = "\u{1}_IO_2_1_stdout_"]
    pub static mut _IO_2_1_stdout_: _IO_FILE_plus;
}
extern "C" {
    #[link_name = "\u{1}_IO_2_1_stderr_"]
    pub static mut _IO_2_1_stderr_: _IO_FILE_plus;
}
pub type __io_read_fn = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type __io_write_fn = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> __ssize_t,
>;
pub type __io_seek_fn = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type __io_close_fn = ::std::option::Option<
    unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn __underflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __uflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_getc(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_putc(__c: ::std::os::raw::c_int, __fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_feof(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_ferror(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_peekc_locked(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_flockfile(arg1: *mut _IO_FILE);
}
extern "C" {
    pub fn _IO_funlockfile(arg1: *mut _IO_FILE);
}
extern "C" {
    pub fn _IO_ftrylockfile(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_vfscanf(
        arg1: *mut _IO_FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut __va_list_tag,
        arg4: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_vfprintf(
        arg1: *mut _IO_FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_padn(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int, arg3: __ssize_t)
        -> __ssize_t;
}
extern "C" {
    pub fn _IO_sgetn(arg1: *mut _IO_FILE, arg2: *mut ::std::os::raw::c_void, arg3: usize) -> usize;
}
extern "C" {
    pub fn _IO_seekoff(
        arg1: *mut _IO_FILE,
        arg2: __off64_t,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> __off64_t;
}
extern "C" {
    pub fn _IO_seekpos(
        arg1: *mut _IO_FILE,
        arg2: __off64_t,
        arg3: ::std::os::raw::c_int,
    ) -> __off64_t;
}
extern "C" {
    pub fn _IO_free_backup_area(arg1: *mut _IO_FILE);
}
pub type off_t = __off_t;
pub type fpos_t = _G_fpos_t;
extern "C" {
    #[link_name = "\u{1}stdin"]
    pub static mut stdin: *mut _IO_FILE;
}
extern "C" {
    #[link_name = "\u{1}stdout"]
    pub static mut stdout: *mut _IO_FILE;
}
extern "C" {
    #[link_name = "\u{1}stderr"]
    pub static mut stderr: *mut _IO_FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __s: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}sys_nerr"]
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}sys_errlist"]
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type useconds_t = __useconds_t;
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
extern "C" {
    pub fn access(
        __name: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faccessat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lseek(
        __fd: ::std::os::raw::c_int,
        __offset: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> __off_t;
}
extern "C" {
    pub fn close(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn read(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
    ) -> isize;
}
extern "C" {
    pub fn write(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> isize;
}
extern "C" {
    pub fn pread(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pwrite(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pipe(__pipedes: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alarm(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sleep(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ualarm(__value: __useconds_t, __interval: __useconds_t) -> __useconds_t;
}
extern "C" {
    pub fn usleep(__useconds: __useconds_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pause() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchown(
        __fd: ::std::os::raw::c_int,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchownat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchdir(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getcwd(__buf: *mut ::std::os::raw::c_char, __size: usize)
        -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getwd(__buf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn dup(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dup2(__fd: ::std::os::raw::c_int, __fd2: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__environ"]
    pub static mut __environ: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn execve(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fexecve(
        __fd: ::std::os::raw::c_int,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execv(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execle(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execl(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execvp(
        __file: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execlp(
        __file: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nice(__inc: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _exit(__status: ::std::os::raw::c_int);
}
pub const _PC_LINK_MAX: _bindgen_ty_1 = 0;
pub const _PC_MAX_CANON: _bindgen_ty_1 = 1;
pub const _PC_MAX_INPUT: _bindgen_ty_1 = 2;
pub const _PC_NAME_MAX: _bindgen_ty_1 = 3;
pub const _PC_PATH_MAX: _bindgen_ty_1 = 4;
pub const _PC_PIPE_BUF: _bindgen_ty_1 = 5;
pub const _PC_CHOWN_RESTRICTED: _bindgen_ty_1 = 6;
pub const _PC_NO_TRUNC: _bindgen_ty_1 = 7;
pub const _PC_VDISABLE: _bindgen_ty_1 = 8;
pub const _PC_SYNC_IO: _bindgen_ty_1 = 9;
pub const _PC_ASYNC_IO: _bindgen_ty_1 = 10;
pub const _PC_PRIO_IO: _bindgen_ty_1 = 11;
pub const _PC_SOCK_MAXBUF: _bindgen_ty_1 = 12;
pub const _PC_FILESIZEBITS: _bindgen_ty_1 = 13;
pub const _PC_REC_INCR_XFER_SIZE: _bindgen_ty_1 = 14;
pub const _PC_REC_MAX_XFER_SIZE: _bindgen_ty_1 = 15;
pub const _PC_REC_MIN_XFER_SIZE: _bindgen_ty_1 = 16;
pub const _PC_REC_XFER_ALIGN: _bindgen_ty_1 = 17;
pub const _PC_ALLOC_SIZE_MIN: _bindgen_ty_1 = 18;
pub const _PC_SYMLINK_MAX: _bindgen_ty_1 = 19;
pub const _PC_2_SYMLINKS: _bindgen_ty_1 = 20;
pub type _bindgen_ty_1 = u32;
pub const _SC_ARG_MAX: _bindgen_ty_2 = 0;
pub const _SC_CHILD_MAX: _bindgen_ty_2 = 1;
pub const _SC_CLK_TCK: _bindgen_ty_2 = 2;
pub const _SC_NGROUPS_MAX: _bindgen_ty_2 = 3;
pub const _SC_OPEN_MAX: _bindgen_ty_2 = 4;
pub const _SC_STREAM_MAX: _bindgen_ty_2 = 5;
pub const _SC_TZNAME_MAX: _bindgen_ty_2 = 6;
pub const _SC_JOB_CONTROL: _bindgen_ty_2 = 7;
pub const _SC_SAVED_IDS: _bindgen_ty_2 = 8;
pub const _SC_REALTIME_SIGNALS: _bindgen_ty_2 = 9;
pub const _SC_PRIORITY_SCHEDULING: _bindgen_ty_2 = 10;
pub const _SC_TIMERS: _bindgen_ty_2 = 11;
pub const _SC_ASYNCHRONOUS_IO: _bindgen_ty_2 = 12;
pub const _SC_PRIORITIZED_IO: _bindgen_ty_2 = 13;
pub const _SC_SYNCHRONIZED_IO: _bindgen_ty_2 = 14;
pub const _SC_FSYNC: _bindgen_ty_2 = 15;
pub const _SC_MAPPED_FILES: _bindgen_ty_2 = 16;
pub const _SC_MEMLOCK: _bindgen_ty_2 = 17;
pub const _SC_MEMLOCK_RANGE: _bindgen_ty_2 = 18;
pub const _SC_MEMORY_PROTECTION: _bindgen_ty_2 = 19;
pub const _SC_MESSAGE_PASSING: _bindgen_ty_2 = 20;
pub const _SC_SEMAPHORES: _bindgen_ty_2 = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: _bindgen_ty_2 = 22;
pub const _SC_AIO_LISTIO_MAX: _bindgen_ty_2 = 23;
pub const _SC_AIO_MAX: _bindgen_ty_2 = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: _bindgen_ty_2 = 25;
pub const _SC_DELAYTIMER_MAX: _bindgen_ty_2 = 26;
pub const _SC_MQ_OPEN_MAX: _bindgen_ty_2 = 27;
pub const _SC_MQ_PRIO_MAX: _bindgen_ty_2 = 28;
pub const _SC_VERSION: _bindgen_ty_2 = 29;
pub const _SC_PAGESIZE: _bindgen_ty_2 = 30;
pub const _SC_RTSIG_MAX: _bindgen_ty_2 = 31;
pub const _SC_SEM_NSEMS_MAX: _bindgen_ty_2 = 32;
pub const _SC_SEM_VALUE_MAX: _bindgen_ty_2 = 33;
pub const _SC_SIGQUEUE_MAX: _bindgen_ty_2 = 34;
pub const _SC_TIMER_MAX: _bindgen_ty_2 = 35;
pub const _SC_BC_BASE_MAX: _bindgen_ty_2 = 36;
pub const _SC_BC_DIM_MAX: _bindgen_ty_2 = 37;
pub const _SC_BC_SCALE_MAX: _bindgen_ty_2 = 38;
pub const _SC_BC_STRING_MAX: _bindgen_ty_2 = 39;
pub const _SC_COLL_WEIGHTS_MAX: _bindgen_ty_2 = 40;
pub const _SC_EQUIV_CLASS_MAX: _bindgen_ty_2 = 41;
pub const _SC_EXPR_NEST_MAX: _bindgen_ty_2 = 42;
pub const _SC_LINE_MAX: _bindgen_ty_2 = 43;
pub const _SC_RE_DUP_MAX: _bindgen_ty_2 = 44;
pub const _SC_CHARCLASS_NAME_MAX: _bindgen_ty_2 = 45;
pub const _SC_2_VERSION: _bindgen_ty_2 = 46;
pub const _SC_2_C_BIND: _bindgen_ty_2 = 47;
pub const _SC_2_C_DEV: _bindgen_ty_2 = 48;
pub const _SC_2_FORT_DEV: _bindgen_ty_2 = 49;
pub const _SC_2_FORT_RUN: _bindgen_ty_2 = 50;
pub const _SC_2_SW_DEV: _bindgen_ty_2 = 51;
pub const _SC_2_LOCALEDEF: _bindgen_ty_2 = 52;
pub const _SC_PII: _bindgen_ty_2 = 53;
pub const _SC_PII_XTI: _bindgen_ty_2 = 54;
pub const _SC_PII_SOCKET: _bindgen_ty_2 = 55;
pub const _SC_PII_INTERNET: _bindgen_ty_2 = 56;
pub const _SC_PII_OSI: _bindgen_ty_2 = 57;
pub const _SC_POLL: _bindgen_ty_2 = 58;
pub const _SC_SELECT: _bindgen_ty_2 = 59;
pub const _SC_UIO_MAXIOV: _bindgen_ty_2 = 60;
pub const _SC_IOV_MAX: _bindgen_ty_2 = 60;
pub const _SC_PII_INTERNET_STREAM: _bindgen_ty_2 = 61;
pub const _SC_PII_INTERNET_DGRAM: _bindgen_ty_2 = 62;
pub const _SC_PII_OSI_COTS: _bindgen_ty_2 = 63;
pub const _SC_PII_OSI_CLTS: _bindgen_ty_2 = 64;
pub const _SC_PII_OSI_M: _bindgen_ty_2 = 65;
pub const _SC_T_IOV_MAX: _bindgen_ty_2 = 66;
pub const _SC_THREADS: _bindgen_ty_2 = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: _bindgen_ty_2 = 68;
pub const _SC_GETGR_R_SIZE_MAX: _bindgen_ty_2 = 69;
pub const _SC_GETPW_R_SIZE_MAX: _bindgen_ty_2 = 70;
pub const _SC_LOGIN_NAME_MAX: _bindgen_ty_2 = 71;
pub const _SC_TTY_NAME_MAX: _bindgen_ty_2 = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: _bindgen_ty_2 = 73;
pub const _SC_THREAD_KEYS_MAX: _bindgen_ty_2 = 74;
pub const _SC_THREAD_STACK_MIN: _bindgen_ty_2 = 75;
pub const _SC_THREAD_THREADS_MAX: _bindgen_ty_2 = 76;
pub const _SC_THREAD_ATTR_STACKADDR: _bindgen_ty_2 = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: _bindgen_ty_2 = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: _bindgen_ty_2 = 79;
pub const _SC_THREAD_PRIO_INHERIT: _bindgen_ty_2 = 80;
pub const _SC_THREAD_PRIO_PROTECT: _bindgen_ty_2 = 81;
pub const _SC_THREAD_PROCESS_SHARED: _bindgen_ty_2 = 82;
pub const _SC_NPROCESSORS_CONF: _bindgen_ty_2 = 83;
pub const _SC_NPROCESSORS_ONLN: _bindgen_ty_2 = 84;
pub const _SC_PHYS_PAGES: _bindgen_ty_2 = 85;
pub const _SC_AVPHYS_PAGES: _bindgen_ty_2 = 86;
pub const _SC_ATEXIT_MAX: _bindgen_ty_2 = 87;
pub const _SC_PASS_MAX: _bindgen_ty_2 = 88;
pub const _SC_XOPEN_VERSION: _bindgen_ty_2 = 89;
pub const _SC_XOPEN_XCU_VERSION: _bindgen_ty_2 = 90;
pub const _SC_XOPEN_UNIX: _bindgen_ty_2 = 91;
pub const _SC_XOPEN_CRYPT: _bindgen_ty_2 = 92;
pub const _SC_XOPEN_ENH_I18N: _bindgen_ty_2 = 93;
pub const _SC_XOPEN_SHM: _bindgen_ty_2 = 94;
pub const _SC_2_CHAR_TERM: _bindgen_ty_2 = 95;
pub const _SC_2_C_VERSION: _bindgen_ty_2 = 96;
pub const _SC_2_UPE: _bindgen_ty_2 = 97;
pub const _SC_XOPEN_XPG2: _bindgen_ty_2 = 98;
pub const _SC_XOPEN_XPG3: _bindgen_ty_2 = 99;
pub const _SC_XOPEN_XPG4: _bindgen_ty_2 = 100;
pub const _SC_CHAR_BIT: _bindgen_ty_2 = 101;
pub const _SC_CHAR_MAX: _bindgen_ty_2 = 102;
pub const _SC_CHAR_MIN: _bindgen_ty_2 = 103;
pub const _SC_INT_MAX: _bindgen_ty_2 = 104;
pub const _SC_INT_MIN: _bindgen_ty_2 = 105;
pub const _SC_LONG_BIT: _bindgen_ty_2 = 106;
pub const _SC_WORD_BIT: _bindgen_ty_2 = 107;
pub const _SC_MB_LEN_MAX: _bindgen_ty_2 = 108;
pub const _SC_NZERO: _bindgen_ty_2 = 109;
pub const _SC_SSIZE_MAX: _bindgen_ty_2 = 110;
pub const _SC_SCHAR_MAX: _bindgen_ty_2 = 111;
pub const _SC_SCHAR_MIN: _bindgen_ty_2 = 112;
pub const _SC_SHRT_MAX: _bindgen_ty_2 = 113;
pub const _SC_SHRT_MIN: _bindgen_ty_2 = 114;
pub const _SC_UCHAR_MAX: _bindgen_ty_2 = 115;
pub const _SC_UINT_MAX: _bindgen_ty_2 = 116;
pub const _SC_ULONG_MAX: _bindgen_ty_2 = 117;
pub const _SC_USHRT_MAX: _bindgen_ty_2 = 118;
pub const _SC_NL_ARGMAX: _bindgen_ty_2 = 119;
pub const _SC_NL_LANGMAX: _bindgen_ty_2 = 120;
pub const _SC_NL_MSGMAX: _bindgen_ty_2 = 121;
pub const _SC_NL_NMAX: _bindgen_ty_2 = 122;
pub const _SC_NL_SETMAX: _bindgen_ty_2 = 123;
pub const _SC_NL_TEXTMAX: _bindgen_ty_2 = 124;
pub const _SC_XBS5_ILP32_OFF32: _bindgen_ty_2 = 125;
pub const _SC_XBS5_ILP32_OFFBIG: _bindgen_ty_2 = 126;
pub const _SC_XBS5_LP64_OFF64: _bindgen_ty_2 = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: _bindgen_ty_2 = 128;
pub const _SC_XOPEN_LEGACY: _bindgen_ty_2 = 129;
pub const _SC_XOPEN_REALTIME: _bindgen_ty_2 = 130;
pub const _SC_XOPEN_REALTIME_THREADS: _bindgen_ty_2 = 131;
pub const _SC_ADVISORY_INFO: _bindgen_ty_2 = 132;
pub const _SC_BARRIERS: _bindgen_ty_2 = 133;
pub const _SC_BASE: _bindgen_ty_2 = 134;
pub const _SC_C_LANG_SUPPORT: _bindgen_ty_2 = 135;
pub const _SC_C_LANG_SUPPORT_R: _bindgen_ty_2 = 136;
pub const _SC_CLOCK_SELECTION: _bindgen_ty_2 = 137;
pub const _SC_CPUTIME: _bindgen_ty_2 = 138;
pub const _SC_THREAD_CPUTIME: _bindgen_ty_2 = 139;
pub const _SC_DEVICE_IO: _bindgen_ty_2 = 140;
pub const _SC_DEVICE_SPECIFIC: _bindgen_ty_2 = 141;
pub const _SC_DEVICE_SPECIFIC_R: _bindgen_ty_2 = 142;
pub const _SC_FD_MGMT: _bindgen_ty_2 = 143;
pub const _SC_FIFO: _bindgen_ty_2 = 144;
pub const _SC_PIPE: _bindgen_ty_2 = 145;
pub const _SC_FILE_ATTRIBUTES: _bindgen_ty_2 = 146;
pub const _SC_FILE_LOCKING: _bindgen_ty_2 = 147;
pub const _SC_FILE_SYSTEM: _bindgen_ty_2 = 148;
pub const _SC_MONOTONIC_CLOCK: _bindgen_ty_2 = 149;
pub const _SC_MULTI_PROCESS: _bindgen_ty_2 = 150;
pub const _SC_SINGLE_PROCESS: _bindgen_ty_2 = 151;
pub const _SC_NETWORKING: _bindgen_ty_2 = 152;
pub const _SC_READER_WRITER_LOCKS: _bindgen_ty_2 = 153;
pub const _SC_SPIN_LOCKS: _bindgen_ty_2 = 154;
pub const _SC_REGEXP: _bindgen_ty_2 = 155;
pub const _SC_REGEX_VERSION: _bindgen_ty_2 = 156;
pub const _SC_SHELL: _bindgen_ty_2 = 157;
pub const _SC_SIGNALS: _bindgen_ty_2 = 158;
pub const _SC_SPAWN: _bindgen_ty_2 = 159;
pub const _SC_SPORADIC_SERVER: _bindgen_ty_2 = 160;
pub const _SC_THREAD_SPORADIC_SERVER: _bindgen_ty_2 = 161;
pub const _SC_SYSTEM_DATABASE: _bindgen_ty_2 = 162;
pub const _SC_SYSTEM_DATABASE_R: _bindgen_ty_2 = 163;
pub const _SC_TIMEOUTS: _bindgen_ty_2 = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: _bindgen_ty_2 = 165;
pub const _SC_USER_GROUPS: _bindgen_ty_2 = 166;
pub const _SC_USER_GROUPS_R: _bindgen_ty_2 = 167;
pub const _SC_2_PBS: _bindgen_ty_2 = 168;
pub const _SC_2_PBS_ACCOUNTING: _bindgen_ty_2 = 169;
pub const _SC_2_PBS_LOCATE: _bindgen_ty_2 = 170;
pub const _SC_2_PBS_MESSAGE: _bindgen_ty_2 = 171;
pub const _SC_2_PBS_TRACK: _bindgen_ty_2 = 172;
pub const _SC_SYMLOOP_MAX: _bindgen_ty_2 = 173;
pub const _SC_STREAMS: _bindgen_ty_2 = 174;
pub const _SC_2_PBS_CHECKPOINT: _bindgen_ty_2 = 175;
pub const _SC_V6_ILP32_OFF32: _bindgen_ty_2 = 176;
pub const _SC_V6_ILP32_OFFBIG: _bindgen_ty_2 = 177;
pub const _SC_V6_LP64_OFF64: _bindgen_ty_2 = 178;
pub const _SC_V6_LPBIG_OFFBIG: _bindgen_ty_2 = 179;
pub const _SC_HOST_NAME_MAX: _bindgen_ty_2 = 180;
pub const _SC_TRACE: _bindgen_ty_2 = 181;
pub const _SC_TRACE_EVENT_FILTER: _bindgen_ty_2 = 182;
pub const _SC_TRACE_INHERIT: _bindgen_ty_2 = 183;
pub const _SC_TRACE_LOG: _bindgen_ty_2 = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: _bindgen_ty_2 = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: _bindgen_ty_2 = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: _bindgen_ty_2 = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: _bindgen_ty_2 = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: _bindgen_ty_2 = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: _bindgen_ty_2 = 190;
pub const _SC_LEVEL2_CACHE_SIZE: _bindgen_ty_2 = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: _bindgen_ty_2 = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: _bindgen_ty_2 = 193;
pub const _SC_LEVEL3_CACHE_SIZE: _bindgen_ty_2 = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: _bindgen_ty_2 = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: _bindgen_ty_2 = 196;
pub const _SC_LEVEL4_CACHE_SIZE: _bindgen_ty_2 = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: _bindgen_ty_2 = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: _bindgen_ty_2 = 199;
pub const _SC_IPV6: _bindgen_ty_2 = 235;
pub const _SC_RAW_SOCKETS: _bindgen_ty_2 = 236;
pub const _SC_V7_ILP32_OFF32: _bindgen_ty_2 = 237;
pub const _SC_V7_ILP32_OFFBIG: _bindgen_ty_2 = 238;
pub const _SC_V7_LP64_OFF64: _bindgen_ty_2 = 239;
pub const _SC_V7_LPBIG_OFFBIG: _bindgen_ty_2 = 240;
pub const _SC_SS_REPL_MAX: _bindgen_ty_2 = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: _bindgen_ty_2 = 242;
pub const _SC_TRACE_NAME_MAX: _bindgen_ty_2 = 243;
pub const _SC_TRACE_SYS_MAX: _bindgen_ty_2 = 244;
pub const _SC_TRACE_USER_EVENT_MAX: _bindgen_ty_2 = 245;
pub const _SC_XOPEN_STREAMS: _bindgen_ty_2 = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: _bindgen_ty_2 = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: _bindgen_ty_2 = 248;
pub type _bindgen_ty_2 = u32;
pub const _CS_PATH: _bindgen_ty_3 = 0;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: _bindgen_ty_3 = 1;
pub const _CS_GNU_LIBC_VERSION: _bindgen_ty_3 = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: _bindgen_ty_3 = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: _bindgen_ty_3 = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: _bindgen_ty_3 = 5;
pub const _CS_LFS_CFLAGS: _bindgen_ty_3 = 1000;
pub const _CS_LFS_LDFLAGS: _bindgen_ty_3 = 1001;
pub const _CS_LFS_LIBS: _bindgen_ty_3 = 1002;
pub const _CS_LFS_LINTFLAGS: _bindgen_ty_3 = 1003;
pub const _CS_LFS64_CFLAGS: _bindgen_ty_3 = 1004;
pub const _CS_LFS64_LDFLAGS: _bindgen_ty_3 = 1005;
pub const _CS_LFS64_LIBS: _bindgen_ty_3 = 1006;
pub const _CS_LFS64_LINTFLAGS: _bindgen_ty_3 = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: _bindgen_ty_3 = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: _bindgen_ty_3 = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: _bindgen_ty_3 = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: _bindgen_ty_3 = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: _bindgen_ty_3 = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_3 = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: _bindgen_ty_3 = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_3 = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: _bindgen_ty_3 = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: _bindgen_ty_3 = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: _bindgen_ty_3 = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: _bindgen_ty_3 = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_3 = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_3 = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: _bindgen_ty_3 = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_3 = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: _bindgen_ty_3 = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: _bindgen_ty_3 = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: _bindgen_ty_3 = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: _bindgen_ty_3 = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: _bindgen_ty_3 = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_3 = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: _bindgen_ty_3 = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_3 = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: _bindgen_ty_3 = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: _bindgen_ty_3 = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: _bindgen_ty_3 = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: _bindgen_ty_3 = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_3 = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_3 = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: _bindgen_ty_3 = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_3 = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: _bindgen_ty_3 = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: _bindgen_ty_3 = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: _bindgen_ty_3 = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: _bindgen_ty_3 = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: _bindgen_ty_3 = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_3 = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: _bindgen_ty_3 = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_3 = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: _bindgen_ty_3 = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: _bindgen_ty_3 = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: _bindgen_ty_3 = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: _bindgen_ty_3 = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_3 = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_3 = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: _bindgen_ty_3 = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_3 = 1147;
pub const _CS_V6_ENV: _bindgen_ty_3 = 1148;
pub const _CS_V7_ENV: _bindgen_ty_3 = 1149;
pub type _bindgen_ty_3 = u32;
extern "C" {
    pub fn pathconf(
        __path: *const ::std::os::raw::c_char,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fpathconf(
        __fd: ::std::os::raw::c_int,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sysconf(__name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn confstr(
        __name: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> usize;
}
extern "C" {
    pub fn getpid() -> __pid_t;
}
extern "C" {
    pub fn getppid() -> __pid_t;
}
extern "C" {
    pub fn getpgrp() -> __pid_t;
}
extern "C" {
    pub fn __getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpgrp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setsid() -> __pid_t;
}
extern "C" {
    pub fn getsid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getuid() -> __uid_t;
}
extern "C" {
    pub fn geteuid() -> __uid_t;
}
extern "C" {
    pub fn getgid() -> __gid_t;
}
extern "C" {
    pub fn getegid() -> __gid_t;
}
extern "C" {
    pub fn getgroups(__size: ::std::os::raw::c_int, __list: *mut __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seteuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setgid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setregid(__rgid: __gid_t, __egid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setegid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fork() -> __pid_t;
}
extern "C" {
    pub fn vfork() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyname(__fd: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ttyname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isatty(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyslot() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn link(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn linkat(
        __fromfd: ::std::os::raw::c_int,
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn symlink(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlink(
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> isize;
}
extern "C" {
    pub fn symlinkat(
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlinkat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> isize;
}
extern "C" {
    pub fn unlink(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlinkat(
        __fd: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcgetpgrp(__fd: ::std::os::raw::c_int) -> __pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fd: ::std::os::raw::c_int, __pgrp_id: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getlogin() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getlogin_r(
        __name: *mut ::std::os::raw::c_char,
        __name_len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setlogin(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}optarg"]
    pub static mut optarg: *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}optind"]
    pub static mut optind: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}opterr"]
    pub static mut opterr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}optopt"]
    pub static mut optopt: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getopt(
        ___argc: ::std::os::raw::c_int,
        ___argv: *const *mut ::std::os::raw::c_char,
        __shortopts: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostname(__name: *mut ::std::os::raw::c_char, __len: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostname(
        __name: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostid(__id: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdomainname(
        __name: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setdomainname(
        __name: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vhangup() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn revoke(__file: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn profil(
        __sample_buffer: *mut ::std::os::raw::c_ushort,
        __size: usize,
        __offset: usize,
        __scale: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acct(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getusershell() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn endusershell();
}
extern "C" {
    pub fn setusershell();
}
extern "C" {
    pub fn daemon(
        __nochdir: ::std::os::raw::c_int,
        __noclose: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chroot(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpass(__prompt: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fsync(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostid() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sync();
}
extern "C" {
    pub fn getpagesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdtablesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn truncate(
        __file: *const ::std::os::raw::c_char,
        __length: __off_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftruncate(__fd: ::std::os::raw::c_int, __length: __off_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn brk(__addr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sbrk(__delta: isize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn syscall(__sysno: ::std::os::raw::c_long, ...) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lockf(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        __len: __off_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdatasync(__fildes: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getentropy(
        __buffer: *mut ::std::os::raw::c_void,
        __length: usize,
    ) -> ::std::os::raw::c_int;
}
pub type time_t = __time_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
pub type suseconds_t = __suseconds_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigset_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fd_set>())).__fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(__fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: ::std::os::raw::c_int,
    pub tz_dsttime: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_timezone() {
    assert_eq!(
        ::std::mem::size_of::<timezone>(),
        8usize,
        concat!("Size of: ", stringify!(timezone))
    );
    assert_eq!(
        ::std::mem::align_of::<timezone>(),
        4usize,
        concat!("Alignment of ", stringify!(timezone))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timezone>())).tz_minuteswest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timezone),
            "::",
            stringify!(tz_minuteswest)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timezone>())).tz_dsttime as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(timezone),
            "::",
            stringify!(tz_dsttime)
        )
    );
}
pub type __timezone_ptr_t = *mut timezone;
extern "C" {
    pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn adjtime(__delta: *const timeval, __olddelta: *mut timeval) -> ::std::os::raw::c_int;
}
pub const __itimer_which_ITIMER_REAL: __itimer_which = 0;
pub const __itimer_which_ITIMER_VIRTUAL: __itimer_which = 1;
pub const __itimer_which_ITIMER_PROF: __itimer_which = 2;
pub type __itimer_which = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
#[test]
fn bindgen_test_layout_itimerval() {
    assert_eq!(
        ::std::mem::size_of::<itimerval>(),
        32usize,
        concat!("Size of: ", stringify!(itimerval))
    );
    assert_eq!(
        ::std::mem::align_of::<itimerval>(),
        8usize,
        concat!("Alignment of ", stringify!(itimerval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerval>())).it_interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerval),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerval>())).it_value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerval),
            "::",
            stringify!(it_value)
        )
    );
}
pub type __itimer_which_t = ::std::os::raw::c_int;
extern "C" {
    pub fn getitimer(__which: __itimer_which_t, __value: *mut itimerval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lutimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimes(__fd: ::std::os::raw::c_int, __tvp: *const timeval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __errno_location() -> *mut ::std::os::raw::c_int;
}
pub const _ISupper: _bindgen_ty_4 = 256;
pub const _ISlower: _bindgen_ty_4 = 512;
pub const _ISalpha: _bindgen_ty_4 = 1024;
pub const _ISdigit: _bindgen_ty_4 = 2048;
pub const _ISxdigit: _bindgen_ty_4 = 4096;
pub const _ISspace: _bindgen_ty_4 = 8192;
pub const _ISprint: _bindgen_ty_4 = 16384;
pub const _ISgraph: _bindgen_ty_4 = 32768;
pub const _ISblank: _bindgen_ty_4 = 1;
pub const _IScntrl: _bindgen_ty_4 = 2;
pub const _ISpunct: _bindgen_ty_4 = 4;
pub const _ISalnum: _bindgen_ty_4 = 8;
pub type _bindgen_ty_4 = u32;
extern "C" {
    pub fn __ctype_b_loc() -> *mut *const ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
extern "C" {
    pub fn isalnum(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toascii(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _toupper(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tolower(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
#[test]
fn bindgen_test_layout___locale_struct() {
    assert_eq!(
        ::std::mem::size_of::<__locale_struct>(),
        232usize,
        concat!("Size of: ", stringify!(__locale_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<__locale_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(__locale_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__locale_struct>())).__locales as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__locales)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__locale_struct>())).__ctype_b as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_b)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__locale_struct>())).__ctype_tolower as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_tolower)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__locale_struct>())).__ctype_toupper as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_toupper)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__locale_struct>())).__names as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__names)
        )
    );
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn isalnum_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isalpha_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iscntrl_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn islower_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isgraph_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isprint_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ispunct_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isspace_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isupper_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isxdigit_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isblank_l(arg1: ::std::os::raw::c_int, arg2: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tolower_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn toupper_l(__c: ::std::os::raw::c_int, __l: locale_t) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flock {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[test]
fn bindgen_test_layout_flock() {
    assert_eq!(
        ::std::mem::size_of::<flock>(),
        32usize,
        concat!("Size of: ", stringify!(flock))
    );
    assert_eq!(
        ::std::mem::align_of::<flock>(),
        8usize,
        concat!("Alignment of ", stringify!(flock))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_whence as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_whence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_start as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flock>())).l_pid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(flock),
            "::",
            stringify!(l_pid)
        )
    );
}
pub type mode_t = __mode_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3usize],
}
#[test]
fn bindgen_test_layout_stat() {
    assert_eq!(
        ::std::mem::size_of::<stat>(),
        144usize,
        concat!("Size of: ", stringify!(stat))
    );
    assert_eq!(
        ::std::mem::align_of::<stat>(),
        8usize,
        concat!("Alignment of ", stringify!(stat))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_dev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_dev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_ino as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_ino)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_nlink as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_nlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_mode as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_uid as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_uid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_gid as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_gid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).__pad0 as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(__pad0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_rdev as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_rdev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_size as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_blksize as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_blksize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_blocks as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_blocks)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_atim as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_atim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_mtim as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_mtim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).st_ctim as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(st_ctim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<stat>())).__glibc_reserved as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(stat),
            "::",
            stringify!(__glibc_reserved)
        )
    );
}
extern "C" {
    pub fn fcntl(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn open(
        __file: *const ::std::os::raw::c_char,
        __oflag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn openat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __oflag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn creat(__file: *const ::std::os::raw::c_char, __mode: mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_fadvise(
        __fd: ::std::os::raw::c_int,
        __offset: off_t,
        __len: off_t,
        __advise: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_fallocate(
        __fd: ::std::os::raw::c_int,
        __offset: off_t,
        __len: off_t,
    ) -> ::std::os::raw::c_int;
}
pub type dev_t = __dev_t;
pub type ino_t = __ino_t;
pub type nlink_t = __nlink_t;
extern "C" {
    pub fn stat(__file: *const ::std::os::raw::c_char, __buf: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstat(__fd: ::std::os::raw::c_int, __buf: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstatat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __buf: *mut stat,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lstat(__file: *const ::std::os::raw::c_char, __buf: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chmod(__file: *const ::std::os::raw::c_char, __mode: __mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchmod(__file: *const ::std::os::raw::c_char, __mode: __mode_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchmod(__fd: ::std::os::raw::c_int, __mode: __mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchmodat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn umask(__mask: __mode_t) -> __mode_t;
}
extern "C" {
    pub fn mkdir(__path: *const ::std::os::raw::c_char, __mode: __mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdirat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mknod(
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mknodat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkfifo(__path: *const ::std::os::raw::c_char, __mode: __mode_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkfifoat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimensat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __times: *const timespec,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimens(__fd: ::std::os::raw::c_int, __times: *const timespec)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __fxstat(
        __ver: ::std::os::raw::c_int,
        __fildes: ::std::os::raw::c_int,
        __stat_buf: *mut stat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xstat(
        __ver: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __lxstat(
        __ver: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __fxstatat(
        __ver: ::std::os::raw::c_int,
        __fildes: ::std::os::raw::c_int,
        __filename: *const ::std::os::raw::c_char,
        __stat_buf: *mut stat,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmknod(
        __ver: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmknodat(
        __ver: ::std::os::raw::c_int,
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> ::std::os::raw::c_int;
}
pub type wchar_t = ::std::os::raw::c_int;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = u32;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        ::std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<div_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<div_t>())).rem as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;
extern "C" {
    pub fn gnu_dev_major(__dev: __dev_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn gnu_dev_minor(__dev: __dev_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn gnu_dev_makedev(
        __major: ::std::os::raw::c_uint,
        __minor: ::std::os::raw::c_uint,
    ) -> __dev_t;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__readers as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__readers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__writers as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__wrphase_futex as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__wrphase_futex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__writers_futex as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers_futex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad3 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad4 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__cur_writer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__cur_writer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__shared as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__shared)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__rwelision as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__rwelision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad1 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad2 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_list>())).__prev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__prev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_list>())).__next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__lock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__owner as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__owner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__nusers as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__nusers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__kind as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__spins as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__spins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__elision as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__elision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__list)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq32)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start32)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_refs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_refs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g1_orig_size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_orig_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__wrefs as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wrefs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_signals as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_signals)
        )
    );
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutexattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutexattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_condattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_condattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlockattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlockattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrier_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrier_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrierattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrierattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__align)
        )
    );
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    assert_eq!(
        ::std::mem::size_of::<random_data>(),
        48usize,
        concat!("Size of: ", stringify!(random_data))
    );
    assert_eq!(
        ::std::mem::align_of::<random_data>(),
        8usize,
        concat!("Alignment of ", stringify!(random_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).fptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(fptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).state as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rand_type as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rand_deg as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_deg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rand_sep as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_sep)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).end_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(end_ptr)
        )
    );
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    assert_eq!(
        ::std::mem::size_of::<drand48_data>(),
        24usize,
        concat!("Size of: ", stringify!(drand48_data))
    );
    assert_eq!(
        ::std::mem::align_of::<drand48_data>(),
        8usize,
        concat!("Alignment of ", stringify!(drand48_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__old_x as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__old_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__c as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__c)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__init as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__a as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__a)
        )
    );
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const ::std::os::raw::c_void, arg2: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}__xpg_strerror_r"]
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
}
extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_tm() {
    assert_eq!(
        ::std::mem::size_of::<tm>(),
        56usize,
        concat!("Size of: ", stringify!(tm))
    );
    assert_eq!(
        ::std::mem::align_of::<tm>(),
        8usize,
        concat!("Alignment of ", stringify!(tm))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_min as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_min)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_hour as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_mday as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_mday)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_mon as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_mon)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_year as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_wday as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_wday)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_yday as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_yday)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_isdst as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_isdst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_gmtoff as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_gmtoff)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tm>())).tm_zone as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tm),
            "::",
            stringify!(tm_zone)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[test]
fn bindgen_test_layout_itimerspec() {
    assert_eq!(
        ::std::mem::size_of::<itimerspec>(),
        32usize,
        concat!("Size of: ", stringify!(itimerspec))
    );
    assert_eq!(
        ::std::mem::align_of::<itimerspec>(),
        8usize,
        concat!("Alignment of ", stringify!(itimerspec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerspec>())).it_interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerspec),
            "::",
            stringify!(it_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<itimerspec>())).it_value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(itimerspec),
            "::",
            stringify!(it_value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}__tzname"]
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    #[link_name = "\u{1}__daylight"]
    pub static mut __daylight: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__timezone"]
    pub static mut __timezone: ::std::os::raw::c_long;
}
extern "C" {
    #[link_name = "\u{1}tzname"]
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    #[link_name = "\u{1}daylight"]
    pub static mut daylight: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}timezone"]
    pub static mut timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub fn stime(__when: *const time_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_imaxdiv_t() {
    assert_eq!(
        ::std::mem::size_of::<imaxdiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(imaxdiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<imaxdiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(imaxdiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<imaxdiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(imaxdiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<imaxdiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(imaxdiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
pub type static_assert_test_213sizeof_uint64_t = [::std::os::raw::c_char; 1usize];
pub type bson_unichar_t = u32;
pub const bson_context_flags_t_BSON_CONTEXT_NONE: bson_context_flags_t = 0;
pub const bson_context_flags_t_BSON_CONTEXT_THREAD_SAFE: bson_context_flags_t = 1;
pub const bson_context_flags_t_BSON_CONTEXT_DISABLE_HOST_CACHE: bson_context_flags_t = 2;
pub const bson_context_flags_t_BSON_CONTEXT_DISABLE_PID_CACHE: bson_context_flags_t = 4;
pub const bson_context_flags_t_BSON_CONTEXT_USE_TASK_ID: bson_context_flags_t = 8;
pub type bson_context_flags_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_context_t {
    _unused: [u8; 0],
}
pub type bson_context_t = _bson_context_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bson_t {
    pub flags: u32,
    pub len: u32,
    pub padding: [u8; 120usize],
}
#[test]
fn bindgen_test_layout__bson_t() {
    assert_eq!(
        ::std::mem::size_of::<_bson_t>(),
        128usize,
        concat!("Size of: ", stringify!(_bson_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_t>(),
        4usize,
        concat!("Alignment of ", stringify!(_bson_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_t>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_t),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_t>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_t),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_t>())).padding as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_t),
            "::",
            stringify!(padding)
        )
    );
}
pub type bson_t = _bson_t;
pub type static_assert_test_170bson_t = [::std::os::raw::c_char; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_oid_t {
    pub bytes: [u8; 12usize],
}
#[test]
fn bindgen_test_layout_bson_oid_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_oid_t>(),
        12usize,
        concat!("Size of: ", stringify!(bson_oid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_oid_t>(),
        1usize,
        concat!("Alignment of ", stringify!(bson_oid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_oid_t>())).bytes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_oid_t),
            "::",
            stringify!(bytes)
        )
    );
}
pub type static_assert_test_184oid_t = [::std::os::raw::c_char; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_decimal128_t {
    pub low: u64,
    pub high: u64,
}
#[test]
fn bindgen_test_layout_bson_decimal128_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_decimal128_t>(),
        16usize,
        concat!("Size of: ", stringify!(bson_decimal128_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_decimal128_t>(),
        8usize,
        concat!("Alignment of ", stringify!(bson_decimal128_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_decimal128_t>())).low as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_decimal128_t),
            "::",
            stringify!(low)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_decimal128_t>())).high as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_decimal128_t),
            "::",
            stringify!(high)
        )
    );
}
pub const bson_validate_flags_t_BSON_VALIDATE_NONE: bson_validate_flags_t = 0;
pub const bson_validate_flags_t_BSON_VALIDATE_UTF8: bson_validate_flags_t = 1;
pub const bson_validate_flags_t_BSON_VALIDATE_DOLLAR_KEYS: bson_validate_flags_t = 2;
pub const bson_validate_flags_t_BSON_VALIDATE_DOT_KEYS: bson_validate_flags_t = 4;
pub const bson_validate_flags_t_BSON_VALIDATE_UTF8_ALLOW_NULL: bson_validate_flags_t = 8;
pub const bson_validate_flags_t_BSON_VALIDATE_EMPTY_KEYS: bson_validate_flags_t = 16;
pub type bson_validate_flags_t = u32;
pub const bson_type_t_BSON_TYPE_EOD: bson_type_t = 0;
pub const bson_type_t_BSON_TYPE_DOUBLE: bson_type_t = 1;
pub const bson_type_t_BSON_TYPE_UTF8: bson_type_t = 2;
pub const bson_type_t_BSON_TYPE_DOCUMENT: bson_type_t = 3;
pub const bson_type_t_BSON_TYPE_ARRAY: bson_type_t = 4;
pub const bson_type_t_BSON_TYPE_BINARY: bson_type_t = 5;
pub const bson_type_t_BSON_TYPE_UNDEFINED: bson_type_t = 6;
pub const bson_type_t_BSON_TYPE_OID: bson_type_t = 7;
pub const bson_type_t_BSON_TYPE_BOOL: bson_type_t = 8;
pub const bson_type_t_BSON_TYPE_DATE_TIME: bson_type_t = 9;
pub const bson_type_t_BSON_TYPE_NULL: bson_type_t = 10;
pub const bson_type_t_BSON_TYPE_REGEX: bson_type_t = 11;
pub const bson_type_t_BSON_TYPE_DBPOINTER: bson_type_t = 12;
pub const bson_type_t_BSON_TYPE_CODE: bson_type_t = 13;
pub const bson_type_t_BSON_TYPE_SYMBOL: bson_type_t = 14;
pub const bson_type_t_BSON_TYPE_CODEWSCOPE: bson_type_t = 15;
pub const bson_type_t_BSON_TYPE_INT32: bson_type_t = 16;
pub const bson_type_t_BSON_TYPE_TIMESTAMP: bson_type_t = 17;
pub const bson_type_t_BSON_TYPE_INT64: bson_type_t = 18;
pub const bson_type_t_BSON_TYPE_DECIMAL128: bson_type_t = 19;
pub const bson_type_t_BSON_TYPE_MAXKEY: bson_type_t = 127;
pub const bson_type_t_BSON_TYPE_MINKEY: bson_type_t = 255;
pub type bson_type_t = u32;
pub const bson_subtype_t_BSON_SUBTYPE_BINARY: bson_subtype_t = 0;
pub const bson_subtype_t_BSON_SUBTYPE_FUNCTION: bson_subtype_t = 1;
pub const bson_subtype_t_BSON_SUBTYPE_BINARY_DEPRECATED: bson_subtype_t = 2;
pub const bson_subtype_t_BSON_SUBTYPE_UUID_DEPRECATED: bson_subtype_t = 3;
pub const bson_subtype_t_BSON_SUBTYPE_UUID: bson_subtype_t = 4;
pub const bson_subtype_t_BSON_SUBTYPE_MD5: bson_subtype_t = 5;
pub const bson_subtype_t_BSON_SUBTYPE_USER: bson_subtype_t = 128;
pub type bson_subtype_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bson_value_t {
    pub value_type: bson_type_t,
    pub padding: i32,
    pub value: _bson_value_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _bson_value_t__bindgen_ty_1 {
    pub v_oid: bson_oid_t,
    pub v_int64: i64,
    pub v_int32: i32,
    pub v_int8: i8,
    pub v_double: f64,
    pub v_bool: bool,
    pub v_datetime: i64,
    pub v_timestamp: _bson_value_t__bindgen_ty_1__bindgen_ty_1,
    pub v_utf8: _bson_value_t__bindgen_ty_1__bindgen_ty_2,
    pub v_doc: _bson_value_t__bindgen_ty_1__bindgen_ty_3,
    pub v_binary: _bson_value_t__bindgen_ty_1__bindgen_ty_4,
    pub v_regex: _bson_value_t__bindgen_ty_1__bindgen_ty_5,
    pub v_dbpointer: _bson_value_t__bindgen_ty_1__bindgen_ty_6,
    pub v_code: _bson_value_t__bindgen_ty_1__bindgen_ty_7,
    pub v_codewscope: _bson_value_t__bindgen_ty_1__bindgen_ty_8,
    pub v_symbol: _bson_value_t__bindgen_ty_1__bindgen_ty_9,
    pub v_decimal128: bson_decimal128_t,
    _bindgen_union_align: [u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_1 {
    pub timestamp: u32,
    pub increment: u32,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_1>())).timestamp
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(timestamp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_1>())).increment
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(increment)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_2 {
    pub str: *mut ::std::os::raw::c_char,
    pub len: u32,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_2>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_2>())).str as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(str)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_2>())).len as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_3 {
    pub data: *mut u8,
    pub data_len: u32,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_3>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_3>())).data as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_3>())).data_len
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(data_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_4 {
    pub data: *mut u8,
    pub data_len: u32,
    pub subtype: bson_subtype_t,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_4>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_4>())).data as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_4>())).data_len
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(data_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_4>())).subtype
                as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(subtype)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_5 {
    pub regex: *mut ::std::os::raw::c_char,
    pub options: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_5>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_5>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_5>())).regex as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(regex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_5>())).options
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(options)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_6 {
    pub collection: *mut ::std::os::raw::c_char,
    pub collection_len: u32,
    pub oid: bson_oid_t,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_6() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_6>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_6>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_6>())).collection
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(collection)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_6>())).collection_len
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(collection_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_6>())).oid as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(oid)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_7 {
    pub code: *mut ::std::os::raw::c_char,
    pub code_len: u32,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_7() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_7>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_7)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_7>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_7>())).code as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_7>())).code_len
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(code_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_8 {
    pub code: *mut ::std::os::raw::c_char,
    pub scope_data: *mut u8,
    pub code_len: u32,
    pub scope_len: u32,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_8() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_8>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_8)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_8>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_8>())).code as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_8>())).scope_data
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(scope_data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_8>())).code_len
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(code_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_8>())).scope_len
                as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(scope_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_9 {
    pub symbol: *mut ::std::os::raw::c_char,
    pub len: u32,
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1__bindgen_ty_9() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_9>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_9)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1__bindgen_ty_9>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_9>())).symbol as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_9),
            "::",
            stringify!(symbol)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1__bindgen_ty_9>())).len as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1__bindgen_ty_9),
            "::",
            stringify!(len)
        )
    );
}
#[test]
fn bindgen_test_layout__bson_value_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t__bindgen_ty_1>(),
        24usize,
        concat!("Size of: ", stringify!(_bson_value_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_bson_value_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_oid as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_oid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_int64 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_int64)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_int32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_int32)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_int8 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_int8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_double as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_double)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_bool as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_bool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_datetime as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_datetime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_timestamp as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_timestamp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_utf8 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_utf8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_doc as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_doc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_binary as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_binary)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_regex as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_regex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_dbpointer as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_dbpointer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_code as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_codewscope as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_codewscope)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_symbol as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_symbol)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_bson_value_t__bindgen_ty_1>())).v_decimal128 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t__bindgen_ty_1),
            "::",
            stringify!(v_decimal128)
        )
    );
}
#[test]
fn bindgen_test_layout__bson_value_t() {
    assert_eq!(
        ::std::mem::size_of::<_bson_value_t>(),
        32usize,
        concat!("Size of: ", stringify!(_bson_value_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_value_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_bson_value_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_value_t>())).value_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t),
            "::",
            stringify!(value_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_value_t>())).padding as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t),
            "::",
            stringify!(padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_value_t>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_value_t),
            "::",
            stringify!(value)
        )
    );
}
pub type bson_value_t = _bson_value_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bson_iter_t {
    pub raw: *const u8,
    pub len: u32,
    pub off: u32,
    pub type_: u32,
    pub key: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
    pub d4: u32,
    pub next_off: u32,
    pub err_off: u32,
    pub value: bson_value_t,
}
#[test]
fn bindgen_test_layout_bson_iter_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_iter_t>(),
        80usize,
        concat!("Size of: ", stringify!(bson_iter_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_iter_t>(),
        8usize,
        concat!("Alignment of ", stringify!(bson_iter_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).raw as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(raw)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).off as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(off)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).type_ as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).key as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).d1 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(d1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).d2 as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(d2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).d3 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(d3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).d4 as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(d4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).next_off as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(next_off)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).err_off as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(err_off)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_iter_t>())).value as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_iter_t),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_reader_t {
    pub type_: u32,
}
#[test]
fn bindgen_test_layout_bson_reader_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_reader_t>(),
        4usize,
        concat!("Size of: ", stringify!(bson_reader_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_reader_t>(),
        4usize,
        concat!("Alignment of ", stringify!(bson_reader_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_reader_t>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_reader_t),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_visitor_t {
    pub visit_before: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_after: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_corrupt: ::std::option::Option<
        unsafe extern "C" fn(iter: *const bson_iter_t, data: *mut ::std::os::raw::c_void),
    >,
    pub visit_double: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_double: f64,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_utf8: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_utf8_len: usize,
            v_utf8: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_document: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_document: *const bson_t,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_array: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_array: *const bson_t,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_binary: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_subtype: bson_subtype_t,
            v_binary_len: usize,
            v_binary: *const u8,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_undefined: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_oid: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_oid: *const bson_oid_t,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_bool: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_bool: bool,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_date_time: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            msec_since_epoch: i64,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_null: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_regex: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_regex: *const ::std::os::raw::c_char,
            v_options: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_dbpointer: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_collection_len: usize,
            v_collection: *const ::std::os::raw::c_char,
            v_oid: *const bson_oid_t,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_code: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_code_len: usize,
            v_code: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_symbol: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_symbol_len: usize,
            v_symbol: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_codewscope: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_code_len: usize,
            v_code: *const ::std::os::raw::c_char,
            v_scope: *const bson_t,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_int32: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_int32: i32,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_timestamp: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_timestamp: u32,
            v_increment: u32,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_int64: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_int64: i64,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_maxkey: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_minkey: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub visit_unsupported_type: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            type_code: u32,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub visit_decimal128: ::std::option::Option<
        unsafe extern "C" fn(
            iter: *const bson_iter_t,
            key: *const ::std::os::raw::c_char,
            v_decimal128: *const bson_decimal128_t,
            data: *mut ::std::os::raw::c_void,
        ) -> bool,
    >,
    pub padding: [*mut ::std::os::raw::c_void; 7usize],
}
#[test]
fn bindgen_test_layout_bson_visitor_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_visitor_t>(),
        256usize,
        concat!("Size of: ", stringify!(bson_visitor_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_visitor_t>(),
        8usize,
        concat!("Alignment of ", stringify!(bson_visitor_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_before as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_before)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_after as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_after)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_corrupt as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_corrupt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_double as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_double)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_utf8 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_utf8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_document as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_document)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_array as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_array)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_binary as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_binary)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_undefined as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_undefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_oid as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_oid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_bool as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_bool)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_date_time as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_date_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_null as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_null)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_regex as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_regex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_dbpointer as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_dbpointer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_code as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_symbol as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_symbol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_codewscope as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_codewscope)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_int32 as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_int32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_timestamp as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_timestamp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_int64 as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_int64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_maxkey as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_maxkey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_minkey as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_minkey)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bson_visitor_t>())).visit_unsupported_type as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_unsupported_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).visit_decimal128 as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(visit_decimal128)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_visitor_t>())).padding as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_visitor_t),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bson_error_t {
    pub domain: u32,
    pub code: u32,
    pub message: [::std::os::raw::c_char; 504usize],
}
#[test]
fn bindgen_test_layout__bson_error_t() {
    assert_eq!(
        ::std::mem::size_of::<_bson_error_t>(),
        512usize,
        concat!("Size of: ", stringify!(_bson_error_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_error_t>(),
        4usize,
        concat!("Alignment of ", stringify!(_bson_error_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_error_t>())).domain as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_error_t),
            "::",
            stringify!(domain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_error_t>())).code as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_error_t),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_error_t>())).message as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_error_t),
            "::",
            stringify!(message)
        )
    );
}
pub type bson_error_t = _bson_error_t;
pub type static_assert_test_525error_t = [::std::os::raw::c_char; 1usize];
extern "C" {
    pub fn bson_context_new(flags: bson_context_flags_t) -> *mut bson_context_t;
}
extern "C" {
    pub fn bson_context_destroy(context: *mut bson_context_t);
}
extern "C" {
    pub fn bson_context_get_default() -> *mut bson_context_t;
}
extern "C" {
    pub fn bson_get_monotonic_time() -> i64;
}
extern "C" {
    pub fn bson_gettimeofday(tv: *mut timeval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_decimal128_to_string(
        dec: *const bson_decimal128_t,
        str: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn bson_decimal128_from_string(
        string: *const ::std::os::raw::c_char,
        dec: *mut bson_decimal128_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_decimal128_from_string_w_len(
        string: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        dec: *mut bson_decimal128_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_set_error(
        error: *mut bson_error_t,
        domain: u32,
        code: u32,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn bson_strerror_r(
        err_code: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_value(iter: *mut bson_iter_t) -> *const bson_value_t;
}
extern "C" {
    pub fn bson_iter_array(iter: *const bson_iter_t, array_len: *mut u32, array: *mut *const u8);
}
extern "C" {
    pub fn bson_iter_binary(
        iter: *const bson_iter_t,
        subtype: *mut bson_subtype_t,
        binary_len: *mut u32,
        binary: *mut *const u8,
    );
}
extern "C" {
    pub fn bson_iter_code(
        iter: *const bson_iter_t,
        length: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_codewscope(
        iter: *const bson_iter_t,
        length: *mut u32,
        scope_len: *mut u32,
        scope: *mut *const u8,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_dbpointer(
        iter: *const bson_iter_t,
        collection_len: *mut u32,
        collection: *mut *const ::std::os::raw::c_char,
        oid: *mut *const bson_oid_t,
    );
}
extern "C" {
    pub fn bson_iter_document(
        iter: *const bson_iter_t,
        document_len: *mut u32,
        document: *mut *const u8,
    );
}
extern "C" {
    pub fn bson_iter_double(iter: *const bson_iter_t) -> f64;
}
extern "C" {
    pub fn bson_iter_as_double(iter: *const bson_iter_t) -> f64;
}
extern "C" {
    pub fn bson_iter_init(iter: *mut bson_iter_t, bson: *const bson_t) -> bool;
}
extern "C" {
    pub fn bson_iter_init_from_data(iter: *mut bson_iter_t, data: *const u8, length: usize)
        -> bool;
}
extern "C" {
    pub fn bson_iter_init_find(
        iter: *mut bson_iter_t,
        bson: *const bson_t,
        key: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_init_find_w_len(
        iter: *mut bson_iter_t,
        bson: *const bson_t,
        key: *const ::std::os::raw::c_char,
        keylen: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_init_find_case(
        iter: *mut bson_iter_t,
        bson: *const bson_t,
        key: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_init_from_data_at_offset(
        iter: *mut bson_iter_t,
        data: *const u8,
        length: usize,
        offset: u32,
        keylen: u32,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_int32(iter: *const bson_iter_t) -> i32;
}
extern "C" {
    pub fn bson_iter_int64(iter: *const bson_iter_t) -> i64;
}
extern "C" {
    pub fn bson_iter_as_int64(iter: *const bson_iter_t) -> i64;
}
extern "C" {
    pub fn bson_iter_find(iter: *mut bson_iter_t, key: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn bson_iter_find_w_len(
        iter: *mut bson_iter_t,
        key: *const ::std::os::raw::c_char,
        keylen: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_find_case(iter: *mut bson_iter_t, key: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn bson_iter_find_descendant(
        iter: *mut bson_iter_t,
        dotkey: *const ::std::os::raw::c_char,
        descendant: *mut bson_iter_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_next(iter: *mut bson_iter_t) -> bool;
}
extern "C" {
    pub fn bson_iter_oid(iter: *const bson_iter_t) -> *const bson_oid_t;
}
extern "C" {
    pub fn bson_iter_decimal128(iter: *const bson_iter_t, dec: *mut bson_decimal128_t) -> bool;
}
extern "C" {
    pub fn bson_iter_key(iter: *const bson_iter_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_key_len(iter: *const bson_iter_t) -> u32;
}
extern "C" {
    pub fn bson_iter_utf8(
        iter: *const bson_iter_t,
        length: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_dup_utf8(
        iter: *const bson_iter_t,
        length: *mut u32,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_date_time(iter: *const bson_iter_t) -> i64;
}
extern "C" {
    pub fn bson_iter_time_t(iter: *const bson_iter_t) -> time_t;
}
extern "C" {
    pub fn bson_iter_timeval(iter: *const bson_iter_t, tv: *mut timeval);
}
extern "C" {
    pub fn bson_iter_timestamp(iter: *const bson_iter_t, timestamp: *mut u32, increment: *mut u32);
}
extern "C" {
    pub fn bson_iter_bool(iter: *const bson_iter_t) -> bool;
}
extern "C" {
    pub fn bson_iter_as_bool(iter: *const bson_iter_t) -> bool;
}
extern "C" {
    pub fn bson_iter_regex(
        iter: *const bson_iter_t,
        options: *mut *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_symbol(
        iter: *const bson_iter_t,
        length: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_iter_type(iter: *const bson_iter_t) -> bson_type_t;
}
extern "C" {
    pub fn bson_iter_recurse(iter: *const bson_iter_t, child: *mut bson_iter_t) -> bool;
}
extern "C" {
    pub fn bson_iter_overwrite_int32(iter: *mut bson_iter_t, value: i32);
}
extern "C" {
    pub fn bson_iter_overwrite_int64(iter: *mut bson_iter_t, value: i64);
}
extern "C" {
    pub fn bson_iter_overwrite_double(iter: *mut bson_iter_t, value: f64);
}
extern "C" {
    pub fn bson_iter_overwrite_decimal128(iter: *mut bson_iter_t, value: *mut bson_decimal128_t);
}
extern "C" {
    pub fn bson_iter_overwrite_bool(iter: *mut bson_iter_t, value: bool);
}
extern "C" {
    pub fn bson_iter_overwrite_oid(iter: *mut bson_iter_t, value: *const bson_oid_t);
}
extern "C" {
    pub fn bson_iter_overwrite_timestamp(iter: *mut bson_iter_t, timestamp: u32, increment: u32);
}
extern "C" {
    pub fn bson_iter_overwrite_date_time(iter: *mut bson_iter_t, value: i64);
}
extern "C" {
    pub fn bson_iter_visit_all(
        iter: *mut bson_iter_t,
        visitor: *const bson_visitor_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn bson_iter_offset(iter: *mut bson_iter_t) -> u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_json_reader_t {
    _unused: [u8; 0],
}
pub type bson_json_reader_t = _bson_json_reader_t;
pub const bson_json_error_code_t_BSON_JSON_ERROR_READ_CORRUPT_JS: bson_json_error_code_t = 1;
pub const bson_json_error_code_t_BSON_JSON_ERROR_READ_INVALID_PARAM: bson_json_error_code_t = 2;
pub const bson_json_error_code_t_BSON_JSON_ERROR_READ_CB_FAILURE: bson_json_error_code_t = 3;
pub type bson_json_error_code_t = u32;
pub type bson_json_reader_cb = ::std::option::Option<
    unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void, buf: *mut u8, count: usize) -> isize,
>;
pub type bson_json_destroy_cb =
    ::std::option::Option<unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn bson_json_reader_new(
        data: *mut ::std::os::raw::c_void,
        cb: bson_json_reader_cb,
        dcb: bson_json_destroy_cb,
        allow_multiple: bool,
        buf_size: usize,
    ) -> *mut bson_json_reader_t;
}
extern "C" {
    pub fn bson_json_reader_new_from_fd(
        fd: ::std::os::raw::c_int,
        close_on_destroy: bool,
    ) -> *mut bson_json_reader_t;
}
extern "C" {
    pub fn bson_json_reader_new_from_file(
        filename: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> *mut bson_json_reader_t;
}
extern "C" {
    pub fn bson_json_reader_destroy(reader: *mut bson_json_reader_t);
}
extern "C" {
    pub fn bson_json_reader_read(
        reader: *mut bson_json_reader_t,
        bson: *mut bson_t,
        error: *mut bson_error_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_json_data_reader_new(allow_multiple: bool, size: usize) -> *mut bson_json_reader_t;
}
extern "C" {
    pub fn bson_json_data_reader_ingest(
        reader: *mut bson_json_reader_t,
        data: *const u8,
        len: usize,
    );
}
extern "C" {
    pub fn bson_uint32_to_string(
        value: u32,
        strptr: *mut *const ::std::os::raw::c_char,
        str: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> usize;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bson_md5_t {
    pub count: [u32; 2usize],
    pub abcd: [u32; 4usize],
    pub buf: [u8; 64usize],
}
#[test]
fn bindgen_test_layout_bson_md5_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_md5_t>(),
        88usize,
        concat!("Size of: ", stringify!(bson_md5_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_md5_t>(),
        4usize,
        concat!("Alignment of ", stringify!(bson_md5_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_md5_t>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_md5_t),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_md5_t>())).abcd as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_md5_t),
            "::",
            stringify!(abcd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_md5_t>())).buf as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_md5_t),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    pub fn bson_md5_init(pms: *mut bson_md5_t);
}
extern "C" {
    pub fn bson_md5_append(pms: *mut bson_md5_t, data: *const u8, nbytes: u32);
}
extern "C" {
    pub fn bson_md5_finish(pms: *mut bson_md5_t, digest: *mut u8);
}
pub type bson_realloc_func = ::std::option::Option<
    unsafe extern "C" fn(
        mem: *mut ::std::os::raw::c_void,
        num_bytes: usize,
        ctx: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_mem_vtable_t {
    pub malloc: ::std::option::Option<
        unsafe extern "C" fn(num_bytes: usize) -> *mut ::std::os::raw::c_void,
    >,
    pub calloc: ::std::option::Option<
        unsafe extern "C" fn(n_members: usize, num_bytes: usize) -> *mut ::std::os::raw::c_void,
    >,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(mem: *mut ::std::os::raw::c_void, num_bytes: usize)
            -> *mut ::std::os::raw::c_void,
    >,
    pub free: ::std::option::Option<unsafe extern "C" fn(mem: *mut ::std::os::raw::c_void)>,
    pub padding: [*mut ::std::os::raw::c_void; 4usize],
}
#[test]
fn bindgen_test_layout__bson_mem_vtable_t() {
    assert_eq!(
        ::std::mem::size_of::<_bson_mem_vtable_t>(),
        64usize,
        concat!("Size of: ", stringify!(_bson_mem_vtable_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_bson_mem_vtable_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_bson_mem_vtable_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_mem_vtable_t>())).malloc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_mem_vtable_t),
            "::",
            stringify!(malloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_mem_vtable_t>())).calloc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_mem_vtable_t),
            "::",
            stringify!(calloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_mem_vtable_t>())).realloc as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_mem_vtable_t),
            "::",
            stringify!(realloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_mem_vtable_t>())).free as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_mem_vtable_t),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bson_mem_vtable_t>())).padding as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_bson_mem_vtable_t),
            "::",
            stringify!(padding)
        )
    );
}
pub type bson_mem_vtable_t = _bson_mem_vtable_t;
extern "C" {
    pub fn bson_mem_set_vtable(vtable: *const bson_mem_vtable_t);
}
extern "C" {
    pub fn bson_mem_restore_vtable();
}
extern "C" {
    pub fn bson_malloc(num_bytes: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn bson_malloc0(num_bytes: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn bson_realloc(
        mem: *mut ::std::os::raw::c_void,
        num_bytes: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn bson_realloc_ctx(
        mem: *mut ::std::os::raw::c_void,
        num_bytes: usize,
        ctx: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn bson_free(mem: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn bson_zero_free(mem: *mut ::std::os::raw::c_void, size: usize);
}
extern "C" {
    pub fn bson_oid_compare(
        oid1: *const bson_oid_t,
        oid2: *const bson_oid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_oid_copy(src: *const bson_oid_t, dst: *mut bson_oid_t);
}
extern "C" {
    pub fn bson_oid_equal(oid1: *const bson_oid_t, oid2: *const bson_oid_t) -> bool;
}
extern "C" {
    pub fn bson_oid_is_valid(str: *const ::std::os::raw::c_char, length: usize) -> bool;
}
extern "C" {
    pub fn bson_oid_get_time_t(oid: *const bson_oid_t) -> time_t;
}
extern "C" {
    pub fn bson_oid_hash(oid: *const bson_oid_t) -> u32;
}
extern "C" {
    pub fn bson_oid_init(oid: *mut bson_oid_t, context: *mut bson_context_t);
}
extern "C" {
    pub fn bson_oid_init_from_data(oid: *mut bson_oid_t, data: *const u8);
}
extern "C" {
    pub fn bson_oid_init_from_string(oid: *mut bson_oid_t, str: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn bson_oid_init_sequence(oid: *mut bson_oid_t, context: *mut bson_context_t);
}
extern "C" {
    pub fn bson_oid_to_string(oid: *const bson_oid_t, str: *mut ::std::os::raw::c_char);
}
pub type bson_reader_read_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        handle: *mut ::std::os::raw::c_void,
        buf: *mut ::std::os::raw::c_void,
        count: usize,
    ) -> isize,
>;
pub type bson_reader_destroy_func_t =
    ::std::option::Option<unsafe extern "C" fn(handle: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn bson_reader_new_from_handle(
        handle: *mut ::std::os::raw::c_void,
        rf: bson_reader_read_func_t,
        df: bson_reader_destroy_func_t,
    ) -> *mut bson_reader_t;
}
extern "C" {
    pub fn bson_reader_new_from_fd(
        fd: ::std::os::raw::c_int,
        close_on_destroy: bool,
    ) -> *mut bson_reader_t;
}
extern "C" {
    pub fn bson_reader_new_from_file(
        path: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> *mut bson_reader_t;
}
extern "C" {
    pub fn bson_reader_new_from_data(data: *const u8, length: usize) -> *mut bson_reader_t;
}
extern "C" {
    pub fn bson_reader_destroy(reader: *mut bson_reader_t);
}
extern "C" {
    pub fn bson_reader_set_read_func(reader: *mut bson_reader_t, func: bson_reader_read_func_t);
}
extern "C" {
    pub fn bson_reader_set_destroy_func(
        reader: *mut bson_reader_t,
        func: bson_reader_destroy_func_t,
    );
}
extern "C" {
    pub fn bson_reader_read(reader: *mut bson_reader_t, reached_eof: *mut bool) -> *const bson_t;
}
extern "C" {
    pub fn bson_reader_tell(reader: *mut bson_reader_t) -> off_t;
}
extern "C" {
    pub fn bson_reader_reset(reader: *mut bson_reader_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_string_t {
    pub str: *mut ::std::os::raw::c_char,
    pub len: u32,
    pub alloc: u32,
}
#[test]
fn bindgen_test_layout_bson_string_t() {
    assert_eq!(
        ::std::mem::size_of::<bson_string_t>(),
        16usize,
        concat!("Size of: ", stringify!(bson_string_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bson_string_t>(),
        8usize,
        concat!("Alignment of ", stringify!(bson_string_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_string_t>())).str as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_string_t),
            "::",
            stringify!(str)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_string_t>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_string_t),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bson_string_t>())).alloc as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bson_string_t),
            "::",
            stringify!(alloc)
        )
    );
}
extern "C" {
    pub fn bson_string_new(str: *const ::std::os::raw::c_char) -> *mut bson_string_t;
}
extern "C" {
    pub fn bson_string_free(
        string: *mut bson_string_t,
        free_segment: bool,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_string_append(string: *mut bson_string_t, str: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn bson_string_append_c(string: *mut bson_string_t, str: ::std::os::raw::c_char);
}
extern "C" {
    pub fn bson_string_append_unichar(string: *mut bson_string_t, unichar: bson_unichar_t);
}
extern "C" {
    pub fn bson_string_append_printf(
        string: *mut bson_string_t,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn bson_string_truncate(string: *mut bson_string_t, len: u32);
}
extern "C" {
    pub fn bson_strdup(str: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_strdup_printf(
        format: *const ::std::os::raw::c_char,
        ...
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_strdupv_printf(
        format: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_strndup(
        str: *const ::std::os::raw::c_char,
        n_bytes: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_strncpy(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
        size: usize,
    );
}
extern "C" {
    pub fn bson_vsnprintf(
        str: *mut ::std::os::raw::c_char,
        size: usize,
        format: *const ::std::os::raw::c_char,
        ap: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_snprintf(
        str: *mut ::std::os::raw::c_char,
        size: usize,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_strfreev(strv: *mut *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn bson_strnlen(s: *const ::std::os::raw::c_char, maxlen: usize) -> usize;
}
extern "C" {
    pub fn bson_ascii_strtoll(
        str: *const ::std::os::raw::c_char,
        endptr: *mut *mut ::std::os::raw::c_char,
        base: ::std::os::raw::c_int,
    ) -> i64;
}
extern "C" {
    pub fn bson_strcasecmp(
        s1: *const ::std::os::raw::c_char,
        s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_utf8_validate(
        utf8: *const ::std::os::raw::c_char,
        utf8_len: usize,
        allow_null: bool,
    ) -> bool;
}
extern "C" {
    pub fn bson_utf8_escape_for_json(
        utf8: *const ::std::os::raw::c_char,
        utf8_len: isize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_utf8_get_char(utf8: *const ::std::os::raw::c_char) -> bson_unichar_t;
}
extern "C" {
    pub fn bson_utf8_next_char(
        utf8: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_utf8_from_unichar(
        unichar: bson_unichar_t,
        utf8: *mut ::std::os::raw::c_char,
        len: *mut u32,
    );
}
extern "C" {
    pub fn bson_value_copy(src: *const bson_value_t, dst: *mut bson_value_t);
}
extern "C" {
    pub fn bson_value_destroy(value: *mut bson_value_t);
}
extern "C" {
    pub fn bson_get_major_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_get_minor_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_get_micro_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_get_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_check_version(
        required_major: ::std::os::raw::c_int,
        required_minor: ::std::os::raw::c_int,
        required_micro: ::std::os::raw::c_int,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_writer_t {
    _unused: [u8; 0],
}
pub type bson_writer_t = _bson_writer_t;
extern "C" {
    pub fn bson_writer_new(
        buf: *mut *mut u8,
        buflen: *mut usize,
        offset: usize,
        realloc_func: bson_realloc_func,
        realloc_func_ctx: *mut ::std::os::raw::c_void,
    ) -> *mut bson_writer_t;
}
extern "C" {
    pub fn bson_writer_destroy(writer: *mut bson_writer_t);
}
extern "C" {
    pub fn bson_writer_get_length(writer: *mut bson_writer_t) -> usize;
}
extern "C" {
    pub fn bson_writer_begin(writer: *mut bson_writer_t, bson: *mut *mut bson_t) -> bool;
}
extern "C" {
    pub fn bson_writer_end(writer: *mut bson_writer_t);
}
extern "C" {
    pub fn bson_writer_rollback(writer: *mut bson_writer_t);
}
pub const bcon_type_t_BCON_TYPE_UTF8: bcon_type_t = 0;
pub const bcon_type_t_BCON_TYPE_DOUBLE: bcon_type_t = 1;
pub const bcon_type_t_BCON_TYPE_DOCUMENT: bcon_type_t = 2;
pub const bcon_type_t_BCON_TYPE_ARRAY: bcon_type_t = 3;
pub const bcon_type_t_BCON_TYPE_BIN: bcon_type_t = 4;
pub const bcon_type_t_BCON_TYPE_UNDEFINED: bcon_type_t = 5;
pub const bcon_type_t_BCON_TYPE_OID: bcon_type_t = 6;
pub const bcon_type_t_BCON_TYPE_BOOL: bcon_type_t = 7;
pub const bcon_type_t_BCON_TYPE_DATE_TIME: bcon_type_t = 8;
pub const bcon_type_t_BCON_TYPE_NULL: bcon_type_t = 9;
pub const bcon_type_t_BCON_TYPE_REGEX: bcon_type_t = 10;
pub const bcon_type_t_BCON_TYPE_DBPOINTER: bcon_type_t = 11;
pub const bcon_type_t_BCON_TYPE_CODE: bcon_type_t = 12;
pub const bcon_type_t_BCON_TYPE_SYMBOL: bcon_type_t = 13;
pub const bcon_type_t_BCON_TYPE_CODEWSCOPE: bcon_type_t = 14;
pub const bcon_type_t_BCON_TYPE_INT32: bcon_type_t = 15;
pub const bcon_type_t_BCON_TYPE_TIMESTAMP: bcon_type_t = 16;
pub const bcon_type_t_BCON_TYPE_INT64: bcon_type_t = 17;
pub const bcon_type_t_BCON_TYPE_DECIMAL128: bcon_type_t = 18;
pub const bcon_type_t_BCON_TYPE_MAXKEY: bcon_type_t = 19;
pub const bcon_type_t_BCON_TYPE_MINKEY: bcon_type_t = 20;
pub const bcon_type_t_BCON_TYPE_BCON: bcon_type_t = 21;
pub const bcon_type_t_BCON_TYPE_ARRAY_START: bcon_type_t = 22;
pub const bcon_type_t_BCON_TYPE_ARRAY_END: bcon_type_t = 23;
pub const bcon_type_t_BCON_TYPE_DOC_START: bcon_type_t = 24;
pub const bcon_type_t_BCON_TYPE_DOC_END: bcon_type_t = 25;
pub const bcon_type_t_BCON_TYPE_END: bcon_type_t = 26;
pub const bcon_type_t_BCON_TYPE_RAW: bcon_type_t = 27;
pub const bcon_type_t_BCON_TYPE_SKIP: bcon_type_t = 28;
pub const bcon_type_t_BCON_TYPE_ITER: bcon_type_t = 29;
pub const bcon_type_t_BCON_TYPE_ERROR: bcon_type_t = 30;
pub type bcon_type_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcon_append_ctx_frame {
    pub i: ::std::os::raw::c_int,
    pub is_array: bool,
    pub __bindgen_padding_0: [u64; 15usize],
    pub bson: bson_t,
}
#[test]
fn bindgen_test_layout_bcon_append_ctx_frame() {
    assert_eq!(
        ::std::mem::size_of::<bcon_append_ctx_frame>(),
        256usize,
        concat!("Size of: ", stringify!(bcon_append_ctx_frame))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcon_append_ctx_frame>())).i as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bcon_append_ctx_frame),
            "::",
            stringify!(i)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcon_append_ctx_frame>())).is_array as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bcon_append_ctx_frame),
            "::",
            stringify!(is_array)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcon_append_ctx_frame>())).bson as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(bcon_append_ctx_frame),
            "::",
            stringify!(bson)
        )
    );
}
pub type bcon_append_ctx_frame_t = bcon_append_ctx_frame;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcon_extract_ctx_frame {
    pub i: ::std::os::raw::c_int,
    pub is_array: bool,
    pub __bindgen_padding_0: [u64; 15usize],
    pub iter: bson_iter_t,
    pub __bindgen_padding_1: [u64; 6usize],
}
#[test]
fn bindgen_test_layout_bcon_extract_ctx_frame() {
    assert_eq!(
        ::std::mem::size_of::<bcon_extract_ctx_frame>(),
        256usize,
        concat!("Size of: ", stringify!(bcon_extract_ctx_frame))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcon_extract_ctx_frame>())).i as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bcon_extract_ctx_frame),
            "::",
            stringify!(i)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcon_extract_ctx_frame>())).is_array as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bcon_extract_ctx_frame),
            "::",
            stringify!(is_array)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bcon_extract_ctx_frame>())).iter as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(bcon_extract_ctx_frame),
            "::",
            stringify!(iter)
        )
    );
}
pub type bcon_extract_ctx_frame_t = bcon_extract_ctx_frame;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bcon_append_ctx_t {
    pub stack: [bcon_append_ctx_frame_t; 100usize],
    pub n: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u32; 31usize],
}
#[test]
fn bindgen_test_layout__bcon_append_ctx_t() {
    assert_eq!(
        ::std::mem::size_of::<_bcon_append_ctx_t>(),
        25728usize,
        concat!("Size of: ", stringify!(_bcon_append_ctx_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bcon_append_ctx_t>())).stack as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bcon_append_ctx_t),
            "::",
            stringify!(stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bcon_append_ctx_t>())).n as *const _ as usize },
        25600usize,
        concat!(
            "Offset of field: ",
            stringify!(_bcon_append_ctx_t),
            "::",
            stringify!(n)
        )
    );
}
pub type bcon_append_ctx_t = _bcon_append_ctx_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bcon_extract_ctx_t {
    pub stack: [bcon_extract_ctx_frame_t; 100usize],
    pub n: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u32; 31usize],
}
#[test]
fn bindgen_test_layout__bcon_extract_ctx_t() {
    assert_eq!(
        ::std::mem::size_of::<_bcon_extract_ctx_t>(),
        25728usize,
        concat!("Size of: ", stringify!(_bcon_extract_ctx_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bcon_extract_ctx_t>())).stack as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bcon_extract_ctx_t),
            "::",
            stringify!(stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_bcon_extract_ctx_t>())).n as *const _ as usize },
        25600usize,
        concat!(
            "Offset of field: ",
            stringify!(_bcon_extract_ctx_t),
            "::",
            stringify!(n)
        )
    );
}
pub type bcon_extract_ctx_t = _bcon_extract_ctx_t;
extern "C" {
    pub fn bcon_append(bson: *mut bson_t, ...);
}
extern "C" {
    pub fn bcon_append_ctx(bson: *mut bson_t, ctx: *mut bcon_append_ctx_t, ...);
}
extern "C" {
    pub fn bcon_append_ctx_va(bson: *mut bson_t, ctx: *mut bcon_append_ctx_t, va: *mut va_list);
}
extern "C" {
    pub fn bcon_append_ctx_init(ctx: *mut bcon_append_ctx_t);
}
extern "C" {
    pub fn bcon_extract_ctx_init(ctx: *mut bcon_extract_ctx_t);
}
extern "C" {
    pub fn bcon_extract_ctx(bson: *mut bson_t, ctx: *mut bcon_extract_ctx_t, ...);
}
extern "C" {
    pub fn bcon_extract_ctx_va(
        bson: *mut bson_t,
        ctx: *mut bcon_extract_ctx_t,
        ap: *mut va_list,
    ) -> bool;
}
extern "C" {
    pub fn bcon_extract(bson: *mut bson_t, ...) -> bool;
}
extern "C" {
    pub fn bcon_extract_va(bson: *mut bson_t, ctx: *mut bcon_extract_ctx_t, ...) -> bool;
}
extern "C" {
    pub fn bcon_new(unused: *mut ::std::os::raw::c_void, ...) -> *mut bson_t;
}
extern "C" {
    pub fn bson_bcon_magic() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_bcone_magic() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_new() -> *mut bson_t;
}
extern "C" {
    pub fn bson_new_from_json(data: *const u8, len: isize, error: *mut bson_error_t)
        -> *mut bson_t;
}
extern "C" {
    pub fn bson_init_from_json(
        bson: *mut bson_t,
        data: *const ::std::os::raw::c_char,
        len: isize,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_init_static(b: *mut bson_t, data: *const u8, length: usize) -> bool;
}
extern "C" {
    pub fn bson_init(b: *mut bson_t);
}
extern "C" {
    pub fn bson_reinit(b: *mut bson_t);
}
extern "C" {
    pub fn bson_new_from_data(data: *const u8, length: usize) -> *mut bson_t;
}
extern "C" {
    pub fn bson_new_from_buffer(
        buf: *mut *mut u8,
        buf_len: *mut usize,
        realloc_func: bson_realloc_func,
        realloc_func_ctx: *mut ::std::os::raw::c_void,
    ) -> *mut bson_t;
}
extern "C" {
    pub fn bson_sized_new(size: usize) -> *mut bson_t;
}
extern "C" {
    pub fn bson_copy(bson: *const bson_t) -> *mut bson_t;
}
extern "C" {
    pub fn bson_copy_to(src: *const bson_t, dst: *mut bson_t);
}
extern "C" {
    pub fn bson_copy_to_excluding(
        src: *const bson_t,
        dst: *mut bson_t,
        first_exclude: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn bson_copy_to_excluding_noinit(
        src: *const bson_t,
        dst: *mut bson_t,
        first_exclude: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn bson_destroy(bson: *mut bson_t);
}
extern "C" {
    pub fn bson_reserve_buffer(bson: *mut bson_t, size: u32) -> *mut u8;
}
extern "C" {
    pub fn bson_steal(dst: *mut bson_t, src: *mut bson_t) -> bool;
}
extern "C" {
    pub fn bson_destroy_with_steal(bson: *mut bson_t, steal: bool, length: *mut u32) -> *mut u8;
}
extern "C" {
    pub fn bson_get_data(bson: *const bson_t) -> *const u8;
}
extern "C" {
    pub fn bson_count_keys(bson: *const bson_t) -> u32;
}
extern "C" {
    pub fn bson_has_field(bson: *const bson_t, key: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn bson_compare(bson: *const bson_t, other: *const bson_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bson_equal(bson: *const bson_t, other: *const bson_t) -> bool;
}
extern "C" {
    pub fn bson_validate(
        bson: *const bson_t,
        flags: bson_validate_flags_t,
        offset: *mut usize,
    ) -> bool;
}
extern "C" {
    pub fn bson_validate_with_error(
        bson: *const bson_t,
        flags: bson_validate_flags_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_as_canonical_extended_json(
        bson: *const bson_t,
        length: *mut usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_as_json(bson: *const bson_t, length: *mut usize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_as_relaxed_extended_json(
        bson: *const bson_t,
        length: *mut usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_array_as_json(
        bson: *const bson_t,
        length: *mut usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bson_append_value(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: *const bson_value_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_array(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        array: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_binary(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        subtype: bson_subtype_t,
        binary: *const u8,
        length: u32,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_bool(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: bool,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_code(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        javascript: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_code_with_scope(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        javascript: *const ::std::os::raw::c_char,
        scope: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_dbpointer(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        collection: *const ::std::os::raw::c_char,
        oid: *const bson_oid_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_double(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: f64,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_document(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_document_begin(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        child: *mut bson_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_document_end(bson: *mut bson_t, child: *mut bson_t) -> bool;
}
extern "C" {
    pub fn bson_append_array_begin(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        child: *mut bson_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_array_end(bson: *mut bson_t, child: *mut bson_t) -> bool;
}
extern "C" {
    pub fn bson_append_int32(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: i32,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_int64(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: i64,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_decimal128(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: *const bson_decimal128_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_iter(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        iter: *const bson_iter_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_minkey(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_maxkey(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_null(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_oid(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        oid: *const bson_oid_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_regex(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        regex: *const ::std::os::raw::c_char,
        options: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_regex_w_len(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        regex: *const ::std::os::raw::c_char,
        regex_length: ::std::os::raw::c_int,
        options: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_utf8(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_symbol(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_time_t(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: time_t,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_timeval(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: *mut timeval,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_date_time(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        value: i64,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_now_utc(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_timestamp(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
        timestamp: u32,
        increment: u32,
    ) -> bool;
}
extern "C" {
    pub fn bson_append_undefined(
        bson: *mut bson_t,
        key: *const ::std::os::raw::c_char,
        key_length: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn bson_concat(dst: *mut bson_t, src: *const bson_t) -> bool;
}
pub type mongoc_host_list_t = _mongoc_host_list_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _mongoc_host_list_t {
    pub next: *mut mongoc_host_list_t,
    pub host: [::std::os::raw::c_char; 256usize],
    pub host_and_port: [::std::os::raw::c_char; 262usize],
    pub port: u16,
    pub family: ::std::os::raw::c_int,
    pub padding: [*mut ::std::os::raw::c_void; 4usize],
}
#[test]
fn bindgen_test_layout__mongoc_host_list_t() {
    assert_eq!(
        ::std::mem::size_of::<_mongoc_host_list_t>(),
        568usize,
        concat!("Size of: ", stringify!(_mongoc_host_list_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_mongoc_host_list_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_mongoc_host_list_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_host_list_t>())).next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_host_list_t),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_host_list_t>())).host as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_host_list_t),
            "::",
            stringify!(host)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_host_list_t>())).host_and_port as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_host_list_t),
            "::",
            stringify!(host_and_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_host_list_t>())).port as *const _ as usize },
        526usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_host_list_t),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_host_list_t>())).family as *const _ as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_host_list_t),
            "::",
            stringify!(family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_host_list_t>())).padding as *const _ as usize },
        536usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_host_list_t),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_read_prefs_t {
    _unused: [u8; 0],
}
pub type mongoc_read_prefs_t = _mongoc_read_prefs_t;
pub const mongoc_read_mode_t_MONGOC_READ_PRIMARY: mongoc_read_mode_t = 1;
pub const mongoc_read_mode_t_MONGOC_READ_SECONDARY: mongoc_read_mode_t = 2;
pub const mongoc_read_mode_t_MONGOC_READ_PRIMARY_PREFERRED: mongoc_read_mode_t = 5;
pub const mongoc_read_mode_t_MONGOC_READ_SECONDARY_PREFERRED: mongoc_read_mode_t = 6;
pub const mongoc_read_mode_t_MONGOC_READ_NEAREST: mongoc_read_mode_t = 10;
pub type mongoc_read_mode_t = u32;
extern "C" {
    pub fn mongoc_read_prefs_new(read_mode: mongoc_read_mode_t) -> *mut mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_read_prefs_copy(
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_read_prefs_destroy(read_prefs: *mut mongoc_read_prefs_t);
}
extern "C" {
    pub fn mongoc_read_prefs_get_mode(read_prefs: *const mongoc_read_prefs_t)
        -> mongoc_read_mode_t;
}
extern "C" {
    pub fn mongoc_read_prefs_set_mode(
        read_prefs: *mut mongoc_read_prefs_t,
        mode: mongoc_read_mode_t,
    );
}
extern "C" {
    pub fn mongoc_read_prefs_get_tags(read_prefs: *const mongoc_read_prefs_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_read_prefs_set_tags(read_prefs: *mut mongoc_read_prefs_t, tags: *const bson_t);
}
extern "C" {
    pub fn mongoc_read_prefs_add_tag(read_prefs: *mut mongoc_read_prefs_t, tag: *const bson_t);
}
extern "C" {
    pub fn mongoc_read_prefs_get_max_staleness_seconds(
        read_prefs: *const mongoc_read_prefs_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_read_prefs_set_max_staleness_seconds(
        read_prefs: *mut mongoc_read_prefs_t,
        max_staleness_seconds: i64,
    );
}
extern "C" {
    pub fn mongoc_read_prefs_is_valid(read_prefs: *const mongoc_read_prefs_t) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_server_description_t {
    _unused: [u8; 0],
}
pub type mongoc_server_description_t = _mongoc_server_description_t;
extern "C" {
    pub fn mongoc_server_description_destroy(description: *mut mongoc_server_description_t);
}
extern "C" {
    pub fn mongoc_server_description_new_copy(
        description: *const mongoc_server_description_t,
    ) -> *mut mongoc_server_description_t;
}
extern "C" {
    pub fn mongoc_server_description_id(description: *const mongoc_server_description_t) -> u32;
}
extern "C" {
    pub fn mongoc_server_description_host(
        description: *const mongoc_server_description_t,
    ) -> *mut mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_server_description_round_trip_time(
        description: *const mongoc_server_description_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_server_description_type(
        description: *const mongoc_server_description_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_server_description_ismaster(
        description: *const mongoc_server_description_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_server_description_compressor_id(
        description: *const mongoc_server_description_t,
    ) -> i32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_topology_description_t {
    _unused: [u8; 0],
}
pub type mongoc_topology_description_t = _mongoc_topology_description_t;
extern "C" {
    pub fn mongoc_topology_description_has_readable_server(
        td: *mut mongoc_topology_description_t,
        prefs: *const mongoc_read_prefs_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_topology_description_has_writable_server(
        td: *mut mongoc_topology_description_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_topology_description_type(
        td: *const mongoc_topology_description_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_topology_description_get_servers(
        td: *const mongoc_topology_description_t,
        n: *mut usize,
    ) -> *mut *mut mongoc_server_description_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_callbacks_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_callbacks_t = _mongoc_apm_callbacks_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_command_started_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_command_started_t = _mongoc_apm_command_started_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_command_succeeded_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_command_succeeded_t = _mongoc_apm_command_succeeded_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_command_failed_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_command_failed_t = _mongoc_apm_command_failed_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_server_changed_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_server_changed_t = _mongoc_apm_server_changed_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_server_opening_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_server_opening_t = _mongoc_apm_server_opening_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_server_closed_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_server_closed_t = _mongoc_apm_server_closed_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_topology_changed_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_topology_changed_t = _mongoc_apm_topology_changed_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_topology_opening_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_topology_opening_t = _mongoc_apm_topology_opening_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_topology_closed_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_topology_closed_t = _mongoc_apm_topology_closed_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_server_heartbeat_started_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_server_heartbeat_started_t = _mongoc_apm_server_heartbeat_started_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_server_heartbeat_succeeded_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_server_heartbeat_succeeded_t = _mongoc_apm_server_heartbeat_succeeded_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_apm_server_heartbeat_failed_t {
    _unused: [u8; 0],
}
pub type mongoc_apm_server_heartbeat_failed_t = _mongoc_apm_server_heartbeat_failed_t;
extern "C" {
    pub fn mongoc_apm_command_started_get_command(
        event: *const mongoc_apm_command_started_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_database_name(
        event: *const mongoc_apm_command_started_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_command_name(
        event: *const mongoc_apm_command_started_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_request_id(
        event: *const mongoc_apm_command_started_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_operation_id(
        event: *const mongoc_apm_command_started_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_host(
        event: *const mongoc_apm_command_started_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_server_id(
        event: *const mongoc_apm_command_started_t,
    ) -> u32;
}
extern "C" {
    pub fn mongoc_apm_command_started_get_context(
        event: *const mongoc_apm_command_started_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_duration(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_reply(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_command_name(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_request_id(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_operation_id(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_host(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_server_id(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> u32;
}
extern "C" {
    pub fn mongoc_apm_command_succeeded_get_context(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_duration(event: *const mongoc_apm_command_failed_t)
        -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_command_name(
        event: *const mongoc_apm_command_failed_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_error(
        event: *const mongoc_apm_command_failed_t,
        error: *mut bson_error_t,
    );
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_reply(
        event: *const mongoc_apm_command_failed_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_request_id(
        event: *const mongoc_apm_command_failed_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_operation_id(
        event: *const mongoc_apm_command_failed_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_host(
        event: *const mongoc_apm_command_failed_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_server_id(
        event: *const mongoc_apm_command_failed_t,
    ) -> u32;
}
extern "C" {
    pub fn mongoc_apm_command_failed_get_context(
        event: *const mongoc_apm_command_failed_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_server_changed_get_host(
        event: *const mongoc_apm_server_changed_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_server_changed_get_topology_id(
        event: *const mongoc_apm_server_changed_t,
        topology_id: *mut bson_oid_t,
    );
}
extern "C" {
    pub fn mongoc_apm_server_changed_get_previous_description(
        event: *const mongoc_apm_server_changed_t,
    ) -> *const mongoc_server_description_t;
}
extern "C" {
    pub fn mongoc_apm_server_changed_get_new_description(
        event: *const mongoc_apm_server_changed_t,
    ) -> *const mongoc_server_description_t;
}
extern "C" {
    pub fn mongoc_apm_server_changed_get_context(
        event: *const mongoc_apm_server_changed_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_server_opening_get_host(
        event: *const mongoc_apm_server_opening_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_server_opening_get_topology_id(
        event: *const mongoc_apm_server_opening_t,
        topology_id: *mut bson_oid_t,
    );
}
extern "C" {
    pub fn mongoc_apm_server_opening_get_context(
        event: *const mongoc_apm_server_opening_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_server_closed_get_host(
        event: *const mongoc_apm_server_closed_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_server_closed_get_topology_id(
        event: *const mongoc_apm_server_closed_t,
        topology_id: *mut bson_oid_t,
    );
}
extern "C" {
    pub fn mongoc_apm_server_closed_get_context(
        event: *const mongoc_apm_server_closed_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_topology_changed_get_topology_id(
        event: *const mongoc_apm_topology_changed_t,
        topology_id: *mut bson_oid_t,
    );
}
extern "C" {
    pub fn mongoc_apm_topology_changed_get_previous_description(
        event: *const mongoc_apm_topology_changed_t,
    ) -> *const mongoc_topology_description_t;
}
extern "C" {
    pub fn mongoc_apm_topology_changed_get_new_description(
        event: *const mongoc_apm_topology_changed_t,
    ) -> *const mongoc_topology_description_t;
}
extern "C" {
    pub fn mongoc_apm_topology_changed_get_context(
        event: *const mongoc_apm_topology_changed_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_topology_opening_get_topology_id(
        event: *const mongoc_apm_topology_opening_t,
        topology_id: *mut bson_oid_t,
    );
}
extern "C" {
    pub fn mongoc_apm_topology_opening_get_context(
        event: *const mongoc_apm_topology_opening_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_topology_closed_get_topology_id(
        event: *const mongoc_apm_topology_closed_t,
        topology_id: *mut bson_oid_t,
    );
}
extern "C" {
    pub fn mongoc_apm_topology_closed_get_context(
        event: *const mongoc_apm_topology_closed_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_started_get_host(
        event: *const mongoc_apm_server_heartbeat_started_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_started_get_context(
        event: *const mongoc_apm_server_heartbeat_started_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_succeeded_get_duration(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_succeeded_get_reply(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_succeeded_get_host(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_succeeded_get_context(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_failed_get_duration(
        event: *const mongoc_apm_server_heartbeat_failed_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_failed_get_error(
        event: *const mongoc_apm_server_heartbeat_failed_t,
        error: *mut bson_error_t,
    );
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_failed_get_host(
        event: *const mongoc_apm_server_heartbeat_failed_t,
    ) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_failed_get_context(
        event: *const mongoc_apm_server_heartbeat_failed_t,
    ) -> *mut ::std::os::raw::c_void;
}
pub type mongoc_apm_command_started_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_command_started_t)>;
pub type mongoc_apm_command_succeeded_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_command_succeeded_t)>;
pub type mongoc_apm_command_failed_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_command_failed_t)>;
pub type mongoc_apm_server_changed_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_server_changed_t)>;
pub type mongoc_apm_server_opening_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_server_opening_t)>;
pub type mongoc_apm_server_closed_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_server_closed_t)>;
pub type mongoc_apm_topology_changed_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_topology_changed_t)>;
pub type mongoc_apm_topology_opening_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_topology_opening_t)>;
pub type mongoc_apm_topology_closed_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_topology_closed_t)>;
pub type mongoc_apm_server_heartbeat_started_cb_t = ::std::option::Option<
    unsafe extern "C" fn(event: *const mongoc_apm_server_heartbeat_started_t),
>;
pub type mongoc_apm_server_heartbeat_succeeded_cb_t = ::std::option::Option<
    unsafe extern "C" fn(event: *const mongoc_apm_server_heartbeat_succeeded_t),
>;
pub type mongoc_apm_server_heartbeat_failed_cb_t =
    ::std::option::Option<unsafe extern "C" fn(event: *const mongoc_apm_server_heartbeat_failed_t)>;
extern "C" {
    pub fn mongoc_apm_callbacks_new() -> *mut mongoc_apm_callbacks_t;
}
extern "C" {
    pub fn mongoc_apm_callbacks_destroy(callbacks: *mut mongoc_apm_callbacks_t);
}
extern "C" {
    pub fn mongoc_apm_set_command_started_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_command_started_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_command_succeeded_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_command_succeeded_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_command_failed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_command_failed_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_server_changed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_changed_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_server_opening_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_opening_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_server_closed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_closed_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_topology_changed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_topology_changed_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_topology_opening_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_topology_opening_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_topology_closed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_topology_closed_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_server_heartbeat_started_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_heartbeat_started_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_server_heartbeat_succeeded_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_heartbeat_succeeded_cb_t,
    );
}
extern "C" {
    pub fn mongoc_apm_set_server_heartbeat_failed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_heartbeat_failed_cb_t,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_write_concern_t {
    _unused: [u8; 0],
}
pub type mongoc_write_concern_t = _mongoc_write_concern_t;
extern "C" {
    pub fn mongoc_write_concern_new() -> *mut mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_write_concern_copy(
        write_concern: *const mongoc_write_concern_t,
    ) -> *mut mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_write_concern_destroy(write_concern: *mut mongoc_write_concern_t);
}
extern "C" {
    pub fn mongoc_write_concern_get_fsync(write_concern: *const mongoc_write_concern_t) -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_set_fsync(write_concern: *mut mongoc_write_concern_t, fsync_: bool);
}
extern "C" {
    pub fn mongoc_write_concern_get_journal(write_concern: *const mongoc_write_concern_t) -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_journal_is_set(
        write_concern: *const mongoc_write_concern_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_set_journal(
        write_concern: *mut mongoc_write_concern_t,
        journal: bool,
    );
}
extern "C" {
    pub fn mongoc_write_concern_get_w(write_concern: *const mongoc_write_concern_t) -> i32;
}
extern "C" {
    pub fn mongoc_write_concern_set_w(write_concern: *mut mongoc_write_concern_t, w: i32);
}
extern "C" {
    pub fn mongoc_write_concern_get_wtag(
        write_concern: *const mongoc_write_concern_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_write_concern_set_wtag(
        write_concern: *mut mongoc_write_concern_t,
        tag: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mongoc_write_concern_get_wtimeout(write_concern: *const mongoc_write_concern_t) -> i32;
}
extern "C" {
    pub fn mongoc_write_concern_set_wtimeout(
        write_concern: *mut mongoc_write_concern_t,
        wtimeout_msec: i32,
    );
}
extern "C" {
    pub fn mongoc_write_concern_get_wmajority(write_concern: *const mongoc_write_concern_t)
        -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_set_wmajority(
        write_concern: *mut mongoc_write_concern_t,
        wtimeout_msec: i32,
    );
}
extern "C" {
    pub fn mongoc_write_concern_is_acknowledged(
        write_concern: *const mongoc_write_concern_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_is_valid(write_concern: *const mongoc_write_concern_t) -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_append(
        write_concern: *mut mongoc_write_concern_t,
        doc: *mut bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_write_concern_is_default(write_concern: *const mongoc_write_concern_t) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_client_session_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_bulk_operation_t {
    _unused: [u8; 0],
}
pub type mongoc_bulk_operation_t = _mongoc_bulk_operation_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_bulk_write_flags_t {
    _unused: [u8; 0],
}
pub type mongoc_bulk_write_flags_t = _mongoc_bulk_write_flags_t;
extern "C" {
    pub fn mongoc_bulk_operation_destroy(bulk: *mut mongoc_bulk_operation_t);
}
extern "C" {
    pub fn mongoc_bulk_operation_execute(
        bulk: *mut mongoc_bulk_operation_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> u32;
}
extern "C" {
    pub fn mongoc_bulk_operation_delete(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_delete_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_insert(
        bulk: *mut mongoc_bulk_operation_t,
        document: *const bson_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_insert_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_bulk_operation_remove(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_remove_many_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_bulk_operation_remove_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_remove_one_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_bulk_operation_replace_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        upsert: bool,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_replace_one_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_bulk_operation_update(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        upsert: bool,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_update_many_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_bulk_operation_update_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        upsert: bool,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_update_one_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_bulk_operation_set_bypass_document_validation(
        bulk: *mut mongoc_bulk_operation_t,
        bypass: bool,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_new(ordered: bool) -> *mut mongoc_bulk_operation_t;
}
extern "C" {
    pub fn mongoc_bulk_operation_set_write_concern(
        bulk: *mut mongoc_bulk_operation_t,
        write_concern: *const mongoc_write_concern_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_set_database(
        bulk: *mut mongoc_bulk_operation_t,
        database: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_set_collection(
        bulk: *mut mongoc_bulk_operation_t,
        collection: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_set_client(
        bulk: *mut mongoc_bulk_operation_t,
        client: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_set_client_session(
        bulk: *mut mongoc_bulk_operation_t,
        client_session: *mut _mongoc_client_session_t,
    );
}
extern "C" {
    pub fn mongoc_bulk_operation_set_hint(bulk: *mut mongoc_bulk_operation_t, server_id: u32);
}
extern "C" {
    pub fn mongoc_bulk_operation_get_hint(bulk: *const mongoc_bulk_operation_t) -> u32;
}
extern "C" {
    pub fn mongoc_bulk_operation_get_write_concern(
        bulk: *const mongoc_bulk_operation_t,
    ) -> *const mongoc_write_concern_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_change_stream_t {
    _unused: [u8; 0],
}
pub type mongoc_change_stream_t = _mongoc_change_stream_t;
extern "C" {
    pub fn mongoc_change_stream_destroy(arg1: *mut mongoc_change_stream_t);
}
extern "C" {
    pub fn mongoc_change_stream_next(
        arg1: *mut mongoc_change_stream_t,
        arg2: *mut *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_change_stream_error_document(
        arg1: *const mongoc_change_stream_t,
        arg2: *mut bson_error_t,
        arg3: *mut *const bson_t,
    ) -> bool;
}
pub const mongoc_delete_flags_t_MONGOC_DELETE_NONE: mongoc_delete_flags_t = 0;
pub const mongoc_delete_flags_t_MONGOC_DELETE_SINGLE_REMOVE: mongoc_delete_flags_t = 1;
pub type mongoc_delete_flags_t = u32;
pub const mongoc_remove_flags_t_MONGOC_REMOVE_NONE: mongoc_remove_flags_t = 0;
pub const mongoc_remove_flags_t_MONGOC_REMOVE_SINGLE_REMOVE: mongoc_remove_flags_t = 1;
pub type mongoc_remove_flags_t = u32;
pub const mongoc_insert_flags_t_MONGOC_INSERT_NONE: mongoc_insert_flags_t = 0;
pub const mongoc_insert_flags_t_MONGOC_INSERT_CONTINUE_ON_ERROR: mongoc_insert_flags_t = 1;
pub type mongoc_insert_flags_t = u32;
pub const mongoc_query_flags_t_MONGOC_QUERY_NONE: mongoc_query_flags_t = 0;
pub const mongoc_query_flags_t_MONGOC_QUERY_TAILABLE_CURSOR: mongoc_query_flags_t = 2;
pub const mongoc_query_flags_t_MONGOC_QUERY_SLAVE_OK: mongoc_query_flags_t = 4;
pub const mongoc_query_flags_t_MONGOC_QUERY_OPLOG_REPLAY: mongoc_query_flags_t = 8;
pub const mongoc_query_flags_t_MONGOC_QUERY_NO_CURSOR_TIMEOUT: mongoc_query_flags_t = 16;
pub const mongoc_query_flags_t_MONGOC_QUERY_AWAIT_DATA: mongoc_query_flags_t = 32;
pub const mongoc_query_flags_t_MONGOC_QUERY_EXHAUST: mongoc_query_flags_t = 64;
pub const mongoc_query_flags_t_MONGOC_QUERY_PARTIAL: mongoc_query_flags_t = 128;
pub type mongoc_query_flags_t = u32;
pub const mongoc_reply_flags_t_MONGOC_REPLY_NONE: mongoc_reply_flags_t = 0;
pub const mongoc_reply_flags_t_MONGOC_REPLY_CURSOR_NOT_FOUND: mongoc_reply_flags_t = 1;
pub const mongoc_reply_flags_t_MONGOC_REPLY_QUERY_FAILURE: mongoc_reply_flags_t = 2;
pub const mongoc_reply_flags_t_MONGOC_REPLY_SHARD_CONFIG_STALE: mongoc_reply_flags_t = 4;
pub const mongoc_reply_flags_t_MONGOC_REPLY_AWAIT_CAPABLE: mongoc_reply_flags_t = 8;
pub type mongoc_reply_flags_t = u32;
pub const mongoc_update_flags_t_MONGOC_UPDATE_NONE: mongoc_update_flags_t = 0;
pub const mongoc_update_flags_t_MONGOC_UPDATE_UPSERT: mongoc_update_flags_t = 1;
pub const mongoc_update_flags_t_MONGOC_UPDATE_MULTI_UPDATE: mongoc_update_flags_t = 2;
pub type mongoc_update_flags_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_cursor_t {
    _unused: [u8; 0],
}
pub type mongoc_cursor_t = _mongoc_cursor_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_client_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn mongoc_cursor_clone(cursor: *const mongoc_cursor_t) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_cursor_destroy(cursor: *mut mongoc_cursor_t);
}
extern "C" {
    pub fn mongoc_cursor_more(cursor: *mut mongoc_cursor_t) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_next(cursor: *mut mongoc_cursor_t, bson: *mut *const bson_t) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_error(cursor: *mut mongoc_cursor_t, error: *mut bson_error_t) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_error_document(
        cursor: *mut mongoc_cursor_t,
        error: *mut bson_error_t,
        doc: *mut *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_get_host(cursor: *mut mongoc_cursor_t, host: *mut mongoc_host_list_t);
}
extern "C" {
    pub fn mongoc_cursor_is_alive(cursor: *const mongoc_cursor_t) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_current(cursor: *const mongoc_cursor_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_cursor_set_batch_size(cursor: *mut mongoc_cursor_t, batch_size: u32);
}
extern "C" {
    pub fn mongoc_cursor_get_batch_size(cursor: *const mongoc_cursor_t) -> u32;
}
extern "C" {
    pub fn mongoc_cursor_set_limit(cursor: *mut mongoc_cursor_t, limit: i64) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_get_limit(cursor: *const mongoc_cursor_t) -> i64;
}
extern "C" {
    pub fn mongoc_cursor_set_hint(cursor: *mut mongoc_cursor_t, server_id: u32) -> bool;
}
extern "C" {
    pub fn mongoc_cursor_get_hint(cursor: *const mongoc_cursor_t) -> u32;
}
extern "C" {
    pub fn mongoc_cursor_get_id(cursor: *const mongoc_cursor_t) -> i64;
}
extern "C" {
    pub fn mongoc_cursor_set_max_await_time_ms(
        cursor: *mut mongoc_cursor_t,
        max_await_time_ms: u32,
    );
}
extern "C" {
    pub fn mongoc_cursor_get_max_await_time_ms(cursor: *const mongoc_cursor_t) -> u32;
}
extern "C" {
    pub fn mongoc_cursor_new_from_command_reply(
        client: *mut _mongoc_client_t,
        reply: *mut bson_t,
        server_id: u32,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_cursor_new_from_command_reply_with_opts(
        client: *mut _mongoc_client_t,
        reply: *mut bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_geo_t {
    pub twod_sphere_version: u8,
    pub twod_bits_precision: u8,
    pub twod_location_min: f64,
    pub twod_location_max: f64,
    pub haystack_bucket_size: f64,
    pub padding: [*mut u8; 32usize],
}
#[test]
fn bindgen_test_layout_mongoc_index_opt_geo_t() {
    assert_eq!(
        ::std::mem::size_of::<mongoc_index_opt_geo_t>(),
        288usize,
        concat!("Size of: ", stringify!(mongoc_index_opt_geo_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mongoc_index_opt_geo_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mongoc_index_opt_geo_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_geo_t>())).twod_sphere_version as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_geo_t),
            "::",
            stringify!(twod_sphere_version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_geo_t>())).twod_bits_precision as *const _
                as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_geo_t),
            "::",
            stringify!(twod_bits_precision)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_geo_t>())).twod_location_min as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_geo_t),
            "::",
            stringify!(twod_location_min)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_geo_t>())).twod_location_max as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_geo_t),
            "::",
            stringify!(twod_location_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_geo_t>())).haystack_bucket_size as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_geo_t),
            "::",
            stringify!(haystack_bucket_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_geo_t>())).padding as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_geo_t),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_storage_t {
    pub type_: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mongoc_index_opt_storage_t() {
    assert_eq!(
        ::std::mem::size_of::<mongoc_index_opt_storage_t>(),
        4usize,
        concat!("Size of: ", stringify!(mongoc_index_opt_storage_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mongoc_index_opt_storage_t>(),
        4usize,
        concat!("Alignment of ", stringify!(mongoc_index_opt_storage_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_storage_t>())).type_ as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_storage_t),
            "::",
            stringify!(type_)
        )
    );
}
pub const mongoc_index_storage_opt_type_t_MONGOC_INDEX_STORAGE_OPT_MMAPV1:
    mongoc_index_storage_opt_type_t = 0;
pub const mongoc_index_storage_opt_type_t_MONGOC_INDEX_STORAGE_OPT_WIREDTIGER:
    mongoc_index_storage_opt_type_t = 1;
pub type mongoc_index_storage_opt_type_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_wt_t {
    pub base: mongoc_index_opt_storage_t,
    pub config_str: *const ::std::os::raw::c_char,
    pub padding: [*mut ::std::os::raw::c_void; 8usize],
}
#[test]
fn bindgen_test_layout_mongoc_index_opt_wt_t() {
    assert_eq!(
        ::std::mem::size_of::<mongoc_index_opt_wt_t>(),
        80usize,
        concat!("Size of: ", stringify!(mongoc_index_opt_wt_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mongoc_index_opt_wt_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mongoc_index_opt_wt_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_wt_t>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_wt_t),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_wt_t>())).config_str as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_wt_t),
            "::",
            stringify!(config_str)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_wt_t>())).padding as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_wt_t),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_t {
    pub is_initialized: bool,
    pub background: bool,
    pub unique: bool,
    pub name: *const ::std::os::raw::c_char,
    pub drop_dups: bool,
    pub sparse: bool,
    pub expire_after_seconds: i32,
    pub v: i32,
    pub weights: *const bson_t,
    pub default_language: *const ::std::os::raw::c_char,
    pub language_override: *const ::std::os::raw::c_char,
    pub geo_options: *mut mongoc_index_opt_geo_t,
    pub storage_options: *mut mongoc_index_opt_storage_t,
    pub partial_filter_expression: *const bson_t,
    pub collation: *const bson_t,
    pub padding: [*mut ::std::os::raw::c_void; 4usize],
}
#[test]
fn bindgen_test_layout_mongoc_index_opt_t() {
    assert_eq!(
        ::std::mem::size_of::<mongoc_index_opt_t>(),
        120usize,
        concat!("Size of: ", stringify!(mongoc_index_opt_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mongoc_index_opt_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mongoc_index_opt_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_t>())).is_initialized as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(is_initialized)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).background as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(background)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).unique as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(unique)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).drop_dups as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(drop_dups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).sparse as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(sparse)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_t>())).expire_after_seconds as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(expire_after_seconds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).v as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(v)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).weights as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(weights)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_t>())).default_language as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(default_language)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_t>())).language_override as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(language_override)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).geo_options as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(geo_options)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_t>())).storage_options as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(storage_options)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mongoc_index_opt_t>())).partial_filter_expression as *const _
                as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(partial_filter_expression)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).collation as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(collation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_index_opt_t>())).padding as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_index_opt_t),
            "::",
            stringify!(padding)
        )
    );
}
extern "C" {
    pub fn mongoc_index_opt_get_default() -> *const mongoc_index_opt_t;
}
extern "C" {
    pub fn mongoc_index_opt_geo_get_default() -> *const mongoc_index_opt_geo_t;
}
extern "C" {
    pub fn mongoc_index_opt_wt_get_default() -> *const mongoc_index_opt_wt_t;
}
extern "C" {
    pub fn mongoc_index_opt_init(opt: *mut mongoc_index_opt_t);
}
extern "C" {
    pub fn mongoc_index_opt_geo_init(opt: *mut mongoc_index_opt_geo_t);
}
extern "C" {
    pub fn mongoc_index_opt_wt_init(opt: *mut mongoc_index_opt_wt_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_read_concern_t {
    _unused: [u8; 0],
}
pub type mongoc_read_concern_t = _mongoc_read_concern_t;
extern "C" {
    pub fn mongoc_read_concern_new() -> *mut mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_read_concern_copy(
        read_concern: *const mongoc_read_concern_t,
    ) -> *mut mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_read_concern_destroy(read_concern: *mut mongoc_read_concern_t);
}
extern "C" {
    pub fn mongoc_read_concern_get_level(
        read_concern: *const mongoc_read_concern_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_read_concern_set_level(
        read_concern: *mut mongoc_read_concern_t,
        level: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_read_concern_append(
        read_concern: *mut mongoc_read_concern_t,
        doc: *mut bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_read_concern_is_default(read_concern: *const mongoc_read_concern_t) -> bool;
}
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_NONE:
    mongoc_find_and_modify_flags_t = 0;
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_REMOVE:
    mongoc_find_and_modify_flags_t = 1;
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_UPSERT:
    mongoc_find_and_modify_flags_t = 2;
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_RETURN_NEW:
    mongoc_find_and_modify_flags_t = 4;
pub type mongoc_find_and_modify_flags_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_find_and_modify_opts_t {
    _unused: [u8; 0],
}
pub type mongoc_find_and_modify_opts_t = _mongoc_find_and_modify_opts_t;
extern "C" {
    pub fn mongoc_find_and_modify_opts_new() -> *mut mongoc_find_and_modify_opts_t;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_set_sort(
        opts: *mut mongoc_find_and_modify_opts_t,
        sort: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_sort(
        opts: *const mongoc_find_and_modify_opts_t,
        sort: *mut bson_t,
    );
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_set_update(
        opts: *mut mongoc_find_and_modify_opts_t,
        update: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_update(
        opts: *const mongoc_find_and_modify_opts_t,
        update: *mut bson_t,
    );
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_set_fields(
        opts: *mut mongoc_find_and_modify_opts_t,
        fields: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_fields(
        opts: *const mongoc_find_and_modify_opts_t,
        fields: *mut bson_t,
    );
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_set_flags(
        opts: *mut mongoc_find_and_modify_opts_t,
        flags: mongoc_find_and_modify_flags_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_flags(
        opts: *const mongoc_find_and_modify_opts_t,
    ) -> mongoc_find_and_modify_flags_t;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_set_bypass_document_validation(
        opts: *mut mongoc_find_and_modify_opts_t,
        bypass: bool,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_bypass_document_validation(
        opts: *const mongoc_find_and_modify_opts_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_set_max_time_ms(
        opts: *mut mongoc_find_and_modify_opts_t,
        max_time_ms: u32,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_max_time_ms(
        opts: *const mongoc_find_and_modify_opts_t,
    ) -> u32;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_append(
        opts: *mut mongoc_find_and_modify_opts_t,
        extra: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_get_extra(
        opts: *const mongoc_find_and_modify_opts_t,
        extra: *mut bson_t,
    );
}
extern "C" {
    pub fn mongoc_find_and_modify_opts_destroy(opts: *mut mongoc_find_and_modify_opts_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_collection_t {
    _unused: [u8; 0],
}
pub type mongoc_collection_t = _mongoc_collection_t;
extern "C" {
    pub fn mongoc_collection_aggregate(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_collection_destroy(collection: *mut mongoc_collection_t);
}
extern "C" {
    pub fn mongoc_collection_copy(collection: *mut mongoc_collection_t)
        -> *mut mongoc_collection_t;
}
extern "C" {
    pub fn mongoc_collection_command(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        command: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_collection_read_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_write_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_read_write_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_command_simple(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_count(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        query: *const bson_t,
        skip: i64,
        limit: i64,
        read_prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_collection_count_with_opts(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        query: *const bson_t,
        skip: i64,
        limit: i64,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_collection_drop(
        collection: *mut mongoc_collection_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_drop_with_opts(
        collection: *mut mongoc_collection_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_drop_index(
        collection: *mut mongoc_collection_t,
        index_name: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_drop_index_with_opts(
        collection: *mut mongoc_collection_t,
        index_name: *const ::std::os::raw::c_char,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_create_index(
        collection: *mut mongoc_collection_t,
        keys: *const bson_t,
        opt: *const mongoc_index_opt_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_create_index_with_opts(
        collection: *mut mongoc_collection_t,
        keys: *const bson_t,
        opt: *const mongoc_index_opt_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_ensure_index(
        collection: *mut mongoc_collection_t,
        keys: *const bson_t,
        opt: *const mongoc_index_opt_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_find_indexes(
        collection: *mut mongoc_collection_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_collection_find_indexes_with_opts(
        collection: *mut mongoc_collection_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_collection_find(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        query: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_collection_find_with_opts(
        collection: *mut mongoc_collection_t,
        filter: *const bson_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_collection_insert(
        collection: *mut mongoc_collection_t,
        flags: mongoc_insert_flags_t,
        document: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_insert_one(
        collection: *mut mongoc_collection_t,
        document: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_insert_many(
        collection: *mut mongoc_collection_t,
        documents: *mut *const bson_t,
        n_documents: usize,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_insert_bulk(
        collection: *mut mongoc_collection_t,
        flags: mongoc_insert_flags_t,
        documents: *mut *const bson_t,
        n_documents: u32,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_update(
        collection: *mut mongoc_collection_t,
        flags: mongoc_update_flags_t,
        selector: *const bson_t,
        update: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_update_one(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        update: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_update_many(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        update: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_replace_one(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        replacement: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_delete(
        collection: *mut mongoc_collection_t,
        flags: mongoc_delete_flags_t,
        selector: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_save(
        collection: *mut mongoc_collection_t,
        document: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_remove(
        collection: *mut mongoc_collection_t,
        flags: mongoc_remove_flags_t,
        selector: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_delete_one(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_delete_many(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_rename(
        collection: *mut mongoc_collection_t,
        new_db: *const ::std::os::raw::c_char,
        new_name: *const ::std::os::raw::c_char,
        drop_target_before_rename: bool,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_rename_with_opts(
        collection: *mut mongoc_collection_t,
        new_db: *const ::std::os::raw::c_char,
        new_name: *const ::std::os::raw::c_char,
        drop_target_before_rename: bool,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_find_and_modify_with_opts(
        collection: *mut mongoc_collection_t,
        query: *const bson_t,
        opts: *const mongoc_find_and_modify_opts_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_find_and_modify(
        collection: *mut mongoc_collection_t,
        query: *const bson_t,
        sort: *const bson_t,
        update: *const bson_t,
        fields: *const bson_t,
        _remove: bool,
        upsert: bool,
        _new: bool,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_stats(
        collection: *mut mongoc_collection_t,
        options: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_create_bulk_operation(
        collection: *mut mongoc_collection_t,
        ordered: bool,
        write_concern: *const mongoc_write_concern_t,
    ) -> *mut mongoc_bulk_operation_t;
}
extern "C" {
    pub fn mongoc_collection_create_bulk_operation_with_opts(
        collection: *mut mongoc_collection_t,
        opts: *const bson_t,
    ) -> *mut mongoc_bulk_operation_t;
}
extern "C" {
    pub fn mongoc_collection_get_read_prefs(
        collection: *const mongoc_collection_t,
    ) -> *const mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_collection_set_read_prefs(
        collection: *mut mongoc_collection_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
}
extern "C" {
    pub fn mongoc_collection_get_read_concern(
        collection: *const mongoc_collection_t,
    ) -> *const mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_collection_set_read_concern(
        collection: *mut mongoc_collection_t,
        read_concern: *const mongoc_read_concern_t,
    );
}
extern "C" {
    pub fn mongoc_collection_get_write_concern(
        collection: *const mongoc_collection_t,
    ) -> *const mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_collection_set_write_concern(
        collection: *mut mongoc_collection_t,
        write_concern: *const mongoc_write_concern_t,
    );
}
extern "C" {
    pub fn mongoc_collection_get_name(
        collection: *mut mongoc_collection_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_collection_get_last_error(
        collection: *const mongoc_collection_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_collection_keys_to_index_string(
        keys: *const bson_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_collection_validate(
        collection: *mut mongoc_collection_t,
        options: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_collection_watch(
        coll: *const mongoc_collection_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_change_stream_t;
}
extern "C" {
    pub fn mongoc_collection_count_documents(
        coll: *mut mongoc_collection_t,
        filter: *const bson_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> i64;
}
extern "C" {
    pub fn mongoc_collection_estimated_document_count(
        coll: *mut mongoc_collection_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> i64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_database_t {
    _unused: [u8; 0],
}
pub type mongoc_database_t = _mongoc_database_t;
extern "C" {
    pub fn mongoc_database_get_name(
        database: *mut mongoc_database_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_database_remove_user(
        database: *mut mongoc_database_t,
        username: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_remove_all_users(
        database: *mut mongoc_database_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_add_user(
        database: *mut mongoc_database_t,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
        roles: *const bson_t,
        custom_data: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_destroy(database: *mut mongoc_database_t);
}
extern "C" {
    pub fn mongoc_database_copy(database: *mut mongoc_database_t) -> *mut mongoc_database_t;
}
extern "C" {
    pub fn mongoc_database_command(
        database: *mut mongoc_database_t,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        command: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_database_read_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_write_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_read_write_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_command_simple(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_drop(database: *mut mongoc_database_t, error: *mut bson_error_t)
        -> bool;
}
extern "C" {
    pub fn mongoc_database_drop_with_opts(
        database: *mut mongoc_database_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_has_collection(
        database: *mut mongoc_database_t,
        name: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_database_create_collection(
        database: *mut mongoc_database_t,
        name: *const ::std::os::raw::c_char,
        options: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_collection_t;
}
extern "C" {
    pub fn mongoc_database_get_read_prefs(
        database: *const mongoc_database_t,
    ) -> *const mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_database_set_read_prefs(
        database: *mut mongoc_database_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
}
extern "C" {
    pub fn mongoc_database_get_write_concern(
        database: *const mongoc_database_t,
    ) -> *const mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_database_set_write_concern(
        database: *mut mongoc_database_t,
        write_concern: *const mongoc_write_concern_t,
    );
}
extern "C" {
    pub fn mongoc_database_get_read_concern(
        database: *const mongoc_database_t,
    ) -> *const mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_database_set_read_concern(
        database: *mut mongoc_database_t,
        read_concern: *const mongoc_read_concern_t,
    );
}
extern "C" {
    pub fn mongoc_database_find_collections(
        database: *mut mongoc_database_t,
        filter: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_database_find_collections_with_opts(
        database: *mut mongoc_database_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_database_get_collection_names(
        database: *mut mongoc_database_t,
        error: *mut bson_error_t,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_database_get_collection_names_with_opts(
        database: *mut mongoc_database_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_database_get_collection(
        database: *mut mongoc_database_t,
        name: *const ::std::os::raw::c_char,
    ) -> *mut mongoc_collection_t;
}
extern "C" {
    pub fn mongoc_database_watch(
        db: *const mongoc_database_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_change_stream_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}
#[test]
fn bindgen_test_layout_iovec() {
    assert_eq!(
        ::std::mem::size_of::<iovec>(),
        16usize,
        concat!("Size of: ", stringify!(iovec))
    );
    assert_eq!(
        ::std::mem::align_of::<iovec>(),
        8usize,
        concat!("Alignment of ", stringify!(iovec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iovec>())).iov_base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(iovec),
            "::",
            stringify!(iov_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iovec>())).iov_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(iovec),
            "::",
            stringify!(iov_len)
        )
    );
}
extern "C" {
    pub fn readv(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
    ) -> isize;
}
extern "C" {
    pub fn writev(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
    ) -> isize;
}
extern "C" {
    pub fn preadv(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
        __offset: __off_t,
    ) -> isize;
}
extern "C" {
    pub fn pwritev(
        __fd: ::std::os::raw::c_int,
        __iovec: *const iovec,
        __count: ::std::os::raw::c_int,
        __offset: __off_t,
    ) -> isize;
}
pub type mongoc_iovec_t = iovec;
pub const __socket_type_SOCK_STREAM: __socket_type = 1;
pub const __socket_type_SOCK_DGRAM: __socket_type = 2;
pub const __socket_type_SOCK_RAW: __socket_type = 3;
pub const __socket_type_SOCK_RDM: __socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: __socket_type = 5;
pub const __socket_type_SOCK_DCCP: __socket_type = 6;
pub const __socket_type_SOCK_PACKET: __socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: __socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: __socket_type = 2048;
pub type __socket_type = u32;
pub type sa_family_t = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::std::os::raw::c_char; 14usize],
}
#[test]
fn bindgen_test_layout_sockaddr() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(sockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::std::os::raw::c_char; 118usize],
    pub __ss_align: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_sockaddr_storage() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_storage>(),
        128usize,
        concat!("Size of: ", stringify!(sockaddr_storage))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_storage>(),
        8usize,
        concat!("Alignment of ", stringify!(sockaddr_storage))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_storage>())).ss_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_storage),
            "::",
            stringify!(ss_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_storage>())).__ss_padding as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_storage),
            "::",
            stringify!(__ss_padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_storage>())).__ss_align as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_storage),
            "::",
            stringify!(__ss_align)
        )
    );
}
pub const MSG_OOB: _bindgen_ty_5 = 1;
pub const MSG_PEEK: _bindgen_ty_5 = 2;
pub const MSG_DONTROUTE: _bindgen_ty_5 = 4;
pub const MSG_CTRUNC: _bindgen_ty_5 = 8;
pub const MSG_PROXY: _bindgen_ty_5 = 16;
pub const MSG_TRUNC: _bindgen_ty_5 = 32;
pub const MSG_DONTWAIT: _bindgen_ty_5 = 64;
pub const MSG_EOR: _bindgen_ty_5 = 128;
pub const MSG_WAITALL: _bindgen_ty_5 = 256;
pub const MSG_FIN: _bindgen_ty_5 = 512;
pub const MSG_SYN: _bindgen_ty_5 = 1024;
pub const MSG_CONFIRM: _bindgen_ty_5 = 2048;
pub const MSG_RST: _bindgen_ty_5 = 4096;
pub const MSG_ERRQUEUE: _bindgen_ty_5 = 8192;
pub const MSG_NOSIGNAL: _bindgen_ty_5 = 16384;
pub const MSG_MORE: _bindgen_ty_5 = 32768;
pub const MSG_WAITFORONE: _bindgen_ty_5 = 65536;
pub const MSG_BATCH: _bindgen_ty_5 = 262144;
pub const MSG_ZEROCOPY: _bindgen_ty_5 = 67108864;
pub const MSG_FASTOPEN: _bindgen_ty_5 = 536870912;
pub const MSG_CMSG_CLOEXEC: _bindgen_ty_5 = 1073741824;
pub type _bindgen_ty_5 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut ::std::os::raw::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut ::std::os::raw::c_void,
    pub msg_controllen: usize,
    pub msg_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_msghdr() {
    assert_eq!(
        ::std::mem::size_of::<msghdr>(),
        56usize,
        concat!("Size of: ", stringify!(msghdr))
    );
    assert_eq!(
        ::std::mem::align_of::<msghdr>(),
        8usize,
        concat!("Alignment of ", stringify!(msghdr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_namelen as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_namelen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_iov as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_iov)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_iovlen as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_iovlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_control as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_control)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_controllen as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_controllen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: ::std::os::raw::c_int,
    pub cmsg_type: ::std::os::raw::c_int,
    pub __cmsg_data: __IncompleteArrayField<::std::os::raw::c_uchar>,
}
#[test]
fn bindgen_test_layout_cmsghdr() {
    assert_eq!(
        ::std::mem::size_of::<cmsghdr>(),
        16usize,
        concat!("Size of: ", stringify!(cmsghdr))
    );
    assert_eq!(
        ::std::mem::align_of::<cmsghdr>(),
        8usize,
        concat!("Alignment of ", stringify!(cmsghdr))
    );
}
extern "C" {
    pub fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
}
pub const SCM_RIGHTS: _bindgen_ty_6 = 1;
pub type _bindgen_ty_6 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct linger {
    pub l_onoff: ::std::os::raw::c_int,
    pub l_linger: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_linger() {
    assert_eq!(
        ::std::mem::size_of::<linger>(),
        8usize,
        concat!("Size of: ", stringify!(linger))
    );
    assert_eq!(
        ::std::mem::align_of::<linger>(),
        4usize,
        concat!("Alignment of ", stringify!(linger))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<linger>())).l_onoff as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(linger),
            "::",
            stringify!(l_onoff)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<linger>())).l_linger as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(linger),
            "::",
            stringify!(l_linger)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osockaddr {
    pub sa_family: ::std::os::raw::c_ushort,
    pub sa_data: [::std::os::raw::c_uchar; 14usize],
}
#[test]
fn bindgen_test_layout_osockaddr() {
    assert_eq!(
        ::std::mem::size_of::<osockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(osockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<osockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(osockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<osockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(osockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<osockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(osockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
pub const SHUT_RD: _bindgen_ty_7 = 0;
pub const SHUT_WR: _bindgen_ty_7 = 1;
pub const SHUT_RDWR: _bindgen_ty_7 = 2;
pub type _bindgen_ty_7 = u32;
extern "C" {
    pub fn socket(
        __domain: ::std::os::raw::c_int,
        __type: ::std::os::raw::c_int,
        __protocol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn socketpair(
        __domain: ::std::os::raw::c_int,
        __type: ::std::os::raw::c_int,
        __protocol: ::std::os::raw::c_int,
        __fds: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bind(
        __fd: ::std::os::raw::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsockname(
        __fd: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn connect(
        __fd: ::std::os::raw::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpeername(
        __fd: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn send(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
        __flags: ::std::os::raw::c_int,
    ) -> isize;
}
extern "C" {
    pub fn recv(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __n: usize,
        __flags: ::std::os::raw::c_int,
    ) -> isize;
}
extern "C" {
    pub fn sendto(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
        __flags: ::std::os::raw::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> isize;
}
extern "C" {
    pub fn recvfrom(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __n: usize,
        __flags: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> isize;
}
extern "C" {
    pub fn sendmsg(
        __fd: ::std::os::raw::c_int,
        __message: *const msghdr,
        __flags: ::std::os::raw::c_int,
    ) -> isize;
}
extern "C" {
    pub fn recvmsg(
        __fd: ::std::os::raw::c_int,
        __message: *mut msghdr,
        __flags: ::std::os::raw::c_int,
    ) -> isize;
}
extern "C" {
    pub fn getsockopt(
        __fd: ::std::os::raw::c_int,
        __level: ::std::os::raw::c_int,
        __optname: ::std::os::raw::c_int,
        __optval: *mut ::std::os::raw::c_void,
        __optlen: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setsockopt(
        __fd: ::std::os::raw::c_int,
        __level: ::std::os::raw::c_int,
        __optname: ::std::os::raw::c_int,
        __optval: *const ::std::os::raw::c_void,
        __optlen: socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn listen(__fd: ::std::os::raw::c_int, __n: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn accept(
        __fd: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn shutdown(
        __fd: ::std::os::raw::c_int,
        __how: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sockatmark(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isfdtype(
        __fd: ::std::os::raw::c_int,
        __fdtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type in_addr_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[test]
fn bindgen_test_layout_in_addr() {
    assert_eq!(
        ::std::mem::size_of::<in_addr>(),
        4usize,
        concat!("Size of: ", stringify!(in_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_addr>())).s_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in_addr),
            "::",
            stringify!(s_addr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_opts {
    pub ip_dst: in_addr,
    pub ip_opts: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout_ip_opts() {
    assert_eq!(
        ::std::mem::size_of::<ip_opts>(),
        44usize,
        concat!("Size of: ", stringify!(ip_opts))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_opts>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_opts))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_opts>())).ip_dst as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_opts),
            "::",
            stringify!(ip_dst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_opts>())).ip_opts as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_opts),
            "::",
            stringify!(ip_opts)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ip_mreqn() {
    assert_eq!(
        ::std::mem::size_of::<ip_mreqn>(),
        12usize,
        concat!("Size of: ", stringify!(ip_mreqn))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_mreqn>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_mreqn))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreqn>())).imr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreqn),
            "::",
            stringify!(imr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreqn>())).imr_address as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreqn),
            "::",
            stringify!(imr_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreqn>())).imr_ifindex as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreqn),
            "::",
            stringify!(imr_ifindex)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_pktinfo {
    pub ipi_ifindex: ::std::os::raw::c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}
#[test]
fn bindgen_test_layout_in_pktinfo() {
    assert_eq!(
        ::std::mem::size_of::<in_pktinfo>(),
        12usize,
        concat!("Size of: ", stringify!(in_pktinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<in_pktinfo>(),
        4usize,
        concat!("Alignment of ", stringify!(in_pktinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_pktinfo>())).ipi_ifindex as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in_pktinfo),
            "::",
            stringify!(ipi_ifindex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_pktinfo>())).ipi_spec_dst as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(in_pktinfo),
            "::",
            stringify!(ipi_spec_dst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_pktinfo>())).ipi_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(in_pktinfo),
            "::",
            stringify!(ipi_addr)
        )
    );
}
pub const IPPROTO_IP: _bindgen_ty_8 = 0;
pub const IPPROTO_ICMP: _bindgen_ty_8 = 1;
pub const IPPROTO_IGMP: _bindgen_ty_8 = 2;
pub const IPPROTO_IPIP: _bindgen_ty_8 = 4;
pub const IPPROTO_TCP: _bindgen_ty_8 = 6;
pub const IPPROTO_EGP: _bindgen_ty_8 = 8;
pub const IPPROTO_PUP: _bindgen_ty_8 = 12;
pub const IPPROTO_UDP: _bindgen_ty_8 = 17;
pub const IPPROTO_IDP: _bindgen_ty_8 = 22;
pub const IPPROTO_TP: _bindgen_ty_8 = 29;
pub const IPPROTO_DCCP: _bindgen_ty_8 = 33;
pub const IPPROTO_IPV6: _bindgen_ty_8 = 41;
pub const IPPROTO_RSVP: _bindgen_ty_8 = 46;
pub const IPPROTO_GRE: _bindgen_ty_8 = 47;
pub const IPPROTO_ESP: _bindgen_ty_8 = 50;
pub const IPPROTO_AH: _bindgen_ty_8 = 51;
pub const IPPROTO_MTP: _bindgen_ty_8 = 92;
pub const IPPROTO_BEETPH: _bindgen_ty_8 = 94;
pub const IPPROTO_ENCAP: _bindgen_ty_8 = 98;
pub const IPPROTO_PIM: _bindgen_ty_8 = 103;
pub const IPPROTO_COMP: _bindgen_ty_8 = 108;
pub const IPPROTO_SCTP: _bindgen_ty_8 = 132;
pub const IPPROTO_UDPLITE: _bindgen_ty_8 = 136;
pub const IPPROTO_MPLS: _bindgen_ty_8 = 137;
pub const IPPROTO_RAW: _bindgen_ty_8 = 255;
pub const IPPROTO_MAX: _bindgen_ty_8 = 256;
pub type _bindgen_ty_8 = u32;
pub const IPPROTO_HOPOPTS: _bindgen_ty_9 = 0;
pub const IPPROTO_ROUTING: _bindgen_ty_9 = 43;
pub const IPPROTO_FRAGMENT: _bindgen_ty_9 = 44;
pub const IPPROTO_ICMPV6: _bindgen_ty_9 = 58;
pub const IPPROTO_NONE: _bindgen_ty_9 = 59;
pub const IPPROTO_DSTOPTS: _bindgen_ty_9 = 60;
pub const IPPROTO_MH: _bindgen_ty_9 = 135;
pub type _bindgen_ty_9 = u32;
pub type in_port_t = u16;
pub const IPPORT_ECHO: _bindgen_ty_10 = 7;
pub const IPPORT_DISCARD: _bindgen_ty_10 = 9;
pub const IPPORT_SYSTAT: _bindgen_ty_10 = 11;
pub const IPPORT_DAYTIME: _bindgen_ty_10 = 13;
pub const IPPORT_NETSTAT: _bindgen_ty_10 = 15;
pub const IPPORT_FTP: _bindgen_ty_10 = 21;
pub const IPPORT_TELNET: _bindgen_ty_10 = 23;
pub const IPPORT_SMTP: _bindgen_ty_10 = 25;
pub const IPPORT_TIMESERVER: _bindgen_ty_10 = 37;
pub const IPPORT_NAMESERVER: _bindgen_ty_10 = 42;
pub const IPPORT_WHOIS: _bindgen_ty_10 = 43;
pub const IPPORT_MTP: _bindgen_ty_10 = 57;
pub const IPPORT_TFTP: _bindgen_ty_10 = 69;
pub const IPPORT_RJE: _bindgen_ty_10 = 77;
pub const IPPORT_FINGER: _bindgen_ty_10 = 79;
pub const IPPORT_TTYLINK: _bindgen_ty_10 = 87;
pub const IPPORT_SUPDUP: _bindgen_ty_10 = 95;
pub const IPPORT_EXECSERVER: _bindgen_ty_10 = 512;
pub const IPPORT_LOGINSERVER: _bindgen_ty_10 = 513;
pub const IPPORT_CMDSERVER: _bindgen_ty_10 = 514;
pub const IPPORT_EFSSERVER: _bindgen_ty_10 = 520;
pub const IPPORT_BIFFUDP: _bindgen_ty_10 = 512;
pub const IPPORT_WHOSERVER: _bindgen_ty_10 = 513;
pub const IPPORT_ROUTESERVER: _bindgen_ty_10 = 520;
pub const IPPORT_RESERVED: _bindgen_ty_10 = 1024;
pub const IPPORT_USERRESERVED: _bindgen_ty_10 = 5000;
pub type _bindgen_ty_10 = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __in6_u: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub __u6_addr8: [u8; 16usize],
    pub __u6_addr16: [u16; 8usize],
    pub __u6_addr32: [u32; 4usize],
    _bindgen_union_align: [u32; 4usize],
}
#[test]
fn bindgen_test_layout_in6_addr__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<in6_addr__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr8 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr16 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr16)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr32)
        )
    );
}
#[test]
fn bindgen_test_layout_in6_addr() {
    assert_eq!(
        ::std::mem::size_of::<in6_addr>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in6_addr>())).__in6_u as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr),
            "::",
            stringify!(__in6_u)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}in6addr_any"]
    pub static in6addr_any: in6_addr;
}
extern "C" {
    #[link_name = "\u{1}in6addr_loopback"]
    pub static in6addr_loopback: in6_addr;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_sockaddr_in() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_in>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr_in))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_in>(),
        4usize,
        concat!("Alignment of ", stringify!(sockaddr_in))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_addr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_zero as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_zero)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[test]
fn bindgen_test_layout_sockaddr_in6() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_in6>(),
        28usize,
        concat!("Size of: ", stringify!(sockaddr_in6))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_in6>(),
        4usize,
        concat!("Alignment of ", stringify!(sockaddr_in6))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_flowinfo as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_flowinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_scope_id as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_scope_id)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[test]
fn bindgen_test_layout_ip_mreq() {
    assert_eq!(
        ::std::mem::size_of::<ip_mreq>(),
        8usize,
        concat!("Size of: ", stringify!(ip_mreq))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_mreq>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_mreq))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq>())).imr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq),
            "::",
            stringify!(imr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq>())).imr_interface as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq),
            "::",
            stringify!(imr_interface)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
#[test]
fn bindgen_test_layout_ip_mreq_source() {
    assert_eq!(
        ::std::mem::size_of::<ip_mreq_source>(),
        12usize,
        concat!("Size of: ", stringify!(ip_mreq_source))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_mreq_source>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_mreq_source))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq_source>())).imr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq_source),
            "::",
            stringify!(imr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq_source>())).imr_interface as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq_source),
            "::",
            stringify!(imr_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq_source>())).imr_sourceaddr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq_source),
            "::",
            stringify!(imr_sourceaddr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_ipv6_mreq() {
    assert_eq!(
        ::std::mem::size_of::<ipv6_mreq>(),
        20usize,
        concat!("Size of: ", stringify!(ipv6_mreq))
    );
    assert_eq!(
        ::std::mem::align_of::<ipv6_mreq>(),
        4usize,
        concat!("Alignment of ", stringify!(ipv6_mreq))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ipv6_mreq>())).ipv6mr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_mreq),
            "::",
            stringify!(ipv6mr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ipv6_mreq>())).ipv6mr_interface as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_mreq),
            "::",
            stringify!(ipv6mr_interface)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_req {
    pub gr_interface: u32,
    pub gr_group: sockaddr_storage,
}
#[test]
fn bindgen_test_layout_group_req() {
    assert_eq!(
        ::std::mem::size_of::<group_req>(),
        136usize,
        concat!("Size of: ", stringify!(group_req))
    );
    assert_eq!(
        ::std::mem::align_of::<group_req>(),
        8usize,
        concat!("Alignment of ", stringify!(group_req))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_req>())).gr_interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(group_req),
            "::",
            stringify!(gr_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_req>())).gr_group as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(group_req),
            "::",
            stringify!(gr_group)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: sockaddr_storage,
    pub gsr_source: sockaddr_storage,
}
#[test]
fn bindgen_test_layout_group_source_req() {
    assert_eq!(
        ::std::mem::size_of::<group_source_req>(),
        264usize,
        concat!("Size of: ", stringify!(group_source_req))
    );
    assert_eq!(
        ::std::mem::align_of::<group_source_req>(),
        8usize,
        concat!("Alignment of ", stringify!(group_source_req))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_source_req>())).gsr_interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(group_source_req),
            "::",
            stringify!(gsr_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_source_req>())).gsr_group as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(group_source_req),
            "::",
            stringify!(gsr_group)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_source_req>())).gsr_source as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(group_source_req),
            "::",
            stringify!(gsr_source)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_msfilter {
    pub imsf_multiaddr: in_addr,
    pub imsf_interface: in_addr,
    pub imsf_fmode: u32,
    pub imsf_numsrc: u32,
    pub imsf_slist: [in_addr; 1usize],
}
#[test]
fn bindgen_test_layout_ip_msfilter() {
    assert_eq!(
        ::std::mem::size_of::<ip_msfilter>(),
        20usize,
        concat!("Size of: ", stringify!(ip_msfilter))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_msfilter>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_msfilter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_interface as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_fmode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_fmode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_numsrc as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_numsrc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_slist as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_slist)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_group: sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist: [sockaddr_storage; 1usize],
}
#[test]
fn bindgen_test_layout_group_filter() {
    assert_eq!(
        ::std::mem::size_of::<group_filter>(),
        272usize,
        concat!("Size of: ", stringify!(group_filter))
    );
    assert_eq!(
        ::std::mem::align_of::<group_filter>(),
        8usize,
        concat!("Alignment of ", stringify!(group_filter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_group as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_group)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_fmode as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_fmode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_numsrc as *const _ as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_numsrc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_slist as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_slist)
        )
    );
}
extern "C" {
    pub fn ntohl(__netlong: u32) -> u32;
}
extern "C" {
    pub fn ntohs(__netshort: u16) -> u16;
}
extern "C" {
    pub fn htonl(__hostlong: u32) -> u32;
}
extern "C" {
    pub fn htons(__hostshort: u16) -> u16;
}
extern "C" {
    pub fn bindresvport(
        __sockfd: ::std::os::raw::c_int,
        __sock_in: *mut sockaddr_in,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bindresvport6(
        __sockfd: ::std::os::raw::c_int,
        __sock_in: *mut sockaddr_in6,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inet_addr(__cp: *const ::std::os::raw::c_char) -> in_addr_t;
}
extern "C" {
    pub fn inet_lnaof(__in: in_addr) -> in_addr_t;
}
extern "C" {
    pub fn inet_makeaddr(__net: in_addr_t, __host: in_addr_t) -> in_addr;
}
extern "C" {
    pub fn inet_netof(__in: in_addr) -> in_addr_t;
}
extern "C" {
    pub fn inet_network(__cp: *const ::std::os::raw::c_char) -> in_addr_t;
}
extern "C" {
    pub fn inet_ntoa(__in: in_addr) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn inet_pton(
        __af: ::std::os::raw::c_int,
        __cp: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inet_ntop(
        __af: ::std::os::raw::c_int,
        __cp: *const ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __len: socklen_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn inet_aton(
        __cp: *const ::std::os::raw::c_char,
        __inp: *mut in_addr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inet_neta(
        __net: in_addr_t,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn inet_net_ntop(
        __af: ::std::os::raw::c_int,
        __cp: *const ::std::os::raw::c_void,
        __bits: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn inet_net_pton(
        __af: ::std::os::raw::c_int,
        __cp: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_void,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inet_nsap_addr(
        __cp: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_uchar,
        __len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn inet_nsap_ntoa(
        __len: ::std::os::raw::c_int,
        __cp: *const ::std::os::raw::c_uchar,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type nfds_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pollfd {
    pub fd: ::std::os::raw::c_int,
    pub events: ::std::os::raw::c_short,
    pub revents: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_pollfd() {
    assert_eq!(
        ::std::mem::size_of::<pollfd>(),
        8usize,
        concat!("Size of: ", stringify!(pollfd))
    );
    assert_eq!(
        ::std::mem::align_of::<pollfd>(),
        4usize,
        concat!("Alignment of ", stringify!(pollfd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pollfd>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pollfd),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pollfd>())).events as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(pollfd),
            "::",
            stringify!(events)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pollfd>())).revents as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(pollfd),
            "::",
            stringify!(revents)
        )
    );
}
extern "C" {
    pub fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rpcent {
    pub r_name: *mut ::std::os::raw::c_char,
    pub r_aliases: *mut *mut ::std::os::raw::c_char,
    pub r_number: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_rpcent() {
    assert_eq!(
        ::std::mem::size_of::<rpcent>(),
        24usize,
        concat!("Size of: ", stringify!(rpcent))
    );
    assert_eq!(
        ::std::mem::align_of::<rpcent>(),
        8usize,
        concat!("Alignment of ", stringify!(rpcent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpcent>())).r_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rpcent),
            "::",
            stringify!(r_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpcent>())).r_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rpcent),
            "::",
            stringify!(r_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpcent>())).r_number as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rpcent),
            "::",
            stringify!(r_number)
        )
    );
}
extern "C" {
    pub fn setrpcent(__stayopen: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endrpcent();
}
extern "C" {
    pub fn getrpcbyname(__name: *const ::std::os::raw::c_char) -> *mut rpcent;
}
extern "C" {
    pub fn getrpcbynumber(__number: ::std::os::raw::c_int) -> *mut rpcent;
}
extern "C" {
    pub fn getrpcent() -> *mut rpcent;
}
extern "C" {
    pub fn getrpcbyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut rpcent,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut rpcent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrpcbynumber_r(
        __number: ::std::os::raw::c_int,
        __result_buf: *mut rpcent,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut rpcent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrpcent_r(
        __result_buf: *mut rpcent,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut rpcent,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct netent {
    pub n_name: *mut ::std::os::raw::c_char,
    pub n_aliases: *mut *mut ::std::os::raw::c_char,
    pub n_addrtype: ::std::os::raw::c_int,
    pub n_net: u32,
}
#[test]
fn bindgen_test_layout_netent() {
    assert_eq!(
        ::std::mem::size_of::<netent>(),
        24usize,
        concat!("Size of: ", stringify!(netent))
    );
    assert_eq!(
        ::std::mem::align_of::<netent>(),
        8usize,
        concat!("Alignment of ", stringify!(netent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_addrtype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_addrtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_net as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_net)
        )
    );
}
extern "C" {
    pub fn __h_errno_location() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn herror(__str: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn hstrerror(__err_num: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostent {
    pub h_name: *mut ::std::os::raw::c_char,
    pub h_aliases: *mut *mut ::std::os::raw::c_char,
    pub h_addrtype: ::std::os::raw::c_int,
    pub h_length: ::std::os::raw::c_int,
    pub h_addr_list: *mut *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_hostent() {
    assert_eq!(
        ::std::mem::size_of::<hostent>(),
        32usize,
        concat!("Size of: ", stringify!(hostent))
    );
    assert_eq!(
        ::std::mem::align_of::<hostent>(),
        8usize,
        concat!("Alignment of ", stringify!(hostent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_addrtype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_addrtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_length as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_addr_list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_addr_list)
        )
    );
}
extern "C" {
    pub fn sethostent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endhostent();
}
extern "C" {
    pub fn gethostent() -> *mut hostent;
}
extern "C" {
    pub fn gethostbyaddr(
        __addr: *const ::std::os::raw::c_void,
        __len: __socklen_t,
        __type: ::std::os::raw::c_int,
    ) -> *mut hostent;
}
extern "C" {
    pub fn gethostbyname(__name: *const ::std::os::raw::c_char) -> *mut hostent;
}
extern "C" {
    pub fn gethostbyname2(
        __name: *const ::std::os::raw::c_char,
        __af: ::std::os::raw::c_int,
    ) -> *mut hostent;
}
extern "C" {
    pub fn gethostent_r(
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostbyaddr_r(
        __addr: *const ::std::os::raw::c_void,
        __len: __socklen_t,
        __type: ::std::os::raw::c_int,
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostbyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostbyname2_r(
        __name: *const ::std::os::raw::c_char,
        __af: ::std::os::raw::c_int,
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setnetent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endnetent();
}
extern "C" {
    pub fn getnetent() -> *mut netent;
}
extern "C" {
    pub fn getnetbyaddr(__net: u32, __type: ::std::os::raw::c_int) -> *mut netent;
}
extern "C" {
    pub fn getnetbyname(__name: *const ::std::os::raw::c_char) -> *mut netent;
}
extern "C" {
    pub fn getnetent_r(
        __result_buf: *mut netent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut netent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getnetbyaddr_r(
        __net: u32,
        __type: ::std::os::raw::c_int,
        __result_buf: *mut netent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut netent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getnetbyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut netent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut netent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct servent {
    pub s_name: *mut ::std::os::raw::c_char,
    pub s_aliases: *mut *mut ::std::os::raw::c_char,
    pub s_port: ::std::os::raw::c_int,
    pub s_proto: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_servent() {
    assert_eq!(
        ::std::mem::size_of::<servent>(),
        32usize,
        concat!("Size of: ", stringify!(servent))
    );
    assert_eq!(
        ::std::mem::align_of::<servent>(),
        8usize,
        concat!("Alignment of ", stringify!(servent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_port as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_proto as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_proto)
        )
    );
}
extern "C" {
    pub fn setservent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endservent();
}
extern "C" {
    pub fn getservent() -> *mut servent;
}
extern "C" {
    pub fn getservbyname(
        __name: *const ::std::os::raw::c_char,
        __proto: *const ::std::os::raw::c_char,
    ) -> *mut servent;
}
extern "C" {
    pub fn getservbyport(
        __port: ::std::os::raw::c_int,
        __proto: *const ::std::os::raw::c_char,
    ) -> *mut servent;
}
extern "C" {
    pub fn getservent_r(
        __result_buf: *mut servent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut servent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getservbyname_r(
        __name: *const ::std::os::raw::c_char,
        __proto: *const ::std::os::raw::c_char,
        __result_buf: *mut servent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut servent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getservbyport_r(
        __port: ::std::os::raw::c_int,
        __proto: *const ::std::os::raw::c_char,
        __result_buf: *mut servent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut servent,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct protoent {
    pub p_name: *mut ::std::os::raw::c_char,
    pub p_aliases: *mut *mut ::std::os::raw::c_char,
    pub p_proto: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_protoent() {
    assert_eq!(
        ::std::mem::size_of::<protoent>(),
        24usize,
        concat!("Size of: ", stringify!(protoent))
    );
    assert_eq!(
        ::std::mem::align_of::<protoent>(),
        8usize,
        concat!("Alignment of ", stringify!(protoent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<protoent>())).p_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(protoent),
            "::",
            stringify!(p_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<protoent>())).p_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(protoent),
            "::",
            stringify!(p_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<protoent>())).p_proto as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(protoent),
            "::",
            stringify!(p_proto)
        )
    );
}
extern "C" {
    pub fn setprotoent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endprotoent();
}
extern "C" {
    pub fn getprotoent() -> *mut protoent;
}
extern "C" {
    pub fn getprotobyname(__name: *const ::std::os::raw::c_char) -> *mut protoent;
}
extern "C" {
    pub fn getprotobynumber(__proto: ::std::os::raw::c_int) -> *mut protoent;
}
extern "C" {
    pub fn getprotoent_r(
        __result_buf: *mut protoent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut protoent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprotobyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut protoent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut protoent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprotobynumber_r(
        __proto: ::std::os::raw::c_int,
        __result_buf: *mut protoent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
        __result: *mut *mut protoent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setnetgrent(__netgroup: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn endnetgrent();
}
extern "C" {
    pub fn getnetgrent(
        __hostp: *mut *mut ::std::os::raw::c_char,
        __userp: *mut *mut ::std::os::raw::c_char,
        __domainp: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn innetgr(
        __netgroup: *const ::std::os::raw::c_char,
        __host: *const ::std::os::raw::c_char,
        __user: *const ::std::os::raw::c_char,
        __domain: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getnetgrent_r(
        __hostp: *mut *mut ::std::os::raw::c_char,
        __userp: *mut *mut ::std::os::raw::c_char,
        __domainp: *mut *mut ::std::os::raw::c_char,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rcmd(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_ushort,
        __locuser: *const ::std::os::raw::c_char,
        __remuser: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rcmd_af(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_ushort,
        __locuser: *const ::std::os::raw::c_char,
        __remuser: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rexec(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __pass: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rexec_af(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __pass: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ruserok(
        __rhost: *const ::std::os::raw::c_char,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ruserok_af(
        __rhost: *const ::std::os::raw::c_char,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iruserok(
        __raddr: u32,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iruserok_af(
        __raddr: *const ::std::os::raw::c_void,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rresvport(__alport: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rresvport_af(
        __alport: *mut ::std::os::raw::c_int,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct addrinfo {
    pub ai_flags: ::std::os::raw::c_int,
    pub ai_family: ::std::os::raw::c_int,
    pub ai_socktype: ::std::os::raw::c_int,
    pub ai_protocol: ::std::os::raw::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::std::os::raw::c_char,
    pub ai_next: *mut addrinfo,
}
#[test]
fn bindgen_test_layout_addrinfo() {
    assert_eq!(
        ::std::mem::size_of::<addrinfo>(),
        48usize,
        concat!("Size of: ", stringify!(addrinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<addrinfo>(),
        8usize,
        concat!("Alignment of ", stringify!(addrinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_family as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_socktype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_socktype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_protocol as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_protocol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_addrlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_addrlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_addr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_canonname as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_canonname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_next as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_next)
        )
    );
}
extern "C" {
    pub fn getaddrinfo(
        __name: *const ::std::os::raw::c_char,
        __service: *const ::std::os::raw::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn freeaddrinfo(__ai: *mut addrinfo);
}
extern "C" {
    pub fn gai_strerror(__ecode: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut ::std::os::raw::c_char,
        __hostlen: socklen_t,
        __serv: *mut ::std::os::raw::c_char,
        __servlen: socklen_t,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type tcp_seq = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub __bindgen_anon_1: tcphdr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tcphdr__bindgen_ty_1 {
    pub __bindgen_anon_1: tcphdr__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: tcphdr__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u32; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcphdr__bindgen_ty_1__bindgen_ty_1 {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}
#[test]
fn bindgen_test_layout_tcphdr__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<tcphdr__bindgen_ty_1__bindgen_ty_1>(),
        20usize,
        concat!("Size of: ", stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<tcphdr__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_sport as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_sport)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_dport as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_dport)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_seq as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_seq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_ack as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_ack)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_flags as *const _
                as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_win as *const _
                as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_win)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_sum as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_sum)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_1>())).th_urp as *const _
                as usize
        },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(th_urp)
        )
    );
}
impl tcphdr__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn th_x2(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_th_x2(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn th_off(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_th_off(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(th_x2: u8, th_off: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let th_x2: u8 = unsafe { ::std::mem::transmute(th_x2) };
            th_x2 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let th_off: u8 = unsafe { ::std::mem::transmute(th_off) };
            th_off as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcphdr__bindgen_ty_1__bindgen_ty_2 {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}
#[test]
fn bindgen_test_layout_tcphdr__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<tcphdr__bindgen_ty_1__bindgen_ty_2>(),
        20usize,
        concat!("Size of: ", stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<tcphdr__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).source as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).dest as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(dest)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).seq as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(seq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).ack_seq as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(ack_seq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).window as *const _
                as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(window)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).check as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(check)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcphdr__bindgen_ty_1__bindgen_ty_2>())).urg_ptr as *const _
                as usize
        },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(tcphdr__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(urg_ptr)
        )
    );
}
impl tcphdr__bindgen_ty_1__bindgen_ty_2 {
    #[inline]
    pub fn res1(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_res1(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn doff(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_doff(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn fin(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_fin(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn syn(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_syn(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn rst(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_rst(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn psh(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_psh(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ack(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_ack(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn urg(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_urg(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn res2(&self) -> u16 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set_res2(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        res1: u16,
        doff: u16,
        fin: u16,
        syn: u16,
        rst: u16,
        psh: u16,
        ack: u16,
        urg: u16,
        res2: u16,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let res1: u16 = unsafe { ::std::mem::transmute(res1) };
            res1 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let doff: u16 = unsafe { ::std::mem::transmute(doff) };
            doff as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let fin: u16 = unsafe { ::std::mem::transmute(fin) };
            fin as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let syn: u16 = unsafe { ::std::mem::transmute(syn) };
            syn as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let rst: u16 = unsafe { ::std::mem::transmute(rst) };
            rst as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let psh: u16 = unsafe { ::std::mem::transmute(psh) };
            psh as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let ack: u16 = unsafe { ::std::mem::transmute(ack) };
            ack as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let urg: u16 = unsafe { ::std::mem::transmute(urg) };
            urg as u64
        });
        __bindgen_bitfield_unit.set(14usize, 2u8, {
            let res2: u16 = unsafe { ::std::mem::transmute(res2) };
            res2 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_tcphdr__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<tcphdr__bindgen_ty_1>(),
        20usize,
        concat!("Size of: ", stringify!(tcphdr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<tcphdr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(tcphdr__bindgen_ty_1))
    );
}
#[test]
fn bindgen_test_layout_tcphdr() {
    assert_eq!(
        ::std::mem::size_of::<tcphdr>(),
        20usize,
        concat!("Size of: ", stringify!(tcphdr))
    );
    assert_eq!(
        ::std::mem::align_of::<tcphdr>(),
        4usize,
        concat!("Alignment of ", stringify!(tcphdr))
    );
}
pub const TCP_ESTABLISHED: _bindgen_ty_11 = 1;
pub const TCP_SYN_SENT: _bindgen_ty_11 = 2;
pub const TCP_SYN_RECV: _bindgen_ty_11 = 3;
pub const TCP_FIN_WAIT1: _bindgen_ty_11 = 4;
pub const TCP_FIN_WAIT2: _bindgen_ty_11 = 5;
pub const TCP_TIME_WAIT: _bindgen_ty_11 = 6;
pub const TCP_CLOSE: _bindgen_ty_11 = 7;
pub const TCP_CLOSE_WAIT: _bindgen_ty_11 = 8;
pub const TCP_LAST_ACK: _bindgen_ty_11 = 9;
pub const TCP_LISTEN: _bindgen_ty_11 = 10;
pub const TCP_CLOSING: _bindgen_ty_11 = 11;
pub type _bindgen_ty_11 = u32;
pub const tcp_ca_state_TCP_CA_Open: tcp_ca_state = 0;
pub const tcp_ca_state_TCP_CA_Disorder: tcp_ca_state = 1;
pub const tcp_ca_state_TCP_CA_CWR: tcp_ca_state = 2;
pub const tcp_ca_state_TCP_CA_Recovery: tcp_ca_state = 3;
pub const tcp_ca_state_TCP_CA_Loss: tcp_ca_state = 4;
pub type tcp_ca_state = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_info {
    pub tcpi_state: u8,
    pub tcpi_ca_state: u8,
    pub tcpi_retransmits: u8,
    pub tcpi_probes: u8,
    pub tcpi_backoff: u8,
    pub tcpi_options: u8,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub tcpi_rto: u32,
    pub tcpi_ato: u32,
    pub tcpi_snd_mss: u32,
    pub tcpi_rcv_mss: u32,
    pub tcpi_unacked: u32,
    pub tcpi_sacked: u32,
    pub tcpi_lost: u32,
    pub tcpi_retrans: u32,
    pub tcpi_fackets: u32,
    pub tcpi_last_data_sent: u32,
    pub tcpi_last_ack_sent: u32,
    pub tcpi_last_data_recv: u32,
    pub tcpi_last_ack_recv: u32,
    pub tcpi_pmtu: u32,
    pub tcpi_rcv_ssthresh: u32,
    pub tcpi_rtt: u32,
    pub tcpi_rttvar: u32,
    pub tcpi_snd_ssthresh: u32,
    pub tcpi_snd_cwnd: u32,
    pub tcpi_advmss: u32,
    pub tcpi_reordering: u32,
    pub tcpi_rcv_rtt: u32,
    pub tcpi_rcv_space: u32,
    pub tcpi_total_retrans: u32,
}
#[test]
fn bindgen_test_layout_tcp_info() {
    assert_eq!(
        ::std::mem::size_of::<tcp_info>(),
        104usize,
        concat!("Size of: ", stringify!(tcp_info))
    );
    assert_eq!(
        ::std::mem::align_of::<tcp_info>(),
        4usize,
        concat!("Alignment of ", stringify!(tcp_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_ca_state as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_ca_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_retransmits as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_retransmits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_probes as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_probes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_backoff as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_backoff)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_options as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rto as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rto)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_ato as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_ato)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_snd_mss as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_snd_mss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rcv_mss as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rcv_mss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_unacked as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_unacked)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_sacked as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_sacked)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_lost as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_lost)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_retrans as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_retrans)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_fackets as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_fackets)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_last_data_sent as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_last_data_sent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_last_ack_sent as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_last_ack_sent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_last_data_recv as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_last_data_recv)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_last_ack_recv as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_last_ack_recv)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_pmtu as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_pmtu)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rcv_ssthresh as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rcv_ssthresh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rtt as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rtt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rttvar as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rttvar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_snd_ssthresh as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_snd_ssthresh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_snd_cwnd as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_snd_cwnd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_advmss as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_advmss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_reordering as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_reordering)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rcv_rtt as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rcv_rtt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_rcv_space as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_rcv_space)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_info>())).tcpi_total_retrans as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_info),
            "::",
            stringify!(tcpi_total_retrans)
        )
    );
}
impl tcp_info {
    #[inline]
    pub fn tcpi_snd_wscale(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_tcpi_snd_wscale(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn tcpi_rcv_wscale(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_tcpi_rcv_wscale(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        tcpi_snd_wscale: u8,
        tcpi_rcv_wscale: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let tcpi_snd_wscale: u8 = unsafe { ::std::mem::transmute(tcpi_snd_wscale) };
            tcpi_snd_wscale as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let tcpi_rcv_wscale: u8 = unsafe { ::std::mem::transmute(tcpi_rcv_wscale) };
            tcpi_rcv_wscale as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_md5sig {
    pub tcpm_addr: sockaddr_storage,
    pub tcpm_flags: u8,
    pub tcpm_prefixlen: u8,
    pub tcpm_keylen: u16,
    pub __tcpm_pad: u32,
    pub tcpm_key: [u8; 80usize],
}
#[test]
fn bindgen_test_layout_tcp_md5sig() {
    assert_eq!(
        ::std::mem::size_of::<tcp_md5sig>(),
        216usize,
        concat!("Size of: ", stringify!(tcp_md5sig))
    );
    assert_eq!(
        ::std::mem::align_of::<tcp_md5sig>(),
        8usize,
        concat!("Alignment of ", stringify!(tcp_md5sig))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_md5sig>())).tcpm_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_md5sig),
            "::",
            stringify!(tcpm_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_md5sig>())).tcpm_flags as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_md5sig),
            "::",
            stringify!(tcpm_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_md5sig>())).tcpm_prefixlen as *const _ as usize },
        129usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_md5sig),
            "::",
            stringify!(tcpm_prefixlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_md5sig>())).tcpm_keylen as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_md5sig),
            "::",
            stringify!(tcpm_keylen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_md5sig>())).__tcpm_pad as *const _ as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_md5sig),
            "::",
            stringify!(__tcpm_pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_md5sig>())).tcpm_key as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_md5sig),
            "::",
            stringify!(tcpm_key)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_repair_opt {
    pub opt_code: u32,
    pub opt_val: u32,
}
#[test]
fn bindgen_test_layout_tcp_repair_opt() {
    assert_eq!(
        ::std::mem::size_of::<tcp_repair_opt>(),
        8usize,
        concat!("Size of: ", stringify!(tcp_repair_opt))
    );
    assert_eq!(
        ::std::mem::align_of::<tcp_repair_opt>(),
        4usize,
        concat!("Alignment of ", stringify!(tcp_repair_opt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_opt>())).opt_code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_opt),
            "::",
            stringify!(opt_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_opt>())).opt_val as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_opt),
            "::",
            stringify!(opt_val)
        )
    );
}
pub const TCP_NO_QUEUE: _bindgen_ty_12 = 0;
pub const TCP_RECV_QUEUE: _bindgen_ty_12 = 1;
pub const TCP_SEND_QUEUE: _bindgen_ty_12 = 2;
pub const TCP_QUEUES_NR: _bindgen_ty_12 = 3;
pub type _bindgen_ty_12 = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_cookie_transactions {
    pub tcpct_flags: u16,
    pub __tcpct_pad1: u8,
    pub tcpct_cookie_desired: u8,
    pub tcpct_s_data_desired: u16,
    pub tcpct_used: u16,
    pub tcpct_value: [u8; 536usize],
}
#[test]
fn bindgen_test_layout_tcp_cookie_transactions() {
    assert_eq!(
        ::std::mem::size_of::<tcp_cookie_transactions>(),
        544usize,
        concat!("Size of: ", stringify!(tcp_cookie_transactions))
    );
    assert_eq!(
        ::std::mem::align_of::<tcp_cookie_transactions>(),
        2usize,
        concat!("Alignment of ", stringify!(tcp_cookie_transactions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcp_cookie_transactions>())).tcpct_flags as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_cookie_transactions),
            "::",
            stringify!(tcpct_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcp_cookie_transactions>())).__tcpct_pad1 as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_cookie_transactions),
            "::",
            stringify!(__tcpct_pad1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcp_cookie_transactions>())).tcpct_cookie_desired as *const _
                as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_cookie_transactions),
            "::",
            stringify!(tcpct_cookie_desired)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcp_cookie_transactions>())).tcpct_s_data_desired as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_cookie_transactions),
            "::",
            stringify!(tcpct_s_data_desired)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcp_cookie_transactions>())).tcpct_used as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_cookie_transactions),
            "::",
            stringify!(tcpct_used)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tcp_cookie_transactions>())).tcpct_value as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_cookie_transactions),
            "::",
            stringify!(tcpct_value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_repair_window {
    pub snd_wl1: u32,
    pub snd_wnd: u32,
    pub max_window: u32,
    pub rcv_wnd: u32,
    pub rcv_wup: u32,
}
#[test]
fn bindgen_test_layout_tcp_repair_window() {
    assert_eq!(
        ::std::mem::size_of::<tcp_repair_window>(),
        20usize,
        concat!("Size of: ", stringify!(tcp_repair_window))
    );
    assert_eq!(
        ::std::mem::align_of::<tcp_repair_window>(),
        4usize,
        concat!("Alignment of ", stringify!(tcp_repair_window))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_window>())).snd_wl1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_window),
            "::",
            stringify!(snd_wl1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_window>())).snd_wnd as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_window),
            "::",
            stringify!(snd_wnd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_window>())).max_window as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_window),
            "::",
            stringify!(max_window)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_window>())).rcv_wnd as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_window),
            "::",
            stringify!(rcv_wnd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tcp_repair_window>())).rcv_wup as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tcp_repair_window),
            "::",
            stringify!(rcv_wup)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::std::os::raw::c_char; 108usize],
}
#[test]
fn bindgen_test_layout_sockaddr_un() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_un>(),
        110usize,
        concat!("Size of: ", stringify!(sockaddr_un))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_un>(),
        2usize,
        concat!("Alignment of ", stringify!(sockaddr_un))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_un>())).sun_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_un),
            "::",
            stringify!(sun_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_un>())).sun_path as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_un),
            "::",
            stringify!(sun_path)
        )
    );
}
pub type mongoc_socklen_t = socklen_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_socket_t {
    _unused: [u8; 0],
}
pub type mongoc_socket_t = _mongoc_socket_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_socket_poll_t {
    pub socket: *mut mongoc_socket_t,
    pub events: ::std::os::raw::c_int,
    pub revents: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mongoc_socket_poll_t() {
    assert_eq!(
        ::std::mem::size_of::<mongoc_socket_poll_t>(),
        16usize,
        concat!("Size of: ", stringify!(mongoc_socket_poll_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mongoc_socket_poll_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mongoc_socket_poll_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_socket_poll_t>())).socket as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_socket_poll_t),
            "::",
            stringify!(socket)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_socket_poll_t>())).events as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_socket_poll_t),
            "::",
            stringify!(events)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mongoc_socket_poll_t>())).revents as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(mongoc_socket_poll_t),
            "::",
            stringify!(revents)
        )
    );
}
extern "C" {
    pub fn mongoc_socket_accept(sock: *mut mongoc_socket_t, expire_at: i64)
        -> *mut mongoc_socket_t;
}
extern "C" {
    pub fn mongoc_socket_bind(
        sock: *mut mongoc_socket_t,
        addr: *const sockaddr,
        addrlen: mongoc_socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_close(socket: *mut mongoc_socket_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_connect(
        sock: *mut mongoc_socket_t,
        addr: *const sockaddr,
        addrlen: mongoc_socklen_t,
        expire_at: i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_getnameinfo(sock: *mut mongoc_socket_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_socket_destroy(sock: *mut mongoc_socket_t);
}
extern "C" {
    pub fn mongoc_socket_errno(sock: *mut mongoc_socket_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_getsockname(
        sock: *mut mongoc_socket_t,
        addr: *mut sockaddr,
        addrlen: *mut mongoc_socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_listen(
        sock: *mut mongoc_socket_t,
        backlog: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_new(
        domain: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        protocol: ::std::os::raw::c_int,
    ) -> *mut mongoc_socket_t;
}
extern "C" {
    pub fn mongoc_socket_recv(
        sock: *mut mongoc_socket_t,
        buf: *mut ::std::os::raw::c_void,
        buflen: usize,
        flags: ::std::os::raw::c_int,
        expire_at: i64,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_socket_setsockopt(
        sock: *mut mongoc_socket_t,
        level: ::std::os::raw::c_int,
        optname: ::std::os::raw::c_int,
        optval: *const ::std::os::raw::c_void,
        optlen: mongoc_socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_socket_send(
        sock: *mut mongoc_socket_t,
        buf: *const ::std::os::raw::c_void,
        buflen: usize,
        expire_at: i64,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_socket_sendv(
        sock: *mut mongoc_socket_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        expire_at: i64,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_socket_check_closed(sock: *mut mongoc_socket_t) -> bool;
}
extern "C" {
    pub fn mongoc_socket_inet_ntop(
        rp: *mut addrinfo,
        buf: *mut ::std::os::raw::c_char,
        buflen: usize,
    );
}
extern "C" {
    pub fn mongoc_socket_poll(sds: *mut mongoc_socket_poll_t, nsds: usize, timeout: i32) -> isize;
}
pub type mongoc_stream_t = _mongoc_stream_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_stream_poll_t {
    pub stream: *mut mongoc_stream_t,
    pub events: ::std::os::raw::c_int,
    pub revents: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__mongoc_stream_poll_t() {
    assert_eq!(
        ::std::mem::size_of::<_mongoc_stream_poll_t>(),
        16usize,
        concat!("Size of: ", stringify!(_mongoc_stream_poll_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_mongoc_stream_poll_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_mongoc_stream_poll_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_poll_t>())).stream as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_poll_t),
            "::",
            stringify!(stream)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_poll_t>())).events as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_poll_t),
            "::",
            stringify!(events)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_poll_t>())).revents as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_poll_t),
            "::",
            stringify!(revents)
        )
    );
}
pub type mongoc_stream_poll_t = _mongoc_stream_poll_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_stream_t {
    pub type_: ::std::os::raw::c_int,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t)>,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> ::std::os::raw::c_int,
    >,
    pub flush: ::std::option::Option<
        unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> ::std::os::raw::c_int,
    >,
    pub writev: ::std::option::Option<
        unsafe extern "C" fn(
            stream: *mut mongoc_stream_t,
            iov: *mut mongoc_iovec_t,
            iovcnt: usize,
            timeout_msec: i32,
        ) -> isize,
    >,
    pub readv: ::std::option::Option<
        unsafe extern "C" fn(
            stream: *mut mongoc_stream_t,
            iov: *mut mongoc_iovec_t,
            iovcnt: usize,
            min_bytes: usize,
            timeout_msec: i32,
        ) -> isize,
    >,
    pub setsockopt: ::std::option::Option<
        unsafe extern "C" fn(
            stream: *mut mongoc_stream_t,
            level: ::std::os::raw::c_int,
            optname: ::std::os::raw::c_int,
            optval: *mut ::std::os::raw::c_void,
            optlen: mongoc_socklen_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_base_stream: ::std::option::Option<
        unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> *mut mongoc_stream_t,
    >,
    pub check_closed:
        ::std::option::Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> bool>,
    pub poll: ::std::option::Option<
        unsafe extern "C" fn(streams: *mut mongoc_stream_poll_t, nstreams: usize, timeout: i32)
            -> isize,
    >,
    pub failed: ::std::option::Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t)>,
    pub timed_out:
        ::std::option::Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> bool>,
    pub padding: [*mut ::std::os::raw::c_void; 4usize],
}
#[test]
fn bindgen_test_layout__mongoc_stream_t() {
    assert_eq!(
        ::std::mem::size_of::<_mongoc_stream_t>(),
        128usize,
        concat!("Size of: ", stringify!(_mongoc_stream_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_mongoc_stream_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_mongoc_stream_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).destroy as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(destroy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).close as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(close)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).flush as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(flush)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).writev as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(writev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).readv as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(readv)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).setsockopt as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(setsockopt)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_stream_t>())).get_base_stream as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(get_base_stream)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).check_closed as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(check_closed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).poll as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(poll)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).failed as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(failed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).timed_out as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(timed_out)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_stream_t>())).padding as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_stream_t),
            "::",
            stringify!(padding)
        )
    );
}
extern "C" {
    pub fn mongoc_stream_get_base_stream(stream: *mut mongoc_stream_t) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_get_tls_stream(stream: *mut mongoc_stream_t) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_close(stream: *mut mongoc_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_stream_destroy(stream: *mut mongoc_stream_t);
}
extern "C" {
    pub fn mongoc_stream_failed(stream: *mut mongoc_stream_t);
}
extern "C" {
    pub fn mongoc_stream_flush(stream: *mut mongoc_stream_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_stream_writev(
        stream: *mut mongoc_stream_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        timeout_msec: i32,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_stream_write(
        stream: *mut mongoc_stream_t,
        buf: *mut ::std::os::raw::c_void,
        count: usize,
        timeout_msec: i32,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_stream_readv(
        stream: *mut mongoc_stream_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        min_bytes: usize,
        timeout_msec: i32,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_stream_read(
        stream: *mut mongoc_stream_t,
        buf: *mut ::std::os::raw::c_void,
        count: usize,
        min_bytes: usize,
        timeout_msec: i32,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_stream_setsockopt(
        stream: *mut mongoc_stream_t,
        level: ::std::os::raw::c_int,
        optname: ::std::os::raw::c_int,
        optval: *mut ::std::os::raw::c_void,
        optlen: mongoc_socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_stream_check_closed(stream: *mut mongoc_stream_t) -> bool;
}
extern "C" {
    pub fn mongoc_stream_timed_out(stream: *mut mongoc_stream_t) -> bool;
}
extern "C" {
    pub fn mongoc_stream_poll(
        streams: *mut mongoc_stream_poll_t,
        nstreams: usize,
        timeout: i32,
    ) -> isize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_file_t = _mongoc_gridfs_file_t;
pub type mongoc_gridfs_file_opt_t = _mongoc_gridfs_file_opt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_opt_t {
    pub md5: *const ::std::os::raw::c_char,
    pub filename: *const ::std::os::raw::c_char,
    pub content_type: *const ::std::os::raw::c_char,
    pub aliases: *const bson_t,
    pub metadata: *const bson_t,
    pub chunk_size: u32,
}
#[test]
fn bindgen_test_layout__mongoc_gridfs_file_opt_t() {
    assert_eq!(
        ::std::mem::size_of::<_mongoc_gridfs_file_opt_t>(),
        48usize,
        concat!("Size of: ", stringify!(_mongoc_gridfs_file_opt_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_mongoc_gridfs_file_opt_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_mongoc_gridfs_file_opt_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_gridfs_file_opt_t>())).md5 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_gridfs_file_opt_t),
            "::",
            stringify!(md5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_gridfs_file_opt_t>())).filename as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_gridfs_file_opt_t),
            "::",
            stringify!(filename)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_gridfs_file_opt_t>())).content_type as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_gridfs_file_opt_t),
            "::",
            stringify!(content_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_gridfs_file_opt_t>())).aliases as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_gridfs_file_opt_t),
            "::",
            stringify!(aliases)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_gridfs_file_opt_t>())).metadata as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_gridfs_file_opt_t),
            "::",
            stringify!(metadata)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_gridfs_file_opt_t>())).chunk_size as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_gridfs_file_opt_t),
            "::",
            stringify!(chunk_size)
        )
    );
}
extern "C" {
    pub fn mongoc_gridfs_file_get_md5(
        file: *mut mongoc_gridfs_file_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_gridfs_file_set_md5(
        file: *mut mongoc_gridfs_file_t,
        str: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mongoc_gridfs_file_get_filename(
        file: *mut mongoc_gridfs_file_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_gridfs_file_set_filename(
        file: *mut mongoc_gridfs_file_t,
        str: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mongoc_gridfs_file_get_content_type(
        file: *mut mongoc_gridfs_file_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_gridfs_file_set_content_type(
        file: *mut mongoc_gridfs_file_t,
        str: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mongoc_gridfs_file_get_aliases(file: *mut mongoc_gridfs_file_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_gridfs_file_set_aliases(file: *mut mongoc_gridfs_file_t, bson: *const bson_t);
}
extern "C" {
    pub fn mongoc_gridfs_file_get_metadata(file: *mut mongoc_gridfs_file_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_gridfs_file_set_metadata(file: *mut mongoc_gridfs_file_t, bson: *const bson_t);
}
extern "C" {
    pub fn mongoc_gridfs_file_get_id(file: *mut mongoc_gridfs_file_t) -> *const bson_value_t;
}
extern "C" {
    pub fn mongoc_gridfs_file_get_length(file: *mut mongoc_gridfs_file_t) -> i64;
}
extern "C" {
    pub fn mongoc_gridfs_file_get_chunk_size(file: *mut mongoc_gridfs_file_t) -> i32;
}
extern "C" {
    pub fn mongoc_gridfs_file_get_upload_date(file: *mut mongoc_gridfs_file_t) -> i64;
}
extern "C" {
    pub fn mongoc_gridfs_file_writev(
        file: *mut mongoc_gridfs_file_t,
        iov: *const mongoc_iovec_t,
        iovcnt: usize,
        timeout_msec: u32,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_gridfs_file_readv(
        file: *mut mongoc_gridfs_file_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        min_bytes: usize,
        timeout_msec: u32,
    ) -> isize;
}
extern "C" {
    pub fn mongoc_gridfs_file_seek(
        file: *mut mongoc_gridfs_file_t,
        delta: i64,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_gridfs_file_tell(file: *mut mongoc_gridfs_file_t) -> u64;
}
extern "C" {
    pub fn mongoc_gridfs_file_set_id(
        file: *mut mongoc_gridfs_file_t,
        id: *const bson_value_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_gridfs_file_save(file: *mut mongoc_gridfs_file_t) -> bool;
}
extern "C" {
    pub fn mongoc_gridfs_file_destroy(file: *mut mongoc_gridfs_file_t);
}
extern "C" {
    pub fn mongoc_gridfs_file_error(
        file: *mut mongoc_gridfs_file_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_gridfs_file_remove(
        file: *mut mongoc_gridfs_file_t,
        error: *mut bson_error_t,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_list_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_file_list_t = _mongoc_gridfs_file_list_t;
extern "C" {
    pub fn mongoc_gridfs_file_list_next(
        list: *mut mongoc_gridfs_file_list_t,
    ) -> *mut mongoc_gridfs_file_t;
}
extern "C" {
    pub fn mongoc_gridfs_file_list_destroy(list: *mut mongoc_gridfs_file_list_t);
}
extern "C" {
    pub fn mongoc_gridfs_file_list_error(
        list: *mut mongoc_gridfs_file_list_t,
        error: *mut bson_error_t,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_t = _mongoc_gridfs_t;
extern "C" {
    pub fn mongoc_gridfs_create_file_from_stream(
        gridfs: *mut mongoc_gridfs_t,
        stream: *mut mongoc_stream_t,
        opt: *mut mongoc_gridfs_file_opt_t,
    ) -> *mut mongoc_gridfs_file_t;
}
extern "C" {
    pub fn mongoc_gridfs_create_file(
        gridfs: *mut mongoc_gridfs_t,
        opt: *mut mongoc_gridfs_file_opt_t,
    ) -> *mut mongoc_gridfs_file_t;
}
extern "C" {
    pub fn mongoc_gridfs_find(
        gridfs: *mut mongoc_gridfs_t,
        query: *const bson_t,
    ) -> *mut mongoc_gridfs_file_list_t;
}
extern "C" {
    pub fn mongoc_gridfs_find_one(
        gridfs: *mut mongoc_gridfs_t,
        query: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_file_t;
}
extern "C" {
    pub fn mongoc_gridfs_find_with_opts(
        gridfs: *mut mongoc_gridfs_t,
        filter: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_gridfs_file_list_t;
}
extern "C" {
    pub fn mongoc_gridfs_find_one_with_opts(
        gridfs: *mut mongoc_gridfs_t,
        filter: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_file_t;
}
extern "C" {
    pub fn mongoc_gridfs_find_one_by_filename(
        gridfs: *mut mongoc_gridfs_t,
        filename: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_file_t;
}
extern "C" {
    pub fn mongoc_gridfs_drop(gridfs: *mut mongoc_gridfs_t, error: *mut bson_error_t) -> bool;
}
extern "C" {
    pub fn mongoc_gridfs_destroy(gridfs: *mut mongoc_gridfs_t);
}
extern "C" {
    pub fn mongoc_gridfs_get_files(gridfs: *mut mongoc_gridfs_t) -> *mut mongoc_collection_t;
}
extern "C" {
    pub fn mongoc_gridfs_get_chunks(gridfs: *mut mongoc_gridfs_t) -> *mut mongoc_collection_t;
}
extern "C" {
    pub fn mongoc_gridfs_remove_by_filename(
        gridfs: *mut mongoc_gridfs_t,
        filename: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> bool;
}
pub type mongoc_ssl_opt_t = _mongoc_ssl_opt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_ssl_opt_t {
    pub pem_file: *const ::std::os::raw::c_char,
    pub pem_pwd: *const ::std::os::raw::c_char,
    pub ca_file: *const ::std::os::raw::c_char,
    pub ca_dir: *const ::std::os::raw::c_char,
    pub crl_file: *const ::std::os::raw::c_char,
    pub weak_cert_validation: bool,
    pub allow_invalid_hostname: bool,
    pub padding: [*mut ::std::os::raw::c_void; 7usize],
}
#[test]
fn bindgen_test_layout__mongoc_ssl_opt_t() {
    assert_eq!(
        ::std::mem::size_of::<_mongoc_ssl_opt_t>(),
        104usize,
        concat!("Size of: ", stringify!(_mongoc_ssl_opt_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_mongoc_ssl_opt_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_mongoc_ssl_opt_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).pem_file as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(pem_file)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).pem_pwd as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(pem_pwd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).ca_file as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(ca_file)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).ca_dir as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(ca_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).crl_file as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(crl_file)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).weak_cert_validation as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(weak_cert_validation)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).allow_invalid_hostname as *const _
                as usize
        },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(allow_invalid_hostname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_mongoc_ssl_opt_t>())).padding as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_mongoc_ssl_opt_t),
            "::",
            stringify!(padding)
        )
    );
}
extern "C" {
    pub fn mongoc_ssl_opt_get_default() -> *const mongoc_ssl_opt_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_uri_t {
    _unused: [u8; 0],
}
pub type mongoc_uri_t = _mongoc_uri_t;
extern "C" {
    pub fn mongoc_uri_copy(uri: *const mongoc_uri_t) -> *mut mongoc_uri_t;
}
extern "C" {
    pub fn mongoc_uri_destroy(uri: *mut mongoc_uri_t);
}
extern "C" {
    pub fn mongoc_uri_new(uri_string: *const ::std::os::raw::c_char) -> *mut mongoc_uri_t;
}
extern "C" {
    pub fn mongoc_uri_new_with_error(
        uri_string: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> *mut mongoc_uri_t;
}
extern "C" {
    pub fn mongoc_uri_new_for_host_port(
        hostname: *const ::std::os::raw::c_char,
        port: u16,
    ) -> *mut mongoc_uri_t;
}
extern "C" {
    pub fn mongoc_uri_get_hosts(uri: *const mongoc_uri_t) -> *const mongoc_host_list_t;
}
extern "C" {
    pub fn mongoc_uri_get_service(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_get_database(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_database(
        uri: *mut mongoc_uri_t,
        database: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_compressors(uri: *const mongoc_uri_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_uri_get_options(uri: *const mongoc_uri_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_uri_get_password(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_password(
        uri: *mut mongoc_uri_t,
        password: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_option_is_int32(key: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn mongoc_uri_option_is_bool(key: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn mongoc_uri_option_is_utf8(key: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_option_as_int32(
        uri: *const mongoc_uri_t,
        option: *const ::std::os::raw::c_char,
        fallback: i32,
    ) -> i32;
}
extern "C" {
    pub fn mongoc_uri_get_option_as_bool(
        uri: *const mongoc_uri_t,
        option: *const ::std::os::raw::c_char,
        fallback: bool,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_option_as_utf8(
        uri: *const mongoc_uri_t,
        option: *const ::std::os::raw::c_char,
        fallback: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_option_as_int32(
        uri: *mut mongoc_uri_t,
        option: *const ::std::os::raw::c_char,
        value: i32,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_set_option_as_bool(
        uri: *mut mongoc_uri_t,
        option: *const ::std::os::raw::c_char,
        value: bool,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_set_option_as_utf8(
        uri: *mut mongoc_uri_t,
        option: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_read_prefs(uri: *const mongoc_uri_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_uri_get_replica_set(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_get_string(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_get_username(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_username(
        uri: *mut mongoc_uri_t,
        username: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_credentials(uri: *const mongoc_uri_t) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_uri_get_auth_source(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_auth_source(
        uri: *mut mongoc_uri_t,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_appname(uri: *const mongoc_uri_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_appname(
        uri: *mut mongoc_uri_t,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_set_compressors(
        uri: *mut mongoc_uri_t,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_auth_mechanism(uri: *const mongoc_uri_t)
        -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_set_auth_mechanism(
        uri: *mut mongoc_uri_t,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_mechanism_properties(
        uri: *const mongoc_uri_t,
        properties: *mut bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_set_mechanism_properties(
        uri: *mut mongoc_uri_t,
        properties: *const bson_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_uri_get_ssl(uri: *const mongoc_uri_t) -> bool;
}
extern "C" {
    pub fn mongoc_uri_unescape(
        escaped_string: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_uri_get_read_prefs_t(uri: *const mongoc_uri_t) -> *const mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_uri_set_read_prefs_t(uri: *mut mongoc_uri_t, prefs: *const mongoc_read_prefs_t);
}
extern "C" {
    pub fn mongoc_uri_get_write_concern(uri: *const mongoc_uri_t) -> *const mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_uri_set_write_concern(uri: *mut mongoc_uri_t, wc: *const mongoc_write_concern_t);
}
extern "C" {
    pub fn mongoc_uri_get_read_concern(uri: *const mongoc_uri_t) -> *const mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_uri_set_read_concern(uri: *mut mongoc_uri_t, rc: *const mongoc_read_concern_t);
}
pub type mongoc_client_t = _mongoc_client_t;
pub type mongoc_client_session_t = _mongoc_client_session_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_session_opt_t {
    _unused: [u8; 0],
}
pub type mongoc_session_opt_t = _mongoc_session_opt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_transaction_opt_t {
    _unused: [u8; 0],
}
pub type mongoc_transaction_opt_t = _mongoc_transaction_opt_t;
pub type mongoc_stream_initiator_t = ::std::option::Option<
    unsafe extern "C" fn(
        uri: *const mongoc_uri_t,
        host: *const mongoc_host_list_t,
        user_data: *mut ::std::os::raw::c_void,
        error: *mut bson_error_t,
    ) -> *mut mongoc_stream_t,
>;
extern "C" {
    pub fn mongoc_client_new(uri_string: *const ::std::os::raw::c_char) -> *mut mongoc_client_t;
}
extern "C" {
    pub fn mongoc_client_new_from_uri(uri: *const mongoc_uri_t) -> *mut mongoc_client_t;
}
extern "C" {
    pub fn mongoc_client_get_uri(client: *const mongoc_client_t) -> *const mongoc_uri_t;
}
extern "C" {
    pub fn mongoc_client_set_stream_initiator(
        client: *mut mongoc_client_t,
        initiator: mongoc_stream_initiator_t,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mongoc_client_command(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        query: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_client_kill_cursor(client: *mut mongoc_client_t, cursor_id: i64);
}
extern "C" {
    pub fn mongoc_client_command_simple(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_read_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_write_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        command: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_read_write_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_command_simple_with_server_id(
        client: *mut mongoc_client_t,
        db_name: *const ::std::os::raw::c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        server_id: u32,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_destroy(client: *mut mongoc_client_t);
}
extern "C" {
    pub fn mongoc_client_start_session(
        client: *mut mongoc_client_t,
        opts: *const mongoc_session_opt_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_client_session_t;
}
extern "C" {
    pub fn mongoc_client_get_database(
        client: *mut mongoc_client_t,
        name: *const ::std::os::raw::c_char,
    ) -> *mut mongoc_database_t;
}
extern "C" {
    pub fn mongoc_client_get_default_database(
        client: *mut mongoc_client_t,
    ) -> *mut mongoc_database_t;
}
extern "C" {
    pub fn mongoc_client_get_gridfs(
        client: *mut mongoc_client_t,
        db: *const ::std::os::raw::c_char,
        prefix: *const ::std::os::raw::c_char,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_t;
}
extern "C" {
    pub fn mongoc_client_get_collection(
        client: *mut mongoc_client_t,
        db: *const ::std::os::raw::c_char,
        collection: *const ::std::os::raw::c_char,
    ) -> *mut mongoc_collection_t;
}
extern "C" {
    pub fn mongoc_client_get_database_names(
        client: *mut mongoc_client_t,
        error: *mut bson_error_t,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_client_get_database_names_with_opts(
        client: *mut mongoc_client_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_client_find_databases(
        client: *mut mongoc_client_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_client_find_databases_with_opts(
        client: *mut mongoc_client_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
}
extern "C" {
    pub fn mongoc_client_get_server_status(
        client: *mut mongoc_client_t,
        read_prefs: *mut mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_get_max_message_size(client: *mut mongoc_client_t) -> i32;
}
extern "C" {
    pub fn mongoc_client_get_max_bson_size(client: *mut mongoc_client_t) -> i32;
}
extern "C" {
    pub fn mongoc_client_get_write_concern(
        client: *const mongoc_client_t,
    ) -> *const mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_client_set_write_concern(
        client: *mut mongoc_client_t,
        write_concern: *const mongoc_write_concern_t,
    );
}
extern "C" {
    pub fn mongoc_client_get_read_concern(
        client: *const mongoc_client_t,
    ) -> *const mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_client_set_read_concern(
        client: *mut mongoc_client_t,
        read_concern: *const mongoc_read_concern_t,
    );
}
extern "C" {
    pub fn mongoc_client_get_read_prefs(
        client: *const mongoc_client_t,
    ) -> *const mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_client_set_read_prefs(
        client: *mut mongoc_client_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
}
extern "C" {
    pub fn mongoc_client_set_ssl_opts(client: *mut mongoc_client_t, opts: *const mongoc_ssl_opt_t);
}
extern "C" {
    pub fn mongoc_client_set_apm_callbacks(
        client: *mut mongoc_client_t,
        callbacks: *mut mongoc_apm_callbacks_t,
        context: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_get_server_description(
        client: *mut mongoc_client_t,
        server_id: u32,
    ) -> *mut mongoc_server_description_t;
}
extern "C" {
    pub fn mongoc_client_get_server_descriptions(
        client: *const mongoc_client_t,
        n: *mut usize,
    ) -> *mut *mut mongoc_server_description_t;
}
extern "C" {
    pub fn mongoc_server_descriptions_destroy_all(
        sds: *mut *mut mongoc_server_description_t,
        n: usize,
    );
}
extern "C" {
    pub fn mongoc_client_select_server(
        client: *mut mongoc_client_t,
        for_writes: bool,
        prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_server_description_t;
}
extern "C" {
    pub fn mongoc_client_set_error_api(client: *mut mongoc_client_t, version: i32) -> bool;
}
extern "C" {
    pub fn mongoc_client_set_appname(
        client: *mut mongoc_client_t,
        appname: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_watch(
        client: *mut mongoc_client_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_change_stream_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_client_pool_t {
    _unused: [u8; 0],
}
pub type mongoc_client_pool_t = _mongoc_client_pool_t;
extern "C" {
    pub fn mongoc_client_pool_new(uri: *const mongoc_uri_t) -> *mut mongoc_client_pool_t;
}
extern "C" {
    pub fn mongoc_client_pool_destroy(pool: *mut mongoc_client_pool_t);
}
extern "C" {
    pub fn mongoc_client_pool_pop(pool: *mut mongoc_client_pool_t) -> *mut mongoc_client_t;
}
extern "C" {
    pub fn mongoc_client_pool_push(pool: *mut mongoc_client_pool_t, client: *mut mongoc_client_t);
}
extern "C" {
    pub fn mongoc_client_pool_try_pop(pool: *mut mongoc_client_pool_t) -> *mut mongoc_client_t;
}
extern "C" {
    pub fn mongoc_client_pool_max_size(pool: *mut mongoc_client_pool_t, max_pool_size: u32);
}
extern "C" {
    pub fn mongoc_client_pool_min_size(pool: *mut mongoc_client_pool_t, min_pool_size: u32);
}
extern "C" {
    pub fn mongoc_client_pool_set_ssl_opts(
        pool: *mut mongoc_client_pool_t,
        opts: *const mongoc_ssl_opt_t,
    );
}
extern "C" {
    pub fn mongoc_client_pool_set_apm_callbacks(
        pool: *mut mongoc_client_pool_t,
        callbacks: *mut mongoc_apm_callbacks_t,
        context: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_pool_set_error_api(pool: *mut mongoc_client_pool_t, version: i32) -> bool;
}
extern "C" {
    pub fn mongoc_client_pool_set_appname(
        pool: *mut mongoc_client_pool_t,
        appname: *const ::std::os::raw::c_char,
    ) -> bool;
}
pub const mongoc_error_domain_t_MONGOC_ERROR_CLIENT: mongoc_error_domain_t = 1;
pub const mongoc_error_domain_t_MONGOC_ERROR_STREAM: mongoc_error_domain_t = 2;
pub const mongoc_error_domain_t_MONGOC_ERROR_PROTOCOL: mongoc_error_domain_t = 3;
pub const mongoc_error_domain_t_MONGOC_ERROR_CURSOR: mongoc_error_domain_t = 4;
pub const mongoc_error_domain_t_MONGOC_ERROR_QUERY: mongoc_error_domain_t = 5;
pub const mongoc_error_domain_t_MONGOC_ERROR_INSERT: mongoc_error_domain_t = 6;
pub const mongoc_error_domain_t_MONGOC_ERROR_SASL: mongoc_error_domain_t = 7;
pub const mongoc_error_domain_t_MONGOC_ERROR_BSON: mongoc_error_domain_t = 8;
pub const mongoc_error_domain_t_MONGOC_ERROR_MATCHER: mongoc_error_domain_t = 9;
pub const mongoc_error_domain_t_MONGOC_ERROR_NAMESPACE: mongoc_error_domain_t = 10;
pub const mongoc_error_domain_t_MONGOC_ERROR_COMMAND: mongoc_error_domain_t = 11;
pub const mongoc_error_domain_t_MONGOC_ERROR_COLLECTION: mongoc_error_domain_t = 12;
pub const mongoc_error_domain_t_MONGOC_ERROR_GRIDFS: mongoc_error_domain_t = 13;
pub const mongoc_error_domain_t_MONGOC_ERROR_SCRAM: mongoc_error_domain_t = 14;
pub const mongoc_error_domain_t_MONGOC_ERROR_SERVER_SELECTION: mongoc_error_domain_t = 15;
pub const mongoc_error_domain_t_MONGOC_ERROR_WRITE_CONCERN: mongoc_error_domain_t = 16;
pub const mongoc_error_domain_t_MONGOC_ERROR_SERVER: mongoc_error_domain_t = 17;
pub const mongoc_error_domain_t_MONGOC_ERROR_TRANSACTION: mongoc_error_domain_t = 18;
pub type mongoc_error_domain_t = u32;
pub const mongoc_error_code_t_MONGOC_ERROR_STREAM_INVALID_TYPE: mongoc_error_code_t = 1;
pub const mongoc_error_code_t_MONGOC_ERROR_STREAM_INVALID_STATE: mongoc_error_code_t = 2;
pub const mongoc_error_code_t_MONGOC_ERROR_STREAM_NAME_RESOLUTION: mongoc_error_code_t = 3;
pub const mongoc_error_code_t_MONGOC_ERROR_STREAM_SOCKET: mongoc_error_code_t = 4;
pub const mongoc_error_code_t_MONGOC_ERROR_STREAM_CONNECT: mongoc_error_code_t = 5;
pub const mongoc_error_code_t_MONGOC_ERROR_STREAM_NOT_ESTABLISHED: mongoc_error_code_t = 6;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_NOT_READY: mongoc_error_code_t = 7;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_TOO_BIG: mongoc_error_code_t = 8;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_TOO_SMALL: mongoc_error_code_t = 9;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_GETNONCE: mongoc_error_code_t = 10;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_AUTHENTICATE: mongoc_error_code_t = 11;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_NO_ACCEPTABLE_PEER: mongoc_error_code_t = 12;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_IN_EXHAUST: mongoc_error_code_t = 13;
pub const mongoc_error_code_t_MONGOC_ERROR_PROTOCOL_INVALID_REPLY: mongoc_error_code_t = 14;
pub const mongoc_error_code_t_MONGOC_ERROR_PROTOCOL_BAD_WIRE_VERSION: mongoc_error_code_t = 15;
pub const mongoc_error_code_t_MONGOC_ERROR_CURSOR_INVALID_CURSOR: mongoc_error_code_t = 16;
pub const mongoc_error_code_t_MONGOC_ERROR_QUERY_FAILURE: mongoc_error_code_t = 17;
pub const mongoc_error_code_t_MONGOC_ERROR_BSON_INVALID: mongoc_error_code_t = 18;
pub const mongoc_error_code_t_MONGOC_ERROR_MATCHER_INVALID: mongoc_error_code_t = 19;
pub const mongoc_error_code_t_MONGOC_ERROR_NAMESPACE_INVALID: mongoc_error_code_t = 20;
pub const mongoc_error_code_t_MONGOC_ERROR_NAMESPACE_INVALID_FILTER_TYPE: mongoc_error_code_t = 21;
pub const mongoc_error_code_t_MONGOC_ERROR_COMMAND_INVALID_ARG: mongoc_error_code_t = 22;
pub const mongoc_error_code_t_MONGOC_ERROR_COLLECTION_INSERT_FAILED: mongoc_error_code_t = 23;
pub const mongoc_error_code_t_MONGOC_ERROR_COLLECTION_UPDATE_FAILED: mongoc_error_code_t = 24;
pub const mongoc_error_code_t_MONGOC_ERROR_COLLECTION_DELETE_FAILED: mongoc_error_code_t = 25;
pub const mongoc_error_code_t_MONGOC_ERROR_COLLECTION_DOES_NOT_EXIST: mongoc_error_code_t = 26;
pub const mongoc_error_code_t_MONGOC_ERROR_GRIDFS_INVALID_FILENAME: mongoc_error_code_t = 27;
pub const mongoc_error_code_t_MONGOC_ERROR_SCRAM_NOT_DONE: mongoc_error_code_t = 28;
pub const mongoc_error_code_t_MONGOC_ERROR_SCRAM_PROTOCOL_ERROR: mongoc_error_code_t = 29;
pub const mongoc_error_code_t_MONGOC_ERROR_QUERY_COMMAND_NOT_FOUND: mongoc_error_code_t = 59;
pub const mongoc_error_code_t_MONGOC_ERROR_QUERY_NOT_TAILABLE: mongoc_error_code_t = 13051;
pub const mongoc_error_code_t_MONGOC_ERROR_SERVER_SELECTION_BAD_WIRE_VERSION: mongoc_error_code_t =
    13052;
pub const mongoc_error_code_t_MONGOC_ERROR_SERVER_SELECTION_FAILURE: mongoc_error_code_t = 13053;
pub const mongoc_error_code_t_MONGOC_ERROR_SERVER_SELECTION_INVALID_ID: mongoc_error_code_t = 13054;
pub const mongoc_error_code_t_MONGOC_ERROR_GRIDFS_CHUNK_MISSING: mongoc_error_code_t = 13055;
pub const mongoc_error_code_t_MONGOC_ERROR_GRIDFS_PROTOCOL_ERROR: mongoc_error_code_t = 13056;
pub const mongoc_error_code_t_MONGOC_ERROR_PROTOCOL_ERROR: mongoc_error_code_t = 17;
pub const mongoc_error_code_t_MONGOC_ERROR_WRITE_CONCERN_ERROR: mongoc_error_code_t = 64;
pub const mongoc_error_code_t_MONGOC_ERROR_DUPLICATE_KEY: mongoc_error_code_t = 11000;
pub const mongoc_error_code_t_MONGOC_ERROR_CHANGE_STREAM_NO_RESUME_TOKEN: mongoc_error_code_t =
    11001;
pub const mongoc_error_code_t_MONGOC_ERROR_CLIENT_SESSION_FAILURE: mongoc_error_code_t = 11002;
pub const mongoc_error_code_t_MONGOC_ERROR_TRANSACTION_INVALID_STATE: mongoc_error_code_t = 11003;
pub const mongoc_error_code_t_MONGOC_ERROR_GRIDFS_CORRUPT: mongoc_error_code_t = 11004;
pub type mongoc_error_code_t = u32;
extern "C" {
    pub fn mongoc_error_has_label(
        reply: *const bson_t,
        label: *const ::std::os::raw::c_char,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_page_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_file_page_t = _mongoc_gridfs_file_page_t;
extern "C" {
    pub fn mongoc_init();
}
extern "C" {
    pub fn mongoc_cleanup();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_matcher_t {
    _unused: [u8; 0],
}
pub type mongoc_matcher_t = _mongoc_matcher_t;
extern "C" {
    pub fn mongoc_matcher_new(
        query: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_matcher_t;
}
extern "C" {
    pub fn mongoc_matcher_match(matcher: *const mongoc_matcher_t, document: *const bson_t) -> bool;
}
extern "C" {
    pub fn mongoc_matcher_destroy(matcher: *mut mongoc_matcher_t);
}
extern "C" {
    pub fn mongoc_handshake_data_append(
        driver_name: *const ::std::os::raw::c_char,
        driver_version: *const ::std::os::raw::c_char,
        platform: *const ::std::os::raw::c_char,
    ) -> bool;
}
pub const mongoc_opcode_t_MONGOC_OPCODE_REPLY: mongoc_opcode_t = 1;
pub const mongoc_opcode_t_MONGOC_OPCODE_UPDATE: mongoc_opcode_t = 2001;
pub const mongoc_opcode_t_MONGOC_OPCODE_INSERT: mongoc_opcode_t = 2002;
pub const mongoc_opcode_t_MONGOC_OPCODE_QUERY: mongoc_opcode_t = 2004;
pub const mongoc_opcode_t_MONGOC_OPCODE_GET_MORE: mongoc_opcode_t = 2005;
pub const mongoc_opcode_t_MONGOC_OPCODE_DELETE: mongoc_opcode_t = 2006;
pub const mongoc_opcode_t_MONGOC_OPCODE_KILL_CURSORS: mongoc_opcode_t = 2007;
pub const mongoc_opcode_t_MONGOC_OPCODE_COMPRESSED: mongoc_opcode_t = 2012;
pub const mongoc_opcode_t_MONGOC_OPCODE_MSG: mongoc_opcode_t = 2013;
pub type mongoc_opcode_t = u32;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_ERROR: mongoc_log_level_t = 0;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_CRITICAL: mongoc_log_level_t = 1;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_WARNING: mongoc_log_level_t = 2;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_MESSAGE: mongoc_log_level_t = 3;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_INFO: mongoc_log_level_t = 4;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_DEBUG: mongoc_log_level_t = 5;
pub const mongoc_log_level_t_MONGOC_LOG_LEVEL_TRACE: mongoc_log_level_t = 6;
pub type mongoc_log_level_t = u32;
pub type mongoc_log_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        log_level: mongoc_log_level_t,
        log_domain: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn mongoc_log_set_handler(
        log_func: mongoc_log_func_t,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mongoc_log(
        log_level: mongoc_log_level_t,
        log_domain: *const ::std::os::raw::c_char,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn mongoc_log_default_handler(
        log_level: mongoc_log_level_t,
        log_domain: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mongoc_log_level_str(log_level: mongoc_log_level_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_log_trace_enable();
}
extern "C" {
    pub fn mongoc_log_trace_disable();
}
extern "C" {
    pub fn mongoc_transaction_opts_new() -> *mut mongoc_transaction_opt_t;
}
extern "C" {
    pub fn mongoc_transaction_opts_clone(
        opts: *const mongoc_transaction_opt_t,
    ) -> *mut mongoc_transaction_opt_t;
}
extern "C" {
    pub fn mongoc_transaction_opts_destroy(opts: *mut mongoc_transaction_opt_t);
}
extern "C" {
    pub fn mongoc_transaction_opts_set_read_concern(
        opts: *mut mongoc_transaction_opt_t,
        read_concern: *const mongoc_read_concern_t,
    );
}
extern "C" {
    pub fn mongoc_transaction_opts_get_read_concern(
        opts: *const mongoc_transaction_opt_t,
    ) -> *const mongoc_read_concern_t;
}
extern "C" {
    pub fn mongoc_transaction_opts_set_write_concern(
        opts: *mut mongoc_transaction_opt_t,
        write_concern: *const mongoc_write_concern_t,
    );
}
extern "C" {
    pub fn mongoc_transaction_opts_get_write_concern(
        opts: *const mongoc_transaction_opt_t,
    ) -> *const mongoc_write_concern_t;
}
extern "C" {
    pub fn mongoc_transaction_opts_set_read_prefs(
        opts: *mut mongoc_transaction_opt_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
}
extern "C" {
    pub fn mongoc_transaction_opts_get_read_prefs(
        opts: *const mongoc_transaction_opt_t,
    ) -> *const mongoc_read_prefs_t;
}
extern "C" {
    pub fn mongoc_session_opts_new() -> *mut mongoc_session_opt_t;
}
extern "C" {
    pub fn mongoc_session_opts_set_causal_consistency(
        opts: *mut mongoc_session_opt_t,
        causal_consistency: bool,
    );
}
extern "C" {
    pub fn mongoc_session_opts_get_causal_consistency(opts: *const mongoc_session_opt_t) -> bool;
}
extern "C" {
    pub fn mongoc_session_opts_set_default_transaction_opts(
        opts: *mut mongoc_session_opt_t,
        txn_opts: *const mongoc_transaction_opt_t,
    );
}
extern "C" {
    pub fn mongoc_session_opts_get_default_transaction_opts(
        opts: *const mongoc_session_opt_t,
    ) -> *const mongoc_transaction_opt_t;
}
extern "C" {
    pub fn mongoc_session_opts_clone(
        opts: *const mongoc_session_opt_t,
    ) -> *mut mongoc_session_opt_t;
}
extern "C" {
    pub fn mongoc_session_opts_destroy(opts: *mut mongoc_session_opt_t);
}
extern "C" {
    pub fn mongoc_client_session_get_client(
        session: *const mongoc_client_session_t,
    ) -> *mut mongoc_client_t;
}
extern "C" {
    pub fn mongoc_client_session_get_opts(
        session: *const mongoc_client_session_t,
    ) -> *const mongoc_session_opt_t;
}
extern "C" {
    pub fn mongoc_client_session_get_lsid(session: *const mongoc_client_session_t)
        -> *const bson_t;
}
extern "C" {
    pub fn mongoc_client_session_get_cluster_time(
        session: *const mongoc_client_session_t,
    ) -> *const bson_t;
}
extern "C" {
    pub fn mongoc_client_session_advance_cluster_time(
        session: *mut mongoc_client_session_t,
        cluster_time: *const bson_t,
    );
}
extern "C" {
    pub fn mongoc_client_session_get_operation_time(
        session: *const mongoc_client_session_t,
        timestamp: *mut u32,
        increment: *mut u32,
    );
}
extern "C" {
    pub fn mongoc_client_session_advance_operation_time(
        session: *mut mongoc_client_session_t,
        timestamp: u32,
        increment: u32,
    );
}
extern "C" {
    pub fn mongoc_client_session_start_transaction(
        session: *mut mongoc_client_session_t,
        opts: *const mongoc_transaction_opt_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_session_in_transaction(session: *const mongoc_client_session_t) -> bool;
}
extern "C" {
    pub fn mongoc_client_session_commit_transaction(
        session: *mut mongoc_client_session_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_session_abort_transaction(
        session: *mut mongoc_client_session_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_session_append(
        client_session: *const mongoc_client_session_t,
        opts: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_client_session_destroy(session: *mut mongoc_client_session_t);
}
extern "C" {
    pub fn mongoc_stream_buffered_new(
        base_stream: *mut mongoc_stream_t,
        buffer_size: usize,
    ) -> *mut mongoc_stream_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_stream_file_t {
    _unused: [u8; 0],
}
pub type mongoc_stream_file_t = _mongoc_stream_file_t;
extern "C" {
    pub fn mongoc_stream_file_new(fd: ::std::os::raw::c_int) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_file_new_for_path(
        path: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
    ) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_file_get_fd(stream: *mut mongoc_stream_file_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_stream_gridfs_new(file: *mut mongoc_gridfs_file_t) -> *mut mongoc_stream_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_stream_socket_t {
    _unused: [u8; 0],
}
pub type mongoc_stream_socket_t = _mongoc_stream_socket_t;
extern "C" {
    pub fn mongoc_stream_socket_new(socket: *mut mongoc_socket_t) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_socket_get_socket(
        stream: *mut mongoc_stream_socket_t,
    ) -> *mut mongoc_socket_t;
}
extern "C" {
    pub fn mongoc_get_major_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_get_minor_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_get_micro_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mongoc_get_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mongoc_check_version(
        required_major: ::std::os::raw::c_int,
        required_minor: ::std::os::raw::c_int,
        required_micro: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_rand_seed(buf: *const ::std::os::raw::c_void, num: ::std::os::raw::c_int);
}
extern "C" {
    pub fn mongoc_rand_add(
        buf: *const ::std::os::raw::c_void,
        num: ::std::os::raw::c_int,
        entropy: f64,
    );
}
extern "C" {
    pub fn mongoc_rand_status() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_stream_tls_t {
    _unused: [u8; 0],
}
pub type mongoc_stream_tls_t = _mongoc_stream_tls_t;
extern "C" {
    pub fn mongoc_stream_tls_handshake(
        stream: *mut mongoc_stream_t,
        host: *const ::std::os::raw::c_char,
        timeout_msec: i32,
        events: *mut ::std::os::raw::c_int,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_stream_tls_handshake_block(
        stream: *mut mongoc_stream_t,
        host: *const ::std::os::raw::c_char,
        timeout_msec: i32,
        error: *mut bson_error_t,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_stream_tls_do_handshake(stream: *mut mongoc_stream_t, timeout_msec: i32) -> bool;
}
extern "C" {
    pub fn mongoc_stream_tls_check_cert(
        stream: *mut mongoc_stream_t,
        host: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn mongoc_stream_tls_new_with_hostname(
        base_stream: *mut mongoc_stream_t,
        host: *const ::std::os::raw::c_char,
        opt: *mut mongoc_ssl_opt_t,
        client: ::std::os::raw::c_int,
    ) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_tls_new(
        base_stream: *mut mongoc_stream_t,
        opt: *mut mongoc_ssl_opt_t,
        client: ::std::os::raw::c_int,
    ) -> *mut mongoc_stream_t;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
