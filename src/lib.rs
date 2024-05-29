use std::ffi::CStr;
use std::os::raw::c_char;

mod any_wrap;
mod tiff_wrap;

/// Called in C#
/// [DllImport("rust_image.dll", EntryPoint = "tiff_encode_16", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Auto)]
/// public static extern unsafe bool TiffEncoder([In] byte* input, [In] byte* output, uint width, ushort min, ushort max);
///
/// static unsafe void TiffEncoder(string input, string output, uint width, ushort min, ushort max)
/// {
///     var inputCharSpan = input.AsSpan();
///     Span<byte> inputByteSpan = stackalloc byte[Encoding.UTF8.GetByteCount(inputCharSpan)];
///     Encoding.UTF8.GetBytes(inputCharSpan, inputByteSpan);
///
///     var outputCharSpan = output.AsSpan();
///     Span<byte> outputByteSpan = stackalloc byte[Encoding.UTF8.GetByteCount(outputCharSpan)];
///     Encoding.UTF8.GetBytes(outputCharSpan, outputByteSpan);
///
///     fixed (byte* inptr = inputByteSpan)
///     {
///         fixed (byte* outptr = outputByteSpan)
///         {
///             NativeUtil.TiffEncoder(inptr, outptr, width, min, max);
///         }
///     }
/// }
///
#[no_mangle]
pub extern "C" fn tiff_encode_16(
    input: *const c_char,
    output: *const c_char,
    width: u32,
    min: u16,
    max: u16,
) {
    let input_path = unsafe { CStr::from_ptr(input) }.to_str().unwrap();
    let output_path = unsafe { CStr::from_ptr(output) }.to_str().unwrap();

    tiff_wrap::encode_from_file_16(input_path, output_path, width, min, max);
}

#[no_mangle]
pub extern "C" fn tiff_encode_8(input: *const c_char, output: *const c_char, width: u32) {
    let input_path = unsafe { CStr::from_ptr(input) }.to_str().unwrap();
    let output_path = unsafe { CStr::from_ptr(output) }.to_str().unwrap();

    tiff_wrap::encode_from_file_8(input_path, output_path, width);
}

#[no_mangle]
pub extern "C" fn any_encode_16(
    input: *const c_char,
    output: *const c_char,
    width: u32,
    min: u16,
    max: u16,
) {
    let input_path = unsafe { CStr::from_ptr(input) }.to_str().unwrap();
    let output_path = unsafe { CStr::from_ptr(output) }.to_str().unwrap();

    any_wrap::encode_from_file_16(input_path, output_path, width, min, max);
}

#[no_mangle]
pub extern "C" fn any_encode_8(input: *const c_char, output: *const c_char, width: u32) {
    let input_path = unsafe { CStr::from_ptr(input) }.to_str().unwrap();
    let output_path = unsafe { CStr::from_ptr(output) }.to_str().unwrap();

    any_wrap::encode_from_file_8(input_path, output_path, width);
}
