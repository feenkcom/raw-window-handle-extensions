use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{HaikuDisplayHandle, HaikuWindowHandle};
use std::ptr::NonNull;

impl From<HaikuWindowHandle> for VeryRawWindowHandle {
    fn from(value: HaikuWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Haiku,
            ptr_1: value.b_window.as_ptr(),
            ptr_2: value
                .b_direct_window
                .map(|ptr| ptr.as_ptr())
                .unwrap_or_else(|| std::ptr::null_mut()),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for HaikuWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Haiku);
        let mut window_handle =
            Self::new(NonNull::new(value.ptr_1).expect("Haiku window must not be null"));
        if !value.ptr_2.is_null() {
            window_handle.b_direct_window = Some(NonNull::new(value.ptr_2).unwrap());
        }
        window_handle
    }
}

impl From<HaikuDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: HaikuDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Haiku,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for HaikuDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Haiku);
        Self::new()
    }
}
