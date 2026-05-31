//! Zero-Copy Tensor Wrapper for ggml_tensor Integration
//!
//! This module provides safe wrapper types for exchanging tensor data
//! between llama.cpp's ggml_tensor and HADES Kernel's MappedTensor.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

/// Wrapper around a raw ggml_tensor data pointer
///
/// This wrapper provides safe access to tensor data without copying.
/// The underlying data is owned by llama.cpp - we just provide access.
///
/// # Safety
/// The wrapped pointer must remain valid for the lifetime of this wrapper.
/// llama.cpp must not free or reallocate the data while this wrapper exists.
pub struct GgmlTensorWrapper {
    /// Raw data pointer (owned by ggml_tensor)
    data_ptr: *mut c_void,
    /// Total size in bytes
    size_bytes: usize,
    /// Number of elements
    nelements: usize,
    /// Element size in bytes
    element_size: usize,
    /// Flag indicating if we own the data (false for wrapped ggml tensors)
    #[allow(dead_code)]
    owned: bool,
    /// Validity flag for safety
    valid: AtomicBool,
}

// Safety: GgmlTensorWrapper can be sent between threads
// The underlying data must be thread-safe (managed by llama.cpp)
unsafe impl Send for GgmlTensorWrapper {}
unsafe impl Sync for GgmlTensorWrapper {}

impl GgmlTensorWrapper {
    /// Wrap a raw ggml_tensor data pointer
    ///
    /// # Parameters
    /// - `data_ptr`: Raw pointer from ggml_tensor->data
    /// - `nelements`: Total element count
    /// - `element_size`: Bytes per element (e.g., 4 for f32, 2 for f16)
    ///
    /// # Safety
    /// Caller must ensure data_ptr remains valid for wrapper lifetime
    pub unsafe fn wrap_raw(
        data_ptr: *mut c_void,
        nelements: usize,
        element_size: usize,
    ) -> Self {
        let size_bytes = nelements * element_size;

        Self {
            data_ptr,
            size_bytes,
            nelements,
            element_size,
            owned: false,
            valid: AtomicBool::new(true),
        }
    }

    /// Get the raw data pointer
    pub fn data_ptr(&self) -> *const c_void {
        self.data_ptr
    }

    /// Get mutable data pointer (for in-place modifications)
    pub fn data_ptr_mut(&mut self) -> *mut c_void {
        self.data_ptr
    }

    /// Get total size in bytes
    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }

    /// Get number of elements
    pub fn nelements(&self) -> usize {
        self.nelements
    }

    /// Get element size in bytes
    pub fn element_size(&self) -> usize {
        self.element_size
    }

    /// Check if wrapper is valid
    pub fn is_valid(&self) -> bool {
        self.valid.load(Ordering::SeqCst)
    }

    /// Invalidate the wrapper (call before dropping if data becomes invalid)
    pub fn invalidate(&self) {
        self.valid.store(false, Ordering::SeqCst);
    }

    /// Get data as f32 slice
    ///
    /// # Safety
    /// Caller must ensure element_size == 4 (f32)
    pub unsafe fn as_f32_slice(&self) -> &[f32] {
        assert_eq!(self.element_size, 4, "Element size must be 4 for f32");
        std::slice::from_raw_parts(self.data_ptr as *const f32, self.nelements)
    }

    /// Get data as f16 slice (u16 representation)
    ///
    /// # Safety
    /// Caller must ensure element_size == 2 (f16)
    pub unsafe fn as_f16_slice(&self) -> &[u16] {
        assert_eq!(self.element_size, 2, "Element size must be 2 for f16");
        std::slice::from_raw_parts(self.data_ptr as *const u16, self.nelements)
    }

    /// Get data as u8 slice (for quantized tensors)
    pub unsafe fn as_u8_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.data_ptr as *const u8, self.nelements * self.element_size)
    }

    /// Copy data to destination buffer
    ///
    /// # Parameters
    /// - `dst`: Destination pointer
    /// - `size`: Destination size in bytes
    ///
    /// # Returns
    /// true on success, false if size mismatch
    pub fn copy_to(&self, dst: *mut c_void, size: usize) -> bool {
        if size < self.size_bytes {
            return false;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(self.data_ptr, dst, self.size_bytes);
        }
        true
    }

    /// Copy data from source buffer
    ///
    /// # Parameters
    /// - `src`: Source pointer
    /// - `size`: Source size in bytes
    ///
    /// # Returns
    /// true on success, false if size mismatch
    pub fn copy_from(&mut self, src: *const c_void, size: usize) -> bool {
        if size < self.size_bytes {
            return false;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(src, self.data_ptr, self.size_bytes);
        }
        true
    }
}

