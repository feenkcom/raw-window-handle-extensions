use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{UiKitDisplayHandle, UiKitWindowHandle};

impl From<UiKitWindowHandle> for VeryRawWindowHandle {
    fn from(value: UiKitWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::UiKit,
            ptr_1: value.ui_window,
            ptr_2: value.ui_view,
            ptr_3: value.ui_view_controller,
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for UiKitWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::UiKit);
        let mut window_handle = Self::empty();
        window_handle.ui_window = value.ptr_1;
        window_handle.ui_view = value.ptr_2;
        window_handle.ui_view_controller = value.ptr_3;
        window_handle
    }
}

impl From<UiKitDisplayHandle> for VeryRawDisplayHandle {
    fn from(_value: UiKitDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::UiKit,
            ptr_1: std::ptr::null_mut(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for UiKitDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::UiKit);
        let window_handle = Self::empty();
        window_handle
    }
}
