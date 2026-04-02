#[cfg(target_os = "macos")]
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

#[cfg(target_os = "macos")]
#[repr(C)]
pub struct ANEKernelHandle {
    _unused: [u8; 0],
}

#[cfg(target_os = "macos")]
extern "C" {
    pub fn ane_bridge_init() -> c_int;
    pub fn ane_bridge_compile(
        mil_text: *const c_char,
        mil_len: usize,
        weight_data: *const u8,
        weight_len: usize,
        n_inputs: c_int,
        input_sizes: *const usize,
        n_outputs: c_int,
        output_sizes: *const usize,
    ) -> *mut ANEKernelHandle;
    pub fn ane_bridge_eval(kernel: *mut ANEKernelHandle) -> bool;
    pub fn ane_bridge_write_input(
        kernel: *mut ANEKernelHandle,
        idx: c_int,
        data: *const c_void,
        bytes: usize,
    );
    pub fn ane_bridge_read_output(
        kernel: *mut ANEKernelHandle,
        idx: c_int,
        data: *mut c_void,
        bytes: usize,
    );
    pub fn ane_bridge_free(kernel: *mut ANEKernelHandle);
}

// MIL Header for iOS 18 (latest ANE MIL format)
#[cfg(target_os = "macos")]
const MIL_HDR: &str = r#"program(1.3)
[buildInfo = dict<string, string>({"coremlc-component-MIL", "3510.2.1"},
{"coremlc-version", "3505.4.1"}, {"coremltools-component-milinternal", ""},
{"coremltools-version", "9.0"}})]
{
"#;

// Stories110M Configuration
pub const DIM: usize = 768;
pub const HEADS: usize = 12;
pub const HD: usize = 64;
pub const SEQ: usize = 256;
pub const NLAYERS: usize = 12;

#[cfg(target_os = "macos")]
pub struct AneEngine {
    handle: *mut ANEKernelHandle,
}

#[cfg(not(target_os = "macos"))]
pub struct AneEngine {}

#[cfg(target_os = "macos")]
impl AneEngine {
    pub fn new(
        mil: &str,
        weights: &[u8],
        input_sizes: &[usize],
        output_sizes: &[usize],
    ) -> Result<Self, String> {
        unsafe {
            if ane_bridge_init() != 0 {
                return Err("Failed to initialize ANE bridge".to_string());
            }

            let mil_c = CString::new(mil).map_err(|e| e.to_string())?;
            let handle = ane_bridge_compile(
                mil_c.as_ptr(),
                mil.len(),
                weights.as_ptr(),
                weights.len(),
                input_sizes.len() as c_int,
                input_sizes.as_ptr(),
                output_sizes.len() as c_int,
                output_sizes.as_ptr(),
            );

            if handle.is_null() {
                return Err("Failed to compile ANE kernel".to_string());
            }

            Ok(Self { handle })
        }
    }

    /// Generates MIL for a dynamic matmul kernel: y = x @ W
    /// input [1, ic, 1, seq + oc], output [1, oc, 1, seq]
    pub fn gen_dyn_matmul_mil(ic: usize, oc: usize, seq: usize) -> String {
        let sp = seq + oc;
        format!(
            r#"{MIL_HDR}    func main<ios18>(tensor<fp16, [1, {ic}, 1, {sp}]> x) {{
        tensor<int32, [4]> mm_ba = const()[name=string("mm_ba"), val=tensor<int32, [4]>([0,0,0,0])];
        tensor<int32, [4]> mm_sa = const()[name=string("mm_sa"), val=tensor<int32, [4]>([1,{ic},1,{seq}])];
        tensor<fp16, [1,{ic},1,{seq}]> mm_act = slice_by_size(x=x,begin=mm_ba,size=mm_sa)[name=string("mm_act")];
        tensor<int32, [4]> mm_bw = const()[name=string("mm_bw"), val=tensor<int32, [4]>([0,0,0,{seq}])];
        tensor<int32, [4]> mm_sw = const()[name=string("mm_sw"), val=tensor<int32, [4]>([1,{ic},1,{oc}])];
        tensor<fp16, [1,{ic},1,{oc}]> mm_wt = slice_by_size(x=x,begin=mm_bw,size=mm_sw)[name=string("mm_wt")];
        tensor<int32, [4]> mm_ra = const()[name=string("mm_ra"), val=tensor<int32, [4]>([1,1,{ic},{seq}])];
        tensor<fp16, [1,1,{ic},{seq}]> mm_a2 = reshape(shape=mm_ra,x=mm_act)[name=string("mm_a2")];
        tensor<int32, [4]> mm_pm = const()[name=string("mm_pm"), val=tensor<int32, [4]>([0,1,3,2])];
        tensor<fp16, [1,1,{seq},{ic}]> mm_a3 = transpose(perm=mm_pm,x=mm_a2)[name=string("mm_a3")];
        tensor<int32, [4]> mm_rw = const()[name=string("mm_rw"), val=tensor<int32, [4]>([1,1,{ic},{oc}])];
        tensor<fp16, [1,1,{ic},{oc}]> mm_W = reshape(shape=mm_rw,x=mm_wt)[name=string("mm_W")];
        bool bF = const()[name=string("bF"), val=bool(false)];
        tensor<fp16, [1,1,{seq},{oc}]> mm_yh = matmul(transpose_x=bF,transpose_y=bF,x=mm_a3,y=mm_W)[name=string("mm_yh")];
        tensor<fp16, [1,1,{oc},{seq}]> mm_yt = transpose(perm=mm_pm,x=mm_yh)[name=string("mm_yt")];
        tensor<int32, [4]> mm_ro = const()[name=string("mm_ro"), val=tensor<int32, [4]>([1,{oc},1,{seq}])];
        tensor<fp16, [1,{oc},1,{seq}]> mm_y = reshape(shape=mm_ro,x=mm_yt)[name=string("mm_y")];
    }} -> (mm_y);
}}
"#
        )
    }

    pub fn execute(
        &self,
        inputs: &[Vec<u8>],
        output_sizes: &[usize],
    ) -> Result<Vec<Vec<u8>>, String> {
        unsafe {
            for (idx, input) in inputs.iter().enumerate() {
                ane_bridge_write_input(
                    self.handle,
                    idx as c_int,
                    input.as_ptr() as *const c_void,
                    input.len(),
                );
            }

            if !ane_bridge_eval(self.handle) {
                return Err("ANE evaluation failed".to_string());
            }

            let mut outputs = Vec::new();
            for (idx, &size) in output_sizes.iter().enumerate() {
                let mut buffer = vec![0u8; size];
                ane_bridge_read_output(
                    self.handle,
                    idx as c_int,
                    buffer.as_mut_ptr() as *mut c_void,
                    size,
                );
                outputs.push(buffer);
            }

            Ok(outputs)
        }
    }
}

