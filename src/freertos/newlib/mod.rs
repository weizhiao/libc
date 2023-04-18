pub type dev_t = u32;
pub type ino_t = u32;
pub type off_t = i64;
pub type nlink_t = ::c_ushort;
pub type time_t = i32;

pub type blkcnt_t = i32;
pub type blksize_t = i32;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_spare1: ::c_long,
        pub st_mtime: ::time_t,
        pub st_spare2: ::c_long,
        pub st_ctime: ::time_t,
        pub st_spare3: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_spare4: [::c_long; 2usize],
    }

    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
    }
}

// Other constants
pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const EDEADLK: ::c_int = 45;
pub const ENOLCK: ::c_int = 46;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENOLINK: ::c_int = 67;
pub const EPROTO: ::c_int = 71;
pub const EMULTIHOP: ::c_int = 74;
pub const EBADMSG: ::c_int = 77;
pub const EFTYPE: ::c_int = 79;
pub const ENOSYS: ::c_int = 88;
pub const ENOTEMPTY: ::c_int = 90;
pub const ENAMETOOLONG: ::c_int = 91;
pub const ELOOP: ::c_int = 92;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EAFNOSUPPORT: ::c_int = 106;
pub const EPROTOTYPE: ::c_int = 107;
pub const ENOTSOCK: ::c_int = 108;
pub const ENOPROTOOPT: ::c_int = 109;
pub const ECONNREFUSED: ::c_int = 111;
pub const EADDRINUSE: ::c_int = 112;
pub const ECONNABORTED: ::c_int = 113;
pub const ENETUNREACH: ::c_int = 114;
pub const ENETDOWN: ::c_int = 115;
pub const ETIMEDOUT: ::c_int = 116;
pub const EHOSTDOWN: ::c_int = 117;
pub const EHOSTUNREACH: ::c_int = 118;
pub const EINPROGRESS: ::c_int = 119;
pub const EALREADY: ::c_int = 120;
pub const EDESTADDRREQ: ::c_int = 121;
pub const EMSGSIZE: ::c_int = 122;
pub const EPROTONOSUPPORT: ::c_int = 123;
pub const EADDRNOTAVAIL: ::c_int = 125;
pub const ENETRESET: ::c_int = 126;
pub const EISCONN: ::c_int = 127;
pub const ENOTCONN: ::c_int = 128;
pub const ETOOMANYREFS: ::c_int = 129;
pub const EDQUOT: ::c_int = 132;
pub const ESTALE: ::c_int = 133;
pub const ENOTSUP: ::c_int = 134;
pub const EILSEQ: ::c_int = 138;
pub const EOVERFLOW: ::c_int = 139;
pub const ECANCELED: ::c_int = 140;
pub const ENOTRECOVERABLE: ::c_int = 141;
pub const EOWNERDEAD: ::c_int = 142;
pub const EWOULDBLOCK: ::c_int = 11;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_GETOWN: ::c_int = 5;
pub const F_SETOWN: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_RGETLK: ::c_int = 10;
pub const F_RSETLK: ::c_int = 11;
pub const F_CNVT: ::c_int = 12;
pub const F_RSETLKW: ::c_int = 13;
pub const F_DUPFD_CLOEXEC: ::c_int = 14;

//文件操作
pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_APPEND: ::c_int = 8;
pub const O_CREAT: ::c_int = 512;
pub const O_TRUNC: ::c_int = 1024;
pub const O_EXCL: ::c_int = 2048;
pub const O_SYNC: ::c_int = 8192;
pub const O_NONBLOCK: ::c_int = 16384;

pub const O_ACCMODE: ::c_int = 3;
pub const O_CLOEXEC: ::c_int = 0x80000;

pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;

pub const S_BLKSIZE: ::mode_t = 1024;
pub const S_IREAD: ::mode_t = 256;
pub const S_IWRITE: ::mode_t = 128;
pub const S_IEXEC: ::mode_t = 64;
pub const S_ENFMT: ::mode_t = 1024;
pub const S_IFMT: ::mode_t = 61440;
pub const S_IFDIR: ::mode_t = 16384;
pub const S_IFCHR: ::mode_t = 8192;
pub const S_IFBLK: ::mode_t = 24576;
pub const S_IFREG: ::mode_t = 32768;
pub const S_IFLNK: ::mode_t = 40960;
pub const S_IFSOCK: ::mode_t = 49152;
pub const S_IFIFO: ::mode_t = 4096;
pub const S_IRUSR: ::mode_t = 256;
pub const S_IWUSR: ::mode_t = 128;
pub const S_IXUSR: ::mode_t = 64;
pub const S_IRGRP: ::mode_t = 32;
pub const S_IWGRP: ::mode_t = 16;
pub const S_IXGRP: ::mode_t = 8;
pub const S_IROTH: ::mode_t = 4;
pub const S_IWOTH: ::mode_t = 2;
pub const S_IXOTH: ::mode_t = 1;

//标准输入、输出
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;
