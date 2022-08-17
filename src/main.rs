use ffi::*;
use std::ffi::{c_void, CString};
use std::io::{Error, ErrorKind};
use std::os::raw::{c_char, c_int};
include! {"./ffi.rs"}
fn main() {}

const OK: i32 = LSM_OK as i32;

pub struct LSM<'a, K: LSMType<'a>, V: LSMType<'a>>(
    std::marker::PhantomData<K>,
    std::marker::PhantomData<V>,
    std::marker::PhantomData<&'a ()>,
    *mut lsm_db,
);

pub trait LSMType<'a>: Sized {
    fn to_raw(self) -> (*mut c_void, c_int);
    fn from_raw(ptr: *const c_void, ptr_len: c_int) -> Self;
}

macro_rules! to_raw(
    ($t:ty) => (
        impl<'a> LSMType<'a> for $t {
            fn to_raw(self) -> (*mut c_void, c_int) {
                let raw = Box::into_raw(Box::new(self)) as *mut c_void;
                return (raw, std::mem::size_of::<$t>() as _);
            }
            fn from_raw(ptr: *const c_void, _:c_int) -> Self {
                let handle = ptr as *mut $t;
                return unsafe { *handle };
            }
        }
    )
);

to_raw!(i32);

// impl<'a> LSMType<'a> for String {
//     fn to_raw(self) -> (*mut c_void, c_int) {
//         let l = self.len();
//         let ck = CString::new(self).unwrap();
//         let ptr_ck = ck.as_ptr() as *mut c_void;
//         return (ptr_ck, l as _);
//         // let l = self.len();
//         // let raw = Box::into_raw(Box::new(self)) as *mut c_void;
//         // return (raw, l as _);
//         // let cstr = CString::new(self).unwrap();
//         // return (cstr.into_raw() as _, l as _);
//     }
//     fn from_raw(ptr: *const c_void, ptr_len: c_int) -> Self {
//         let kraw =
//             unsafe { String::from_raw_parts(ptr as *mut u8, ptr_len as usize, ptr_len as usize) };
//         return kraw;
//         // let result = unsafe { std::ffi::CStr::from_ptr(ptr as _) };
//         // let lol = result.to_str();
//         // let result_str = result.to_str().unwrap();
//         // return result_str.to_owned();
//     }
// }

pub struct LSMIterator<'a, K: LSMType<'a>, V: LSMType<'a>> {
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,
    _a: std::marker::PhantomData<&'a ()>,
    f_csr: Option<*mut lsm_cursor>,
    b_csr: Option<*mut lsm_cursor>,
    is_first: bool,
    is_last: bool,
}

impl<'a, K: LSMType<'a>, V: LSMType<'a>> LSMIterator<'a, K, V> {
    fn new(f_raw: Option<*mut lsm_cursor>, b_raw: Option<*mut lsm_cursor>) -> Self {
        LSMIterator::<'a, K, V> {
            _k: std::marker::PhantomData,
            _v: std::marker::PhantomData,
            _a: std::marker::PhantomData,
            f_csr: f_raw,
            b_csr: b_raw,
            is_first: true,
            is_last: true,
        }
    }
    fn get_kv(&self, cursor: *mut lsm_cursor) -> Option<(K, V)> {
        let mut key_ptr: *const c_void = std::ptr::null();
        let mut key_len: c_int = 0;
        let mut val_ptr: *const c_void = std::ptr::null();
        let mut val_len: c_int = 0;
        if unsafe { lsm_csr_key(cursor, &mut key_ptr, &mut key_len as *mut c_int) } != OK {
            return None;
        }
        match unsafe { lsm_csr_value(cursor, &mut val_ptr, &mut val_len as *mut c_int) } {
            OK => {
                let k = K::from_raw(key_ptr, key_len);
                let v = V::from_raw(val_ptr, val_len);
                return Some((k, v));
            }
            _ => None,
        }
    }
}

