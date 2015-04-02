//! High level bindings to [libxdo](http://www.semicomplete.com/files/xdotool/docs/html/)

#![warn(missing_docs)]

extern crate libxdo_sys as sys;
extern crate libc;

use std::ffi::{CString, NulError};
use std::convert::From;
use libc::c_int;
use std::ptr::null;

/// An XDo instance
pub struct XDo {
    handle: *mut sys::xdo
}

/// An error that can happen when trying to create an XDo instance.
#[derive(Debug)]
pub enum XDoCreationError {
    /// The parameter passed in contained a nul character.
    NulError(NulError),
    /// Unknown error
    Unknown
}

impl From<NulError> for XDoCreationError {
    fn from(err: NulError) -> XDoCreationError {
        XDoCreationError::NulError(err)
    }
}

#[derive(Debug)]
enum XDoOperationErrorKind {
    NulError(NulError),
    OperationFailed
}

/// An error originating from an XDo operation.
#[derive(Debug)]
struct XDoOperationError {
    kind: XDoOperationErrorKind
}

impl From<NulError> for XDoOperationError {
    fn from(err: NulError) -> XDoOperationError {
        XDoOperationError {
            kind: XDoOperationErrorKind::NulError(err)
        }
    }
}

/// Result of an XDo operation.
pub type OpResult = Result<(), XDoOperationError>;

macro_rules! xdo (
    ($fncall: expr) => {
        unsafe {
            match $fncall {
                0 => Ok(()),
                _ => Err(XDoOperationError{ kind: XDoOperationErrorKind::OperationFailed })
            }
        }
    }
);

impl XDo {
    /// Creates a new XDo instance.
    ///
    /// # Parameters
    ///
    /// display - An optional string display name, such as ":0". If None, uses the environment
    /// DISPLAY.
    ///
    /// # Returns
    ///
    /// Returns a new XDo instance, or an XDoCreationError on error.
    pub fn new(display: Option<&str>) -> Result<XDo, XDoCreationError> {
        let display = match display {
            Some(display) => {
                let cstr = try!(CString::new(display));
                cstr.as_ptr()
            },
            None => null()
        };
        let handle = unsafe { sys::xdo_new(display) };
        if handle.is_null() {
            return Err(XDoCreationError::Unknown);
        }
        Ok(XDo {
            handle: handle
        })
    }
    /// Moves the mouse to the specified position.
    pub fn move_mouse(&self, x: i32, y: i32, screen: i32) -> OpResult {
        xdo!(sys::xdo_mousemove(self.handle, x as c_int, y as c_int, screen as c_int))
    }
    /// Moves the mouse relative to the current position.
    pub fn move_mouse_relative(&self, x: i32, y: i32) -> OpResult {
        xdo!(sys::xdo_mousemove_relative(self.handle, x as c_int, y as c_int))
    }
    /// Does a mouse click.
    pub fn click(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_click(self.handle, sys::CURRENTWINDOW, button as c_int))
    }
    /// Holds a mouse button down.
    pub fn mouse_down(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_mousedown(self.handle, sys::CURRENTWINDOW, button as c_int))
    }
    /// Releases a mouse button.
    pub fn mouse_up(&self, button: i32) -> OpResult {
        xdo!(sys::xdo_mouseup(self.handle, sys::CURRENTWINDOW, button as c_int))
    }
    /// Types the specified text.
    pub fn type_text(&self, text: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(text));
        xdo!(sys::xdo_type(self.handle, sys::CURRENTWINDOW, string.as_ptr(), delay_microsecs))
    }
    /// Does the specified key sequence.
    pub fn key_sequence(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_keysequence(self.handle, sys::CURRENTWINDOW, string.as_ptr(),
                                  delay_microsecs))
    }
    /// Releases the specified key sequence.
    pub fn key_sequence_up(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_keysequence_up(self.handle, sys::CURRENTWINDOW, string.as_ptr(),
                                     delay_microsecs))
    }
    /// Presses the specified key sequence down.
    pub fn key_sequence_down(&self, sequence: &str, delay_microsecs: u32) -> OpResult {
        let string = try!(CString::new(sequence));
        xdo!(sys::xdo_keysequence_down(self.handle, sys::CURRENTWINDOW, string.as_ptr(),
                                       delay_microsecs))
    }
}

impl Drop for XDo {
    fn drop(&mut self) {
        unsafe { sys::xdo_free(self.handle); }
    }
}
