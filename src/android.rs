use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{AndroidDisplayHandle, AndroidNdkWindowHandle};
use std::ptr::NonNull;

impl From<AndroidNdkWindowHandle> for VeryRawWindowHandle {
    fn from(value: AndroidNdkWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::AndroidNdk,
            ptr_1: value.a_native_window.as_ptr(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for AndroidNdkWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::AndroidNdk);
        Self::new(
            NonNull::new(value.ptr_1.into())
                .expect("Android native window pointer must not be null"),
        )
    }
}

impl From<AndroidDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: AndroidDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Android,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for AndroidDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Android);
        Self::new()
    }
}
