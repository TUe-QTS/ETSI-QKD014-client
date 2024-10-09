use crate::error::ErrorType::{InvalidArgument, InvalidHost, InvalidResponse};
use crate::{ETSI014Client, Error};
use libc::{c_char, size_t};
use secrets::SecretVec;
use std::ffi::{c_int, CStr, CString};
use std::future::Future;
use std::path::PathBuf;

pub const SAE_ID_LENGTH: usize = 37;

#[repr(C)]
pub struct CStatus {
    pub source_kme_id: [c_char; 255],
    pub target_kme_id: [c_char; 255],
    pub source_sae_id: [c_char; 255],
    pub target_sae_id: [c_char; 255],
    pub key_size: u32,
    pub stored_key_count: u32,
    pub max_key_count: u32,
    pub max_key_per_request: u32,
    pub max_key_size: u32,
    pub min_key_size: u32,
    pub max_sae_id_count: u32,
}

pub unsafe fn create_error_cstr(s: Error) -> *const c_char {
    CString::new(s.to_string())
        .unwrap_or_else(|_| CString::new("Null byte in error string").expect(""))
        .into_raw()
}

pub unsafe fn create_cstr<const SIZE: usize>(s: String) -> Result<[c_char; SIZE], Error> {
    let c_string = CString::new(s.clone()).map_err(|e| {
        Error::new(
            format!("Null byte in C string: {s}"),
            InvalidResponse,
            Some(Box::new(e)),
        )
    })?;
    let char_array = c_string.as_bytes_with_nul();
    if char_array.len() > SIZE {
        let char_array_size = char_array.len();
        return Err(Error::new(
            format!("String longer than {SIZE} characters ({char_array_size}): {s}"),
            InvalidResponse,
            None,
        ));
    };
    let mut r: [c_char; SIZE] = [0; SIZE];
    for (i, &byte) in char_array.iter().enumerate() {
        r[i] = byte as c_char;
    }
    Ok(r)
}

