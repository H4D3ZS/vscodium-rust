use memmap2::Mmap;
use std::fs::File;
use std::os::raw::c_char;
use std::path::Path;

/// Opaque C-Compatible struct defining the active memory map bounds
pub struct AimMemory {
    _file: File,
    mmap: Mmap,
}

#[no_mangle]
pub extern "C" fn aim_mount_vfs(path_ptr: *const c_char) -> *mut AimMemory {
    if path_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    let c_str = unsafe { std::ffi::CStr::from_ptr(path_ptr) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    
    // Exact mapping for the physical .aim binary payload on disk natively
    let target = format!("{}\\.aim\\memory.aim", path_str);
    
    let file = match File::open(Path::new(&target)) {
        Ok(f) => f,
        Err(_) => return std::ptr::null_mut(),
    };
    
    // Mmap zero-copy directly off the OS File Descriptor into native RAM implicitly
    let mmap = match unsafe { Mmap::map(&file) } {
        Ok(m) => m,
        Err(_) => return std::ptr::null_mut(),
    };
    
    let aim_mem = Box::new(AimMemory {
        _file: file,
        mmap,
    });
    
    Box::into_raw(aim_mem)
}

#[no_mangle]
pub extern "C" fn aim_get_tensor(
    aim_ptr: *mut AimMemory,
    out_size: *mut usize,
) -> *const f32 {
    if aim_ptr.is_null() || out_size.is_null() {
        return std::ptr::null();
    }
    
    let aim = unsafe { &*aim_ptr };
    let bytes = &aim.mmap[..];
    
    // The .aim format encapsulates the 1536 float32 tensor directly after the JSON Header structural block.
    // For this raw C-FFI mapping, we locate the closure of the JSON object '}' to extract the float map natively.
    let mut header_end = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'}' {
            header_end = i + 1;
            break;
        }
    }
    
    if header_end == 0 || header_end + (1536 * 4) > bytes.len() {
        return std::ptr::null();
    }
    
    unsafe {
        *out_size = 1536; // Constant structural token dimension
    }
    
    // Return explicit unsafe pointer to physical OS memory bounding natively (Zero Copy / Zero Tokens)
    let tensor_start = &bytes[header_end];
    tensor_start as *const u8 as *const f32
}

#[no_mangle]
pub extern "C" fn aim_unmount_vfs(aim_ptr: *mut AimMemory) {
    if aim_ptr.is_null() {
        return;
    }
    // Reclaim ownership seamlessly to allow Rust's intrinsic Drop trait to un-map the active page natively
    unsafe {
        let _ = Box::from_raw(aim_ptr);
    }
}
