use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{HaikuDisplayHandle, HaikuWindowHandle};

impl From<HaikuWindowHandle> for VeryRawWindowHandle {
    fn from(value: HaikuWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Haiku,
            ptr_1: value.b_window,
            ptr_2: value.b_direct_window,
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for HaikuWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Haiku);
        let mut window_handle = Self::empty();
        window_handle.b_window = value.ptr_1;
        window_handle.b_direct_window = value.ptr_2;
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
        let window_handle = Self::empty();
        window_handle
    }
}