#[no_mangle]
pub unsafe extern "C" fn e14_new_etsi014_client(
    host: *const c_char,
    port: u16,
    cert_path: *const c_char,
    key_path: *const c_char,
    server_ca_path: *const c_char,
    etsi014_client: *mut *const ETSI014Client,
    error_str: *mut *const c_char,
) -> c_int {
    let host = match CStr::from_ptr(host).to_str() {
        Ok(h) => h,
        Err(utf8error) => {
            let error = Error::new(
                "Host is not valid UTF8".to_string(),
                InvalidHost,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let cert_path = match CStr::from_ptr(cert_path).to_str() {
        Ok(s) => PathBuf::from(s),
        Err(utf8error) => {
            let error = Error::new(
                "cert_path is not valid UTF8".to_string(),
                InvalidHost,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let key_path = match CStr::from_ptr(key_path).to_str() {
        Ok(s) => PathBuf::from(s),
        Err(utf8error) => {
            let error = Error::new(
                "key_path is not valid UTF8".to_string(),
                InvalidHost,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let server_ca_path = match CStr::from_ptr(server_ca_path).to_str() {
        Ok(s) => PathBuf::from(s),
        Err(utf8error) => {
            let error = Error::new(
                "server_ca_path is not valid UTF8".to_string(),
                InvalidHost,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    match ETSI014Client::new(host, port, &cert_path, &key_path, &server_ca_path) {
        Ok(client) => {
            *etsi014_client = Box::into_raw(Box::new(client));
            0
        }
        Err(e) => {
            *error_str = create_error_cstr(e);
            1
        }
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}

#[no_mangle]
pub unsafe extern "C" fn e14_get_status(
    client: *const ETSI014Client,
    target_sae_id: *const c_char,
    status: *mut CStatus,
    error_str: *mut *const c_char,
) -> c_int {
    let client = match client.as_ref() {
        Some(client) => client,
        None => {
            let error = Error::new(
                "Null pointer passed to get_status".to_string(),
                InvalidArgument,
                None,
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let target_sae_id = match CStr::from_ptr(target_sae_id).to_str() {
        Ok(id) => id,
        Err(utf8error) => {
            let error = Error::new(
                "target_sae_id is not valid UTF8".to_string(),
                InvalidArgument,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let status_result = block_on(client.get_status(target_sae_id));
    match status_result {
        Ok(s) => {
            let source_kme_id = match create_cstr(s.source_kme_id) {
                Ok(s) => s,
                Err(e) => {
                    *error_str = create_error_cstr(e);
                    return 1;
                }
            };
            let target_kme_id = match create_cstr(s.target_kme_id) {
                Ok(s) => s,
                Err(e) => {
                    *error_str = create_error_cstr(e);
                    return 1;
                }
            };
            let source_sae_id = match create_cstr(s.source_sae_id) {
                Ok(s) => s,
                Err(e) => {
                    *error_str = create_error_cstr(e);
                    return 1;
                }
            };
            let target_sae_id = match create_cstr(s.target_sae_id) {
                Ok(s) => s,
                Err(e) => {
                    *error_str = create_error_cstr(e);
                    return 1;
                }
            };
            *status = CStatus {
                source_kme_id,
                target_kme_id,
                source_sae_id,
                target_sae_id,
                key_size: s.key_size,
                stored_key_count: s.stored_key_count,
                max_key_count: s.max_key_count,
                max_key_per_request: s.max_key_per_request,
                max_key_size: s.max_key_size,
                min_key_size: s.min_key_size,
                max_sae_id_count: s.max_sae_id_count,
            };
            0
        }
        Err(e) => {
            *error_str = create_error_cstr(e);
            1
        }
    }
}

#[repr(packed)]
pub struct KeyBytesProtected {}

#[repr(packed)]
pub struct KeyBytesBorrow {}

#[repr(C)]
pub struct CKey {
    pub uuid: [c_char; SAE_ID_LENGTH],
    pub key_size: u32,
    pub key_bytes_protected: *const KeyBytesProtected,
}

unsafe fn key_vec_to_ckey(uuid: [c_char; SAE_ID_LENGTH], key_vec: SecretVec<u8>) -> CKey {
    let key_size = key_vec.len() as u32;
    let key_vec_ptr: *mut SecretVec<u8> = Box::into_raw(Box::new(key_vec));
    CKey {
        uuid,
        key_size,
        key_bytes_protected: key_vec_ptr as *const KeyBytesProtected,
    }
}

/// If functions returns a 0, the caller must call [`e14_free_etsi014_client`]. Otherwise,
/// the caller must call [`e14_free_error_str`]. Before using a qkd key, the caller must call
/// [`e14_unprotect_qkd_key_bytes`]. After a qkd key is not necessary anymore, the caller must
/// call [`e14_free_qkd_key_bytes`].
#[no_mangle]
pub unsafe extern "C" fn e14_get_keys(
    client: *const ETSI014Client,
    key_size_bits: u32,
    target_sae_id: *const c_char,
    _additional_target_sae_ids: *const c_char,
    additional_target_sae_ids_size: size_t,
    amount_of_keys: u32,
    keys: *mut CKey,
    error_str: *mut *const c_char,
) -> c_int {
    if additional_target_sae_ids_size != 0 {
        todo!("additional_target_sae_ids not yet implemented in c bindings");
    }
    let client = match client.as_ref() {
        Some(client) => client,
        None => {
            let error = Error::new(
                "Null pointer passed to get_status".to_string(),
                InvalidArgument,
                None,
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let target_sae_id = match CStr::from_ptr(target_sae_id).to_str() {
        Ok(id) => id,
        Err(utf8error) => {
            let error = Error::new(
                "target_sae_id is not valid UTF8".to_string(),
                InvalidArgument,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let keys = std::slice::from_raw_parts_mut(keys, amount_of_keys as usize);
    let get_keys_result =
        block_on(client.get_keys(key_size_bits, target_sae_id, &[], amount_of_keys));
    match get_keys_result {
        Ok(keys_recv) => {
            let keys_recv_len = keys_recv.len();
            if keys_recv_len != amount_of_keys as usize {
                *error_str = create_error_cstr(Error::new(
                    format!("Got {keys_recv_len} instead of {amount_of_keys} keys"),
                    InvalidResponse,
                    None,
                ));
                return 1;
            }
            for (i, (uuid_string, key_vec)) in keys_recv.into_iter().enumerate() {
                let uuid = match create_cstr(uuid_string) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        *error_str = create_error_cstr(e);
                        return 1;
                    }
                };
                keys[i] = key_vec_to_ckey(uuid, key_vec);
            }
            0
        }
        Err(e) => {
            *error_str = create_error_cstr(e);
            1
        }
    }
}

/// Documentation of function [`e14_get_keys`] also applies to this function.
#[no_mangle]
pub unsafe extern "C" fn e14_get_keys_by_ids(
    client: *const ETSI014Client,
    target_sae_id: *const c_char,
    key_ids: *mut *mut c_char,
    key_ids_len: size_t,
    keys: *mut CKey,
    error_str: *mut *const c_char,
) -> c_int {
    let client = match client.as_ref() {
        Some(client) => client,
        None => {
            let error = Error::new(
                "Null pointer passed to get_status".to_string(),
                InvalidArgument,
                None,
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let target_sae_id = match CStr::from_ptr(target_sae_id).to_str() {
        Ok(id) => id,
        Err(utf8error) => {
            let error = Error::new(
                "target_sae_id is not valid UTF8".to_string(),
                InvalidArgument,
                Some(Box::new(utf8error)),
            );
            *error_str = create_error_cstr(error);
            return 1;
        }
    };
    let mut key_ids_vec = Vec::with_capacity(key_ids_len);
    let key_ids = std::slice::from_raw_parts(key_ids, key_ids_len);
    for (i, &ptr) in key_ids.iter().enumerate() {
        let sae_id = match CStr::from_ptr(ptr).to_str() {
            Ok(id) => id,
            Err(utf8error) => {
                let error = Error::new(
                    format!("SAE ID {i} is not valid UTF8"),
                    InvalidArgument,
                    Some(Box::new(utf8error)),
                );
                *error_str = create_error_cstr(error);
                return 1;
            }
        };
        key_ids_vec.push(sae_id);
    }
    let keys = std::slice::from_raw_parts_mut(keys, key_ids_len);
    let get_keys_result =
        block_on(client.get_keys_by_ids(target_sae_id, key_ids_vec.as_slice()));
    match get_keys_result {
        Ok(keys_recv) => {
            let keys_recv_len = keys_recv.len();
            if keys_recv_len != key_ids_len {
                *error_str = create_error_cstr(Error::new(
                    format!("Got {keys_recv_len} instead of {key_ids_len} keys"),
                    InvalidResponse,
                    None,
                ));
                return 1;
            }
            for (i, (uuid_string, key_vec)) in keys_recv.into_iter().enumerate() {
                let uuid = match create_cstr(uuid_string) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        *error_str = create_error_cstr(e);
                        return 1;
                    }
                };
                keys[i] = key_vec_to_ckey(uuid, key_vec)
            }
            0
        }
        Err(e) => {
            *error_str = create_error_cstr(e);
            1
        }
    }
}

/// Unprotect memory to allow read and write access to qkd key.
/// To protect the memory again, call [`e14_protect_qkd_key_bytes`] instead.
/// e14_protect_qkd_key_bytes must be called:
/// * immediately after you are done reading/writing to key_bytes.
/// * before calling this function again.
/// * before calling [`e14_free_qkd_key_bytes`].
#[no_mangle]
#[allow(dyn_drop)]
pub unsafe extern "C" fn e14_unprotect_qkd_key_bytes(
    key_bytes_protected: *const KeyBytesProtected,
    key_bytes_borrow: *mut *const KeyBytesBorrow,
    key_bytes: *mut *const u8,
) {
    let qkd_key_vec = key_bytes_protected as *mut SecretVec<u8>;
    if qkd_key_vec.is_null() {
        return;
    }
    let mut ref_mut = (*qkd_key_vec).borrow_mut();
    *key_bytes = ref_mut.as_mut_ptr();
    let drop_box: Box<dyn Drop> = Box::new(ref_mut);
    let borrow = Box::into_raw(Box::new(drop_box));
    *key_bytes_borrow = borrow as *const KeyBytesBorrow;
}

/// Prevent read and write access to key_bytes.
#[no_mangle]
#[allow(dyn_drop)]
pub unsafe extern "C" fn e14_protect_qkd_key_bytes(
    borrow: *mut *const KeyBytesBorrow,
    key_bytes: *mut *const u8,
) {
    let borrow_drop = borrow as *mut *mut Box<dyn Drop>;
    if borrow_drop.is_null() || (*borrow_drop).is_null() {
        return;
    }
    let _ = Box::from_raw(*borrow_drop);
    *borrow = std::ptr::null();
    *key_bytes = std::ptr::null();
}

/// Will overwrite qkd key and deallocate memory.
#[no_mangle]
pub unsafe extern "C" fn e14_free_qkd_key_bytes(
    key_bytes_protected: *mut *const KeyBytesProtected,
) {
    let qkd_key_vec = key_bytes_protected as *mut *const SecretVec<u8>;
    if qkd_key_vec.is_null() || (*qkd_key_vec).is_null() {
        return;
    }
    let _ = Box::from_raw(*qkd_key_vec as *mut SecretVec<u8>);
    *qkd_key_vec = std::ptr::null();
}

#[no_mangle]
pub unsafe extern "C" fn e14_free_error_str(error_str: *mut *const c_char) {
    if error_str.is_null() || (*error_str).is_null() {
        return;
    }
    let _ = CString::from_raw(*error_str as *mut c_char);
    *error_str = std::ptr::null();
}

#[no_mangle]
pub unsafe extern "C" fn e14_free_etsi014_client(client: *mut *const ETSI014Client) {
    if client.is_null() || (*client).is_null() {
        return;
    }
    let _ = Box::from_raw(*client as *mut ETSI014Client);
    *client = std::ptr::null();
}
