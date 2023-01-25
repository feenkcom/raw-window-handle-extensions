use std::ffi::c_void;

use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::error::{UnsupportedDisplayHandleError, UnsupportedWindowHandleError};

mod android;
mod appkit;
mod error;
mod haiku;
mod redox;
mod uikit;
mod unix;
mod web;
mod windows;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RawWindowHandleType {
    Unknown,
    UiKit,
    AppKit,
    Orbital,
    Xlib,
    Xcb,
    Wayland,
    Drm,
    Gbm,
    Win32,
    WinRt,
    Web,
    AndroidNdk,
    Haiku,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RawDisplayHandleType {
    Unknown,
    UiKit,
    AppKit,
    Orbital,
    Xlib,
    Xcb,
    Wayland,
    Drm,
    Gbm,
    Windows,
    Web,
    Android,
    Haiku,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct VeryRawWindowHandle {
    pub handle_type: RawWindowHandleType,
    pub ptr_1: *mut c_void,
    pub ptr_2: *mut c_void,
    pub ptr_3: *mut c_void,
    pub id_1: u64,
    pub id_2: u64,
}

impl VeryRawWindowHandle {
    pub fn unknown() -> Self {
        Self {
            handle_type: RawWindowHandleType::Unknown,
            ptr_1: std::ptr::null_mut(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VeryRawDisplayHandle {
    pub handle_type: RawDisplayHandleType,
    pub ptr_1: *mut c_void,
    pub id_1: u64,
}

impl VeryRawDisplayHandle {
    pub fn unknown() -> Self {
        Self {
            handle_type: RawDisplayHandleType::Unknown,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<RawWindowHandle> for VeryRawWindowHandle {
    fn from(value: RawWindowHandle) -> Self {
        match value {
            RawWindowHandle::UiKit(handle) => handle.into(),
            RawWindowHandle::AppKit(handle) => handle.into(),
            RawWindowHandle::Orbital(handle) => handle.into(),
            RawWindowHandle::Xlib(handle) => handle.into(),
            RawWindowHandle::Xcb(handle) => handle.into(),
            RawWindowHandle::Wayland(handle) => handle.into(),
            RawWindowHandle::Drm(handle) => handle.into(),
            RawWindowHandle::Gbm(handle) => handle.into(),
            RawWindowHandle::Win32(handle) => handle.into(),
            RawWindowHandle::WinRt(handle) => handle.into(),
            RawWindowHandle::Web(handle) => handle.into(),
            RawWindowHandle::AndroidNdk(handle) => handle.into(),
            RawWindowHandle::Haiku(handle) => handle.into(),
            _ => Self::unknown(),
        }
    }
}

impl TryFrom<VeryRawWindowHandle> for RawWindowHandle {
    type Error = UnsupportedWindowHandleError;

    fn try_from(value: VeryRawWindowHandle) -> Result<Self, Self::Error> {
        match value.handle_type {
            RawWindowHandleType::Unknown => {
                Err(UnsupportedWindowHandleError::new(value.handle_type))
            }
            RawWindowHandleType::UiKit => Ok(RawWindowHandle::UiKit(value.into())),
            RawWindowHandleType::AppKit => Ok(RawWindowHandle::AppKit(value.into())),
            RawWindowHandleType::Orbital => Ok(RawWindowHandle::Orbital(value.into())),
            RawWindowHandleType::Xlib => Ok(RawWindowHandle::Xlib(value.into())),
            RawWindowHandleType::Xcb => Ok(RawWindowHandle::Xcb(value.into())),
            RawWindowHandleType::Wayland => Ok(RawWindowHandle::Wayland(value.into())),
            RawWindowHandleType::Drm => Ok(RawWindowHandle::Drm(value.into())),
            RawWindowHandleType::Gbm => Ok(RawWindowHandle::Gbm(value.into())),
            RawWindowHandleType::Win32 => Ok(RawWindowHandle::Win32(value.into())),
            RawWindowHandleType::WinRt => Ok(RawWindowHandle::WinRt(value.into())),
            RawWindowHandleType::Web => Ok(RawWindowHandle::Web(value.into())),
            RawWindowHandleType::AndroidNdk => Ok(RawWindowHandle::AndroidNdk(value.into())),
            RawWindowHandleType::Haiku => Ok(RawWindowHandle::Haiku(value.into())),
        }
    }
}

impl From<RawDisplayHandle> for VeryRawDisplayHandle {
    fn from(value: RawDisplayHandle) -> Self {
        match value {
            RawDisplayHandle::UiKit(handle) => handle.into(),
            RawDisplayHandle::AppKit(handle) => handle.into(),
            RawDisplayHandle::Orbital(handle) => handle.into(),
            RawDisplayHandle::Xlib(handle) => handle.into(),
            RawDisplayHandle::Xcb(handle) => handle.into(),
            RawDisplayHandle::Wayland(handle) => handle.into(),
            RawDisplayHandle::Drm(handle) => handle.into(),
            RawDisplayHandle::Gbm(handle) => handle.into(),
            RawDisplayHandle::Windows(handle) => handle.into(),
            RawDisplayHandle::Web(handle) => handle.into(),
            RawDisplayHandle::Android(handle) => handle.into(),
            RawDisplayHandle::Haiku(handle) => handle.into(),
            _ => Self::unknown(),
        }
    }
}

impl TryFrom<VeryRawDisplayHandle> for RawDisplayHandle {
    type Error = UnsupportedDisplayHandleError;

    fn try_from(value: VeryRawDisplayHandle) -> Result<Self, Self::Error> {
        match value.handle_type {
            RawDisplayHandleType::Unknown => {
                Err(UnsupportedDisplayHandleError::new(value.handle_type))
            }
            RawDisplayHandleType::UiKit => Ok(RawDisplayHandle::UiKit(value.into())),
            RawDisplayHandleType::AppKit => Ok(RawDisplayHandle::AppKit(value.into())),
            RawDisplayHandleType::Orbital => Ok(RawDisplayHandle::Orbital(value.into())),
            RawDisplayHandleType::Xlib => Ok(RawDisplayHandle::Xlib(value.into())),
            RawDisplayHandleType::Xcb => Ok(RawDisplayHandle::Xcb(value.into())),
            RawDisplayHandleType::Wayland => Ok(RawDisplayHandle::Wayland(value.into())),
            RawDisplayHandleType::Drm => Ok(RawDisplayHandle::Drm(value.into())),
            RawDisplayHandleType::Gbm => Ok(RawDisplayHandle::Gbm(value.into())),
            RawDisplayHandleType::Windows => Ok(RawDisplayHandle::Windows(value.into())),
            RawDisplayHandleType::Web => Ok(RawDisplayHandle::Web(value.into())),
            RawDisplayHandleType::Android => Ok(RawDisplayHandle::Android(value.into())),
            RawDisplayHandleType::Haiku => Ok(RawDisplayHandle::Haiku(value.into())),
        }
    }
}
