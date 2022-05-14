#![allow(dead_code)]
extern crate pcre2_sys;

use pcre2_sys as pcre2;

pub mod replace_options;
pub mod compile_options;

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub code: i32,
    pub offset: usize
}

#[derive(Debug)]
pub struct Regex {
    pattern: *mut pcre2::pcre2_code_8,
    pub options: u32
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe { 
            pcre2::pcre2_code_free_8(self.pattern);
        };
    }
}

impl Regex {
    // TODO, match, test, find, replace with callback

    /// See also http://www.pcre.org/current/doc/html/pcre2_compile.html
    pub unsafe fn new<S: AsRef<str>>(pattern: S, options: u32) -> Result<Regex,CompileError> {
        let mut error_code = Box::new(0);
        let mut error_offset = Box::new(0);
        let regex = Regex {
            pattern: pcre2::pcre2_compile_8(
                pattern.as_ref().as_ptr(), pattern.as_ref().len(), 
                options,
                &mut *error_code, &mut *error_offset,
                std::ptr::null_mut()
            ),
            options
        };
        
        if *error_code != 100 {
            let mut out_buffer = Vec::<u8>::with_capacity(64);
            out_buffer.set_len(64);
            out_buffer.fill(0);
            pcre2::pcre2_get_error_message_8(*error_code, out_buffer.as_mut_ptr(), 64);
            let message = String::from_utf8_lossy(&out_buffer).to_string();

            return Err(CompileError { message, code:*error_code.clone(), offset:*error_offset.clone()});
        } else {
            return Ok(regex);
        }
    }

    /// See also http://www.pcre.org/current/doc/html/pcre2_substitute.html
    pub unsafe fn replace<S: AsRef<str>, K: AsRef<str>>(&self,text: S, replacement: K, offset: usize, options: u32) -> String {
        let buffer_size: *mut usize = Box::into_raw(Box::new(0));
        
        // Calculate buffer size
        pcre2::pcre2_substitute_8(
            self.pattern,
            text.as_ref().as_ptr(), text.as_ref().len(),
            offset, options | replace_options::SUBSTITUTE_OVERFLOW_LENGTH,
            std::ptr::null_mut(), std::ptr::null_mut(),
            replacement.as_ref().as_ptr(), replacement.as_ref().len(), 
            [0u8;0].as_mut_ptr(), buffer_size
        );
        
        let mut buffer = Vec::<u8>::with_capacity(*buffer_size);
        buffer.set_len(*buffer_size);
        buffer.fill(0);

        // Perform substitution
        pcre2::pcre2_substitute_8(
            self.pattern,
            text.as_ref().as_ptr(), text.as_ref().len(),
            offset, options,
            std::ptr::null_mut(), std::ptr::null_mut(),
            replacement.as_ref().as_ptr(), replacement.as_ref().len(), 
            buffer.as_mut_ptr(), buffer_size
        );

        return buffer.into_iter().map(|c| c as char).collect::<String>();
    }
}