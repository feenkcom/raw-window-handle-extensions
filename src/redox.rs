use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{OrbitalDisplayHandle, OrbitalWindowHandle};

impl From<OrbitalWindowHandle> for VeryRawWindowHandle {
    fn from(value: OrbitalWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Orbital,
            ptr_1: value.window,
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for OrbitalWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Orbital);
        let mut window_handle = Self::empty();
        window_handle.window = value.ptr_1;
        window_handle
    }
}

impl From<OrbitalDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: OrbitalDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Orbital,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}
impl From<VeryRawDisplayHandle> for OrbitalDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Orbital);
        let window_handle = Self::empty();
        window_handle
    }
}
