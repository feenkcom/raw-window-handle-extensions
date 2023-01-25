use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{WebDisplayHandle, WebWindowHandle};

impl From<WebWindowHandle> for VeryRawWindowHandle {
    fn from(value: WebWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Web,
            ptr_1: std::ptr::null_mut(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: value.id as u64,
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for WebWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Web);
        let mut window_handle = Self::empty();
        window_handle.id = value.id_1 as u32;
        window_handle
    }
}

impl From<WebDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: WebDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Web,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for WebDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Web);
        let window_handle = Self::empty();
        window_handle
    }
}