impl<'a, K: LSMType<'a>, V: LSMType<'a>> LSM<'a, K, V> {
    pub fn single(self, key: K) -> Option<V> {
        let mut cursor: *mut lsm_cursor = std::ptr::null_mut();
        if unsafe { lsm_csr_open(self.3, &mut cursor) } != OK {
            return None;
        }
        let k = key.to_raw();
        unsafe { lsm_csr_seek(cursor, k.0, k.1, LSM_SEEK_EQ as _) };
        let mut val_ptr: *const c_void = std::ptr::null();
        let mut val_len: c_int = 0;
        let r = if unsafe { lsm_csr_value(cursor, &mut val_ptr, &mut val_len as *mut c_int) } == OK
        {
            let v = V::from_raw(val_ptr, val_len);
            Some(v)
        } else {
            None
        };
        unsafe {
            Box::from_raw(k.0);
        };
        return r;
    }
    pub fn delete(self, key: K) -> Result<(), Error> {
        let k = key.to_raw();
        let r = if unsafe { lsm_delete(self.3, k.0, k.1) } == OK {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other))
        };
        unsafe {
            Box::from_raw(k.0);
        };
        return r;
    }
    pub fn insert(&self, key: K, value: V) -> Result<(), Error> {
        let k = key.to_raw();
        let v = value.to_raw();
        let r = if unsafe { lsm_insert(self.3, k.0, k.1, v.0, v.1) } == OK {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other))
        };
        unsafe {
            Box::from_raw(k.0);
            Box::from_raw(v.0)
        };
        return r;
    }
    pub fn open(name: &str) -> Result<LSM<'a, K, V>, Error> {
        let lsm_env: *mut lsm_env = std::ptr::null_mut();
        let mut lsm_db: *mut lsm_db = std::ptr::null_mut();
        unsafe {
            let ok = lsm_new(lsm_env, &mut lsm_db);
            if ok != OK {
                return Err(Error::from(ErrorKind::Other));
            }
            let cname = CString::new(name).unwrap();
            let c_world: *const c_char = cname.as_ptr() as *const c_char;
            match lsm_open(lsm_db, c_world) {
                OK => Ok(LSM(
                    std::marker::PhantomData,
                    std::marker::PhantomData,
                    std::marker::PhantomData,
                    lsm_db,
                )),
                _ => Err(Error::from(ErrorKind::Other)),
            }
        }
    }
}

impl<'a, K: LSMType<'a>, V: LSMType<'a>> IntoIterator for &'a LSM<'a, K, V> {
    type Item = (K, V);
    type IntoIter = LSMIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut f_raw: *mut lsm_cursor = std::ptr::null_mut();
        let mut b_raw: *mut lsm_cursor = std::ptr::null_mut();
        match unsafe { lsm_csr_open(self.3, &mut f_raw) } {
            OK => match unsafe { lsm_csr_open(self.3, &mut b_raw) } {
                OK => Self::IntoIter::new(Some(f_raw), Some(b_raw)),
                _ => Self::IntoIter::new(None, None),
            },
            _ => Self::IntoIter::new(None, None),
        }
    }
}

impl<'a, K: LSMType<'a>, V: LSMType<'a>> Iterator for LSMIterator<'a, K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.f_csr?;
        if self.is_first {
            if unsafe { lsm_csr_first(cursor) } != OK {
                unsafe { lsm_csr_close(cursor) };
                return None;
            }
            self.is_first = false;
            self.f_csr = Some(cursor);
        }
        match unsafe { lsm_csr_valid(cursor) } {
            OK => None,
            _ => match self.get_kv(cursor) {
                Some(kv) => {
                    unsafe { lsm_csr_next(cursor) };
                    return Some(kv);
                }
                _ => {
                    unsafe { lsm_csr_close(cursor) };
                    None
                }
            },
        }
    }
}

impl<'a, K: LSMType<'a>, V: LSMType<'a>> DoubleEndedIterator for LSMIterator<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let cursor = self.b_csr?;
        if self.is_last {
            if unsafe { lsm_csr_last(cursor) } != OK {
                unsafe { lsm_csr_close(cursor) };
                return None;
            }
            self.is_last = false;
            self.b_csr = Some(cursor);
        }
        match unsafe { lsm_csr_valid(cursor) } {
            OK => None,
            _ => match self.get_kv(cursor) {
                Some(kv) => {
                    unsafe { lsm_csr_prev(cursor) };
                    return Some(kv);
                }
                _ => {
                    unsafe { lsm_csr_close(cursor) };
                    None
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_lol() {
        let lsm = LSM::<i32, i32>::open("name").expect("cannot open db");
        for i in 0..10 {
            lsm.insert(i, i + 1);
        }
        // lsm.insert("11", "1").expect("cannot insert");
        // lsm.insert("222", "2").expect("cannot insert");
        for (k, v) in lsm.into_iter().rev() {
            println!("{}", k);
            println!("{}", v);
        }
        // lsm.insert("33", "1").expect("cannot insert");
        // for val in lsm.into_iter() {
        //     println!("{}", val.k);
        //     println!("{}", val.v);
        // }
        // lsm.insert("222", "2").expect("cannot insert");
        // for lol in lsm {}
        //let mut values = LSMValues::new(&lsm).expect("cannot create iterator");
    }

    #[test]
    fn bench_add_two() {
        let lsm = LSM::<i32, i32>::open("name").expect("cannot open db");
        let now = std::time::Instant::now();
        let j = 0;
        for i in 1..100_000 {
            lsm.insert(j, i + 1);
        }
        println!("{}", now.elapsed().as_micros());
    }

    // #[test]
    // fn bench_add_three() {
    //     let lsm = LSM::<String, String>::open("name").expect("cannot open db");
    //     let now = std::time::Instant::now();
    //     for i in 0..10 {
    //         lsm.insert(i.to_string(), (i + 1).to_string());
    //     }

    //     for (k, v) in lsm.into_iter().rev() {
    //         println!("{}", k);
    //         println!("{}", v);
    //     }

    //     println!("{}", now.elapsed().as_micros());
    // }
}
