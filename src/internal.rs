// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! internal class to handle OpenAl context and device.
//!
//! Work as a Singleton, check_al_context must be called before each OpenAl object
//! to be sure that the context is created.

#![macro_use]

use std::ffi::CString;
use std::cell::RefCell;
use std::ptr;
use openal::ffi;
use record_context;
use record_context::RecordContext;

thread_local!(static AL_CONTEXT: RefCell<Box<OpenAlData>> = RefCell::new(Box::new(OpenAlData::default())));

#[derive(Clone)]
pub struct OpenAlData {
    pub al_context: ffi::ALCcontextPtr,
    pub al_device: ffi::ALCdevicePtr,
    pub al_capt_device: ffi::ALCdevicePtr
}

impl OpenAlData {
    /// Create a new OpenAlData struct
    ///
    /// Private method.
    fn new() -> Result<OpenAlData, String> {
        let device = unsafe { ffi::alcOpenDevice(ptr::null_mut()) };
        if device == 0 {
            return Err("internal error: cannot open the default device.".to_string());
        }
        let context = unsafe { ffi::alcCreateContext(device, ptr::null_mut()) };
        if context == 0 {
            return Err("internal error: cannot create the OpenAL context.".to_string());
        }
        if unsafe { ffi::alcMakeContextCurrent(context) } == ffi::ALC_FALSE {
            return Err("internal error: cannot make the OpenAL context current.".to_string());
        }

        Ok(
            OpenAlData {
                al_context: context,
                al_device: device,
                al_capt_device: 0
            }
        )
    }

    fn default() -> OpenAlData {
        OpenAlData {
            al_context: 0,
            al_device: 0,
            al_capt_device: 0
        }
    }

    fn is_default(&self) -> bool {
        if self.al_context == 0 &&
           self.al_device == 0 &&
           self.al_capt_device == 0 {
            true
        } else {
            false
        }
    }

    /// Check if the context is created.
    ///
    /// This function check is the OpenAl context is already created.
    /// If context doesn't exist, create it, and store it in a local_data,
    /// else get it from the local data and return it.
    ///
    /// # Return
    /// A result containing nothing if the OpenAlData struct exist,
    /// otherwise an error message.
    pub fn check_al_context() -> Result<(), String> {
        if unsafe { ffi::alcGetCurrentContext() != 0 } {
            return Ok(())
        }
        AL_CONTEXT.with(|f| {
            let is_def = f.borrow_mut().is_default();
            if is_def {
                match OpenAlData::new() {
                    Ok(al_data) => {
                        *f.borrow_mut() = Box::new(al_data); Ok(())
                    },
                    Err(err) => Err(err)
                }
            } else {
                Ok(())
            }
        })
    }

    fn is_input_context_init() -> Result<RecordContext, String> {
        // let is_some = AL_CONTEXT.get().is_some();
        AL_CONTEXT.with(|f| {
            let is_def = f.borrow_mut().is_default();
            if !is_def {
                let mut new_context = f.borrow_mut();
                if !new_context.al_capt_device == 0 {
                    Ok(record_context::new(new_context.al_capt_device))
                } else {
					let c_str = CString::new("ALC_EXT_CAPTURE").unwrap();
                    if unsafe {
                        ffi::alcIsExtensionPresent(new_context.al_device, c_str.as_ptr()) } == ffi::ALC_FALSE {
                        return Err("Error: no input device available on your system.".to_string())
                    } else {
                        new_context.al_capt_device = unsafe {
                        ffi::alcCaptureOpenDevice(ptr::null_mut(),
                                                  44100,
                                                  ffi::AL_FORMAT_MONO16,
                                                  44100) };
                        if new_context.al_capt_device == 0 {
                            return Err("internal error: cannot open the default capture device.".to_string())
                        } else {
                           let cap_device = new_context.al_capt_device;
                           return Ok(record_context::new(cap_device))
                        }
                    }
                    Err("Error: you must request the input context, \
                        in the task where you initialize ears.".to_string())
                }
            } else {
                Err("Error: you must request the input context, \
                    in the task where you initialize ears.".to_string())
            }
        })
    }

    /// Check if the input context is created.
    ///
    /// This function check if the input OpenAl context is already created.
    /// The input openAL context need the normal AL context + its own extension.
    /// So check if the context exist first, then load the input extension.
    ///
    /// # Return
    /// A result containing nothing if the OpenAlData struct exist,
    /// otherwise an error message.
    pub fn check_al_input_context() -> Result<RecordContext, String> {
        if unsafe { !ffi::alcGetCurrentContext() == 0 } {
            OpenAlData::is_input_context_init()
        } else {
            match OpenAlData::check_al_context() {
                Ok(_)    => OpenAlData::is_input_context_init(),
                Err(err) => Err(err)
            }
        }
    }
}

impl Drop for OpenAlData {
    fn drop(&mut self) {
        unsafe {
            ffi::alcDestroyContext(self.al_context);
            if !self.al_capt_device == 0 {
                ffi::alcCaptureCloseDevice(self.al_capt_device);
            }
            ffi::alcCloseDevice(self.al_device);
        }
    }
}

macro_rules! check_openal_context(
    ($def_ret:expr) => (
            match OpenAlData::check_al_context() {
                Ok(_)    => {},
                Err(err) => { println!("{}", err); return $def_ret; }
            }
        );
);
