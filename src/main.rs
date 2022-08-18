use ffi::*;
use std::ffi::{c_void, CStr, CString};
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
    fn free(ptr: *mut c_void);
    fn to_raw(self) -> (*mut c_void, c_int);
    fn from_raw(ptr: *const c_void, ptr_len: c_int) -> Self;
}

macro_rules! to_raw(
    ($t:ty) => (
        impl<'a> LSMType<'a> for $t {
            fn free(ptr: *mut c_void) {
                unsafe {
                    Box::from_raw(ptr);
                };
            }
            fn to_raw(self) -> (*mut c_void, c_int) {
                let raw = Box::into_raw(Box::new(self)) as *mut c_void;
                return (raw, std::mem::size_of::<$t>() as _); //as _ for i64/u64
            }
            fn from_raw(ptr: *const c_void, _:c_int) -> Self {
                let handle = ptr as *mut $t;
                return unsafe { *handle };
            }
        }
    )
);
to_raw!(i8);
to_raw!(i16);
to_raw!(i32);
to_raw!(i64); //test
to_raw!(u8);
to_raw!(u16);
to_raw!(u32);
to_raw!(u64); //test

impl<'a> LSMType<'a> for String {
    fn free(ptr: *mut c_void) {
        unsafe {
            drop(CString::from_raw(ptr as _));
        };
    }
    fn to_raw(self) -> (*mut c_void, c_int) {
        let length = self.len();
        let ck = CString::new(self).unwrap();
        let ptr_ck = ck.into_raw() as *mut c_void;
        return (ptr_ck, length as _);
    }
    fn from_raw(ptr: *const c_void, ptr_len: c_int) -> Self {
        let raw =
            unsafe { String::from_raw_parts(ptr as *mut u8, ptr_len as usize, ptr_len as usize) };
        return raw;
    }
}

impl<'a> LSMType<'a> for &str {
    fn free(ptr: *mut c_void) {
        unsafe {
            drop(CString::from_raw(ptr as _));
        };
    }
    fn to_raw(self) -> (*mut c_void, c_int) {
        let length = self.len();
        let ck = CString::new(self).unwrap();
        let ptr_ck = ck.into_raw() as *mut c_void;
        return (ptr_ck, length as _);
    }
    fn from_raw(ptr: *const c_void, ptr_len: c_int) -> Self {
        let s = unsafe { std::slice::from_raw_parts(ptr as _, ptr_len as _) };
        let lol = std::str::from_utf8(s).unwrap();
        return lol;
    }
}

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
        K::free(k.0);
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
        K::from_raw(k.0, k.1);
        V::from_raw(v.0, v.1);
        return r;
    }
    pub fn open(name: &str) -> Result<LSM<'a, K, V>, Error> {
        let lsm_env: *mut lsm_env = std::ptr::null_mut();
        let mut lsm_db: *mut lsm_db = std::ptr::null_mut();
        unsafe {
            //change this unsafe
            let ok = lsm_new(lsm_env, &mut lsm_db);
            if ok != OK {
                return Err(Error::from(ErrorKind::Other));
            }
            let cname = CString::new(name).unwrap();
            let c_world: *const c_char = cname.as_ptr() as *const c_char;
            let lsm = match lsm_open(lsm_db, c_world) {
                OK => LSM::<'a, K, V>(
                    std::marker::PhantomData,
                    std::marker::PhantomData,
                    std::marker::PhantomData,
                    lsm_db,
                ),
                _ => return Err(Error::from(ErrorKind::Other)),
            };
            let lsm_options: std::collections::HashSet<i32> = std::collections::HashSet::from([
                LSM_CONFIG_AUTOCHECKPOINT as _,
                LSM_CONFIG_AUTOFLUSH as _,
                LSM_CONFIG_AUTOMERGE as _,
                LSM_CONFIG_AUTOWORK as _,
                LSM_CONFIG_MMAP as _,
                LSM_CONFIG_MULTIPLE_PROCESSES as _,
                LSM_CONFIG_SAFETY as _,
                LSM_CONFIG_USE_LOG as _,
            ]);
            let options: Vec<Vec<&str>> = name
                .split("&")
                .map(|x| x.split("=").collect())
                .filter(|x: &Vec<&str>| x.len() == 2)
                .collect(); //improve collect in here¿?
            for option in options {
                let (k, v) = match (option[0].parse::<i32>(), option[1].parse::<i32>()) {
                    (Ok(k), Ok(v)) => match lsm_options.contains(&k) {
                        true => (k, v),
                        false => return Err(Error::from(ErrorKind::Other)),
                    },
                    _ => return Err(Error::from(ErrorKind::Other)),
                };
                match lsm_config(lsm_db, k, v) {
                    OK => continue,
                    _ => return Err(Error::from(ErrorKind::Other)),
                }
            }
            return Ok(lsm);
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
        let lsm = LSM::<&str, &str>::open("name").expect("cannot open db");
        lsm.insert("1", "1");
        lsm.insert("2", "2");

        for (k, v) in lsm.into_iter().rev() {
            println!("{}", k);
            println!("{}", v);
        }
    }

    #[test]
    fn iterate_lil() {
        let lsm = LSM::<String, String>::open("name").expect("cannot open db");
        lsm.insert("1".to_string(), "1".to_string());
        lsm.insert("2".to_string(), "2".to_string());

        for (k, v) in lsm.into_iter().rev() {
            println!("{}", k);
            println!("{}", v);
        }
    }

    #[test]
    fn bench_add_two() {
        let mut lol: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for i in 1..100_000 {
            let leel = i.to_string();
            let lool = i.to_string();
            lol.insert(leel, lool);
        }
        let lsm = LSM::<&str, &str>::open("name").expect("cannot open db");
        let now = std::time::Instant::now();
        for (k, v) in lol {
            lsm.insert(k.as_str(), v.as_str());
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
