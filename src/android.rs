use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{AndroidDisplayHandle, AndroidNdkWindowHandle};

impl From<AndroidNdkWindowHandle> for VeryRawWindowHandle {
    fn from(value: AndroidNdkWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::AndroidNdk,
            ptr_1: value.a_native_window,
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
        let mut window_handle = Self::empty();
        window_handle.a_native_window = value.ptr_1;
        window_handle
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
        let window_handle = Self::empty();
        window_handle
    }
}
