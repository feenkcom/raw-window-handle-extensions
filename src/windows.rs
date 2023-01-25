use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{Win32WindowHandle, WinRtWindowHandle, WindowsDisplayHandle};

impl From<Win32WindowHandle> for VeryRawWindowHandle {
    fn from(value: Win32WindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Win32,
            ptr_1: value.hwnd,
            ptr_2: value.hinstance,
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for Win32WindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Win32);
        let mut window_handle = Self::empty();
        window_handle.hwnd = value.ptr_1;
        window_handle.hinstance = value.ptr_2;
        window_handle
    }
}

impl From<WinRtWindowHandle> for VeryRawWindowHandle {
    fn from(value: WinRtWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::WinRt,
            ptr_1: value.core_window,
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for WinRtWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::WinRt);
        let mut window_handle = Self::empty();
        window_handle.core_window = value.ptr_1;
        window_handle
    }
}

impl From<WindowsDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: WindowsDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Windows,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for WindowsDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Windows);
        let window_handle = Self::empty();
        window_handle
    }
}