impl Drop for GgmlTensorWrapper {
    fn drop(&mut self) {
        self.invalidate();
        // Note: We don't free data_ptr because it's owned by llama.cpp
    }
}

/// Owned tensor buffer (for data that needs to outlive ggml_tensor)
///
/// This allocates its own memory and copies data from a ggml_tensor.
/// Use when you need to keep tensor data after llama.cpp frees it.
pub struct OwnedTensorBuffer {
    data: Vec<u8>,
    nelements: usize,
    element_size: usize,
}

impl OwnedTensorBuffer {
    /// Create from a ggml_tensor wrapper by copying data
    pub fn from_wrapper(wrapper: &GgmlTensorWrapper) -> Self {
        let mut data = vec![0u8; wrapper.size_bytes()];
        unsafe {
            std::ptr::copy_nonoverlapping(
                wrapper.data_ptr(),
                data.as_mut_ptr() as *mut c_void,
                wrapper.size_bytes(),
            );
        }

        Self {
            data,
            nelements: wrapper.nelements(),
            element_size: wrapper.element_size(),
        }
    }

    /// Create from raw parts
    pub fn from_raw(data: Vec<u8>, nelements: usize, element_size: usize) -> Self {
        Self {
            data,
            nelements,
            element_size,
        }
    }

    /// Get data pointer
    pub fn data_ptr(&self) -> *const c_void {
        self.data.as_ptr() as *const c_void
    }

    /// Get mutable data pointer
    pub fn data_ptr_mut(&mut self) -> *mut c_void {
        self.data.as_mut_ptr() as *mut c_void
    }

    /// Get size in bytes
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }

    /// Get as f32 slice
    pub fn as_f32_slice(&self) -> &[f32] {
        assert_eq!(self.element_size, 4);
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const f32, self.nelements) }
    }

    /// Get as mutable f32 slice
    pub fn as_f32_slice_mut(&mut self) -> &mut [f32] {
        assert_eq!(self.element_size, 4);
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut f32, self.nelements) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_wrapper_creation() {
        let mut data = vec![1.0f32, 2.0, 3.0, 4.0];
        let ptr = data.as_mut_ptr() as *mut c_void;

        unsafe {
            let wrapper = GgmlTensorWrapper::wrap_raw(ptr, 4, 4);
            assert_eq!(wrapper.nelements(), 4);
            assert_eq!(wrapper.element_size(), 4);
            assert_eq!(wrapper.size_bytes(), 16);
            assert!(wrapper.is_valid());

            let slice = wrapper.as_f32_slice();
            assert_eq!(slice, &[1.0, 2.0, 3.0, 4.0]);
        }
    }

    #[test]
    fn test_owned_buffer() {
        let mut data = vec![1.0f32, 2.0, 3.0, 4.0];
        let ptr = data.as_mut_ptr() as *mut c_void;

        let wrapper = unsafe { GgmlTensorWrapper::wrap_raw(ptr, 4, 4) };
        let owned = OwnedTensorBuffer::from_wrapper(&wrapper);

        assert_eq!(owned.nelements, 4);
        assert_eq!(owned.element_size, 4);
        assert_eq!(owned.as_f32_slice(), &[1.0, 2.0, 3.0, 4.0]);
    }
}