#[cfg(not(target_os = "macos"))]
impl AneEngine {
    pub fn new(
        _mil: &str,
        _weights: &[u8],
        _input_sizes: &[usize],
        _output_sizes: &[usize],
    ) -> Result<Self, String> {
        Err("ANE support only available on macOS".to_string())
    }

    pub fn execute(
        &self,
        _inputs: &[Vec<u8>],
        _output_sizes: &[usize],
    ) -> Result<Vec<Vec<u8>>, String> {
        Err("ANE support only available on macOS".to_string())
    }

    pub fn gen_dyn_matmul_mil(_ic: usize, _oc: usize, _seq: usize) -> String {
        String::new()
    }
}

/// Simple FP32 to FP16 converter (software-based for portability)
pub fn f32_to_f16(f: f32) -> u16 {
    let bits = f.to_bits();
    let sign = (bits >> 16) & 0x8000;
    let mut expo = ((bits >> 23) & 0xFF) as i32 - 127;
    let mut mant = bits & 0x7FFFFF;

    if expo <= -15 {
        return sign as u16;
    } else if expo >= 16 {
        return (sign | 0x7C00) as u16;
    } else {
        expo += 15;
        mant >>= 13;
        return (sign | ((expo << 10) as u32) | mant) as u16;
    }
}

pub fn f16_to_f32(h: u16) -> f32 {
    let sign = (h & 0x8000) as u32;
    let mut expo = (h & 0x7C00) >> 10;
    let mut mant = (h & 0x03FF) as u32;

    if expo == 0 {
        if mant == 0 {
            return f32::from_bits(sign << 16);
        }
        // Subnormal
        while (mant & 0x0400) == 0 {
            mant <<= 1;
            expo -= 1;
        }
        expo += 1;
        mant &= 0x03FF;
    } else if expo == 31 {
        return f32::from_bits((sign << 16) | 0x7F800000 | (mant << 13));
    }

    let f_expo = (expo as i32 + 127 - 15) as u32;
    f32::from_bits((sign << 16) | (f_expo << 23) | (mant << 13))
}

#[cfg(target_os = "macos")]
impl Drop for AneEngine {
    fn drop(&mut self) {
        unsafe {
            ane_bridge_free(self.handle);
        }
    }
}

unsafe impl Send for AneEngine {}
unsafe impl Sync for AneEngine {}
