use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};

impl From<AppKitWindowHandle> for VeryRawWindowHandle {
    fn from(value: AppKitWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::AppKit,
            ptr_1: value.ns_window,
            ptr_2: value.ns_view,
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for AppKitWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::AppKit);
        let mut window_handle = Self::empty();
        window_handle.ns_window = value.ptr_1;
        window_handle.ns_view = value.ptr_2;
        window_handle
    }
}

impl From<AppKitDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: AppKitDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::AppKit,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for AppKitDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::AppKit);
        let window_handle = Self::empty();
        window_handle
    }
}
