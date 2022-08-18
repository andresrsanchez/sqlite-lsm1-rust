#[allow(warnings)]
mod ffi {
    pub const LSM_LOCK_UNLOCK: u32 = 0;
    pub const LSM_LOCK_SHARED: u32 = 1;
    pub const LSM_LOCK_EXCL: u32 = 2;
    pub const LSM_OPEN_READONLY: u32 = 1;
    pub const LSM_MUTEX_GLOBAL: u32 = 1;
    pub const LSM_MUTEX_HEAP: u32 = 2;
    pub const LSM_OK: u32 = 0;
    pub const LSM_ERROR: u32 = 1;
    pub const LSM_BUSY: u32 = 5;
    pub const LSM_NOMEM: u32 = 7;
    pub const LSM_READONLY: u32 = 8;
    pub const LSM_IOERR: u32 = 10;
    pub const LSM_CORRUPT: u32 = 11;
    pub const LSM_FULL: u32 = 13;
    pub const LSM_CANTOPEN: u32 = 14;
    pub const LSM_PROTOCOL: u32 = 15;
    pub const LSM_MISUSE: u32 = 21;
    pub const LSM_MISMATCH: u32 = 50;
    pub const LSM_IOERR_NOENT: u32 = 266;
    pub const LSM_CONFIG_AUTOFLUSH: u32 = 1;
    pub const LSM_CONFIG_PAGE_SIZE: u32 = 2;
    pub const LSM_CONFIG_SAFETY: u32 = 3;
    pub const LSM_CONFIG_BLOCK_SIZE: u32 = 4;
    pub const LSM_CONFIG_AUTOWORK: u32 = 5;
    pub const LSM_CONFIG_MMAP: u32 = 7;
    pub const LSM_CONFIG_USE_LOG: u32 = 8;
    pub const LSM_CONFIG_AUTOMERGE: u32 = 9;
    pub const LSM_CONFIG_MAX_FREELIST: u32 = 10;
    pub const LSM_CONFIG_MULTIPLE_PROCESSES: u32 = 11;
    pub const LSM_CONFIG_AUTOCHECKPOINT: u32 = 12;
    pub const LSM_CONFIG_SET_COMPRESSION: u32 = 13;
    pub const LSM_CONFIG_GET_COMPRESSION: u32 = 14;
    pub const LSM_CONFIG_SET_COMPRESSION_FACTORY: u32 = 15;
    pub const LSM_CONFIG_READONLY: u32 = 16;
    pub const LSM_SAFETY_OFF: u32 = 0;
    pub const LSM_SAFETY_NORMAL: u32 = 1;
    pub const LSM_SAFETY_FULL: u32 = 2;
    pub const LSM_COMPRESSION_EMPTY: u32 = 0;
    pub const LSM_COMPRESSION_NONE: u32 = 1;
    pub const LSM_INFO_NWRITE: u32 = 1;
    pub const LSM_INFO_NREAD: u32 = 2;
    pub const LSM_INFO_DB_STRUCTURE: u32 = 3;
    pub const LSM_INFO_LOG_STRUCTURE: u32 = 4;
    pub const LSM_INFO_ARRAY_STRUCTURE: u32 = 5;
    pub const LSM_INFO_PAGE_ASCII_DUMP: u32 = 6;
    pub const LSM_INFO_PAGE_HEX_DUMP: u32 = 7;
    pub const LSM_INFO_FREELIST: u32 = 8;
    pub const LSM_INFO_ARRAY_PAGES: u32 = 9;
    pub const LSM_INFO_CHECKPOINT_SIZE: u32 = 10;
    pub const LSM_INFO_TREE_SIZE: u32 = 11;
    pub const LSM_INFO_FREELIST_SIZE: u32 = 12;
    pub const LSM_INFO_COMPRESSION_ID: u32 = 13;
    pub const LSM_SEEK_LEFAST: i32 = -2;
    pub const LSM_SEEK_LE: i32 = -1;
    pub const LSM_SEEK_EQ: u32 = 0;
    pub const LSM_SEEK_GE: u32 = 1;
    pub type size_t = ::std::os::raw::c_ulong;
    pub type wchar_t = ::std::os::raw::c_int;
    pub type max_align_t = u128;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_cursor {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_db {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_file {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_mutex {
        _unused: [u8; 0],
    }
    pub type lsm_i64 = ::std::os::raw::c_longlong;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_env {
        pub nByte: ::std::os::raw::c_int,
        pub iVersion: ::std::os::raw::c_int,
        #[doc = " file i/o"]
        pub pVfsCtx: *mut ::std::os::raw::c_void,
        pub xFullpath: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                arg2: *const ::std::os::raw::c_char,
                arg3: *mut ::std::os::raw::c_char,
                arg4: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xOpen: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                arg2: *const ::std::os::raw::c_char,
                flags: ::std::os::raw::c_int,
                arg3: *mut *mut lsm_file,
            ) -> ::std::os::raw::c_int,
        >,
        pub xRead: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: lsm_i64,
                arg3: *mut ::std::os::raw::c_void,
                arg4: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xWrite: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: lsm_i64,
                arg3: *mut ::std::os::raw::c_void,
                arg4: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xTruncate: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_file, arg2: lsm_i64) -> ::std::os::raw::c_int,
        >,
        pub xSync: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_file) -> ::std::os::raw::c_int,
        >,
        pub xSectorSize: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_file) -> ::std::os::raw::c_int,
        >,
        pub xRemap: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: lsm_i64,
                arg3: *mut *mut ::std::os::raw::c_void,
                arg4: *mut lsm_i64,
            ) -> ::std::os::raw::c_int,
        >,
        pub xFileid: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                pBuf: *mut ::std::os::raw::c_void,
                pnBuf: *mut ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xClose: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_file) -> ::std::os::raw::c_int,
        >,
        pub xUnlink: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                arg2: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        pub xLock: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xTestLock: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xShmMap: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: *mut *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        pub xShmBarrier: ::std::option::Option<unsafe extern "C" fn()>,
        pub xShmUnmap: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_file,
                arg2: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        #[doc = " memory allocation"]
        pub pMemCtx: *mut ::std::os::raw::c_void,
        pub xMalloc: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_env, arg2: size_t) -> *mut ::std::os::raw::c_void,
        >,
        pub xRealloc: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                arg2: *mut ::std::os::raw::c_void,
                arg3: size_t,
            ) -> *mut ::std::os::raw::c_void,
        >,
        pub xFree: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_env, arg2: *mut ::std::os::raw::c_void),
        >,
        pub xSize: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_env, arg2: *mut ::std::os::raw::c_void) -> size_t,
        >,
        #[doc = " mutexes"]
        pub pMutexCtx: *mut ::std::os::raw::c_void,
        pub xMutexStatic: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                arg2: ::std::os::raw::c_int,
                arg3: *mut *mut lsm_mutex,
            ) -> ::std::os::raw::c_int,
        >,
        pub xMutexNew: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                arg2: *mut *mut lsm_mutex,
            ) -> ::std::os::raw::c_int,
        >,
        pub xMutexDel: ::std::option::Option<unsafe extern "C" fn(arg1: *mut lsm_mutex)>,
        pub xMutexEnter: ::std::option::Option<unsafe extern "C" fn(arg1: *mut lsm_mutex)>,
        pub xMutexTry: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_mutex) -> ::std::os::raw::c_int,
        >,
        pub xMutexLeave: ::std::option::Option<unsafe extern "C" fn(arg1: *mut lsm_mutex)>,
        pub xMutexHeld: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_mutex) -> ::std::os::raw::c_int,
        >,
        pub xMutexNotHeld: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut lsm_mutex) -> ::std::os::raw::c_int,
        >,
        #[doc = " other"]
        pub xSleep: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut lsm_env,
                microseconds: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
    }
    #[test]
    fn bindgen_test_layout_lsm_env() {
        assert_eq!(
            ::std::mem::size_of::<lsm_env>(),
            264usize,
            concat!("Size of: ", stringify!(lsm_env))
        );
        assert_eq!(
            ::std::mem::align_of::<lsm_env>(),
            8usize,
            concat!("Alignment of ", stringify!(lsm_env))
        );
        fn test_field_nByte() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).nByte) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(nByte)
                )
            );
        }
        test_field_nByte();
        fn test_field_iVersion() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).iVersion) as usize - ptr as usize
                },
                4usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(iVersion)
                )
            );
        }
        test_field_iVersion();
        fn test_field_pVfsCtx() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).pVfsCtx) as usize - ptr as usize
                },
                8usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(pVfsCtx)
                )
            );
        }
        test_field_pVfsCtx();
        fn test_field_xFullpath() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xFullpath) as usize - ptr as usize
                },
                16usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xFullpath)
                )
            );
        }
        test_field_xFullpath();
        fn test_field_xOpen() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xOpen) as usize - ptr as usize
                },
                24usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xOpen)
                )
            );
        }
        test_field_xOpen();
        fn test_field_xRead() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xRead) as usize - ptr as usize
                },
                32usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xRead)
                )
            );
        }
        test_field_xRead();
        fn test_field_xWrite() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xWrite) as usize - ptr as usize
                },
                40usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xWrite)
                )
            );
        }
        test_field_xWrite();
        fn test_field_xTruncate() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xTruncate) as usize - ptr as usize
                },
                48usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xTruncate)
                )
            );
        }
        test_field_xTruncate();
        fn test_field_xSync() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xSync) as usize - ptr as usize
                },
                56usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xSync)
                )
            );
        }
        test_field_xSync();
        fn test_field_xSectorSize() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xSectorSize) as usize - ptr as usize
                },
                64usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xSectorSize)
                )
            );
        }
        test_field_xSectorSize();
        fn test_field_xRemap() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xRemap) as usize - ptr as usize
                },
                72usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xRemap)
                )
            );
        }
        test_field_xRemap();
        fn test_field_xFileid() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xFileid) as usize - ptr as usize
                },
                80usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xFileid)
                )
            );
        }
        test_field_xFileid();
        fn test_field_xClose() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xClose) as usize - ptr as usize
                },
                88usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xClose)
                )
            );
        }
        test_field_xClose();
        fn test_field_xUnlink() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xUnlink) as usize - ptr as usize
                },
                96usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xUnlink)
                )
            );
        }
        test_field_xUnlink();
        fn test_field_xLock() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xLock) as usize - ptr as usize
                },
                104usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xLock)
                )
            );
        }
        test_field_xLock();
        fn test_field_xTestLock() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xTestLock) as usize - ptr as usize
                },
                112usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xTestLock)
                )
            );
        }
        test_field_xTestLock();
        fn test_field_xShmMap() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xShmMap) as usize - ptr as usize
                },
                120usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xShmMap)
                )
            );
        }
        test_field_xShmMap();
        fn test_field_xShmBarrier() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xShmBarrier) as usize - ptr as usize
                },
                128usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xShmBarrier)
                )
            );
        }
        test_field_xShmBarrier();
        fn test_field_xShmUnmap() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xShmUnmap) as usize - ptr as usize
                },
                136usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xShmUnmap)
                )
            );
        }
        test_field_xShmUnmap();
        fn test_field_pMemCtx() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).pMemCtx) as usize - ptr as usize
                },
                144usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(pMemCtx)
                )
            );
        }
        test_field_pMemCtx();
        fn test_field_xMalloc() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMalloc) as usize - ptr as usize
                },
                152usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMalloc)
                )
            );
        }
        test_field_xMalloc();
        fn test_field_xRealloc() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xRealloc) as usize - ptr as usize
                },
                160usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xRealloc)
                )
            );
        }
        test_field_xRealloc();
        fn test_field_xFree() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xFree) as usize - ptr as usize
                },
                168usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xFree)
                )
            );
        }
        test_field_xFree();
        fn test_field_xSize() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xSize) as usize - ptr as usize
                },
                176usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xSize)
                )
            );
        }
        test_field_xSize();
        fn test_field_pMutexCtx() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).pMutexCtx) as usize - ptr as usize
                },
                184usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(pMutexCtx)
                )
            );
        }
        test_field_pMutexCtx();
        fn test_field_xMutexStatic() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexStatic) as usize - ptr as usize
                },
                192usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexStatic)
                )
            );
        }
        test_field_xMutexStatic();
        fn test_field_xMutexNew() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexNew) as usize - ptr as usize
                },
                200usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexNew)
                )
            );
        }
        test_field_xMutexNew();
        fn test_field_xMutexDel() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexDel) as usize - ptr as usize
                },
                208usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexDel)
                )
            );
        }
        test_field_xMutexDel();
        fn test_field_xMutexEnter() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexEnter) as usize - ptr as usize
                },
                216usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexEnter)
                )
            );
        }
        test_field_xMutexEnter();
        fn test_field_xMutexTry() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexTry) as usize - ptr as usize
                },
                224usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexTry)
                )
            );
        }
        test_field_xMutexTry();
        fn test_field_xMutexLeave() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexLeave) as usize - ptr as usize
                },
                232usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexLeave)
                )
            );
        }
        test_field_xMutexLeave();
        fn test_field_xMutexHeld() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexHeld) as usize - ptr as usize
                },
                240usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexHeld)
                )
            );
        }
        test_field_xMutexHeld();
        fn test_field_xMutexNotHeld() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xMutexNotHeld) as usize - ptr as usize
                },
                248usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xMutexNotHeld)
                )
            );
        }
        test_field_xMutexNotHeld();
        fn test_field_xSleep() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_env>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xSleep) as usize - ptr as usize
                },
                256usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_env),
                    "::",
                    stringify!(xSleep)
                )
            );
        }
        test_field_xSleep();
    }
    extern "C" {
        pub fn lsm_new(arg1: *mut lsm_env, ppDb: *mut *mut lsm_db) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_close(pDb: *mut lsm_db) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_open(
            pDb: *mut lsm_db,
            zFilename: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_get_env(pDb: *mut lsm_db) -> *mut lsm_env;
    }
    extern "C" {
        pub fn lsm_default_env() -> *mut lsm_env;
    }
    extern "C" {
        pub fn lsm_config(
            arg1: *mut lsm_db,
            arg2: ::std::os::raw::c_int,
            args3: ...
        ) -> ::std::os::raw::c_int;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_compress {
        pub pCtx: *mut ::std::os::raw::c_void,
        pub iId: ::std::os::raw::c_uint,
        pub xBound: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                nSrc: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xCompress: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
                arg3: *mut ::std::os::raw::c_int,
                arg4: *const ::std::os::raw::c_char,
                arg5: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xUncompress: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
                arg3: *mut ::std::os::raw::c_int,
                arg4: *const ::std::os::raw::c_char,
                arg5: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        pub xFree: ::std::option::Option<unsafe extern "C" fn(pCtx: *mut ::std::os::raw::c_void)>,
    }
    #[test]
    fn bindgen_test_layout_lsm_compress() {
        assert_eq!(
            ::std::mem::size_of::<lsm_compress>(),
            48usize,
            concat!("Size of: ", stringify!(lsm_compress))
        );
        assert_eq!(
            ::std::mem::align_of::<lsm_compress>(),
            8usize,
            concat!("Alignment of ", stringify!(lsm_compress))
        );
        fn test_field_pCtx() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).pCtx) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress),
                    "::",
                    stringify!(pCtx)
                )
            );
        }
        test_field_pCtx();
        fn test_field_iId() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).iId) as usize - ptr as usize
                },
                8usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress),
                    "::",
                    stringify!(iId)
                )
            );
        }
        test_field_iId();
        fn test_field_xBound() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xBound) as usize - ptr as usize
                },
                16usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress),
                    "::",
                    stringify!(xBound)
                )
            );
        }
        test_field_xBound();
        fn test_field_xCompress() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xCompress) as usize - ptr as usize
                },
                24usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress),
                    "::",
                    stringify!(xCompress)
                )
            );
        }
        test_field_xCompress();
        fn test_field_xUncompress() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xUncompress) as usize - ptr as usize
                },
                32usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress),
                    "::",
                    stringify!(xUncompress)
                )
            );
        }
        test_field_xUncompress();
        fn test_field_xFree() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xFree) as usize - ptr as usize
                },
                40usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress),
                    "::",
                    stringify!(xFree)
                )
            );
        }
        test_field_xFree();
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct lsm_compress_factory {
        pub pCtx: *mut ::std::os::raw::c_void,
        pub xFactory: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut lsm_db,
                arg3: ::std::os::raw::c_uint,
            ) -> ::std::os::raw::c_int,
        >,
        pub xFree: ::std::option::Option<unsafe extern "C" fn(pCtx: *mut ::std::os::raw::c_void)>,
    }
    #[test]
    fn bindgen_test_layout_lsm_compress_factory() {
        assert_eq!(
            ::std::mem::size_of::<lsm_compress_factory>(),
            24usize,
            concat!("Size of: ", stringify!(lsm_compress_factory))
        );
        assert_eq!(
            ::std::mem::align_of::<lsm_compress_factory>(),
            8usize,
            concat!("Alignment of ", stringify!(lsm_compress_factory))
        );
        fn test_field_pCtx() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress_factory>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).pCtx) as usize - ptr as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress_factory),
                    "::",
                    stringify!(pCtx)
                )
            );
        }
        test_field_pCtx();
        fn test_field_xFactory() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress_factory>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xFactory) as usize - ptr as usize
                },
                8usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress_factory),
                    "::",
                    stringify!(xFactory)
                )
            );
        }
        test_field_xFactory();
        fn test_field_xFree() {
            assert_eq!(
                unsafe {
                    let uninit = ::std::mem::MaybeUninit::<lsm_compress_factory>::uninit();
                    let ptr = uninit.as_ptr();
                    ::std::ptr::addr_of!((*ptr).xFree) as usize - ptr as usize
                },
                16usize,
                concat!(
                    "Offset of field: ",
                    stringify!(lsm_compress_factory),
                    "::",
                    stringify!(xFree)
                )
            );
        }
        test_field_xFree();
    }
    extern "C" {
        pub fn lsm_malloc(arg1: *mut lsm_env, arg2: size_t) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn lsm_realloc(
            arg1: *mut lsm_env,
            arg2: *mut ::std::os::raw::c_void,
            arg3: size_t,
        ) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn lsm_free(arg1: *mut lsm_env, arg2: *mut ::std::os::raw::c_void);
    }
    extern "C" {
        pub fn lsm_info(
            arg1: *mut lsm_db,
            arg2: ::std::os::raw::c_int,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_get_user_version(
            arg1: *mut lsm_db,
            arg2: *mut ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_set_user_version(
            arg1: *mut lsm_db,
            arg2: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_begin(pDb: *mut lsm_db, iLevel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_commit(pDb: *mut lsm_db, iLevel: ::std::os::raw::c_int)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_rollback(
            pDb: *mut lsm_db,
            iLevel: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_insert(
            arg1: *mut lsm_db,
            pKey: *const ::std::os::raw::c_void,
            nKey: ::std::os::raw::c_int,
            pVal: *const ::std::os::raw::c_void,
            nVal: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_delete(
            arg1: *mut lsm_db,
            pKey: *const ::std::os::raw::c_void,
            nKey: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_delete_range(
            arg1: *mut lsm_db,
            pKey1: *const ::std::os::raw::c_void,
            nKey1: ::std::os::raw::c_int,
            pKey2: *const ::std::os::raw::c_void,
            nKey2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_work(
            pDb: *mut lsm_db,
            nMerge: ::std::os::raw::c_int,
            nKB: ::std::os::raw::c_int,
            pnWrite: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_flush(pDb: *mut lsm_db) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_checkpoint(
            pDb: *mut lsm_db,
            pnKB: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_open(pDb: *mut lsm_db, ppCsr: *mut *mut lsm_cursor)
            -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_close(pCsr: *mut lsm_cursor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_seek(
            pCsr: *mut lsm_cursor,
            pKey: *const ::std::os::raw::c_void,
            nKey: ::std::os::raw::c_int,
            eSeek: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_first(pCsr: *mut lsm_cursor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_last(pCsr: *mut lsm_cursor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_next(pCsr: *mut lsm_cursor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_prev(pCsr: *mut lsm_cursor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_valid(pCsr: *mut lsm_cursor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_key(
            pCsr: *mut lsm_cursor,
            ppKey: *mut *const ::std::os::raw::c_void,
            pnKey: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_value(
            pCsr: *mut lsm_cursor,
            ppVal: *mut *const ::std::os::raw::c_void,
            pnVal: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_csr_cmp(
            pCsr: *mut lsm_cursor,
            pKey: *const ::std::os::raw::c_void,
            nKey: ::std::os::raw::c_int,
            piRes: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn lsm_config_log(
            arg1: *mut lsm_db,
            arg2: ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::std::os::raw::c_void,
                    arg2: ::std::os::raw::c_int,
                    arg3: *const ::std::os::raw::c_char,
                ),
            >,
            arg3: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn lsm_config_work_hook(
            arg1: *mut lsm_db,
            arg2: ::std::option::Option<
                unsafe extern "C" fn(arg1: *mut lsm_db, arg2: *mut ::std::os::raw::c_void),
            >,
            arg3: *mut ::std::os::raw::c_void,
        );
    }
}
