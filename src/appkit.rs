use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
use std::ptr::NonNull;

impl From<AppKitWindowHandle> for VeryRawWindowHandle {
    fn from(value: AppKitWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::AppKit,
            ptr_1: std::ptr::null_mut(),
            ptr_2: value.ns_view.as_ptr(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for AppKitWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::AppKit);
        Self::new(NonNull::new(value.ptr_2).expect("AppKit ns_view must not be null"))
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
        Self::new()
    }
}
