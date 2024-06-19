use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};
use raw_window_handle::{UiKitDisplayHandle, UiKitWindowHandle};
use std::ptr::NonNull;

impl From<UiKitWindowHandle> for VeryRawWindowHandle {
    fn from(value: UiKitWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::UiKit,
            ptr_1: value.ui_view.as_ptr(),
            ptr_2: value
                .ui_view_controller
                .map(|handle| handle.as_ptr())
                .unwrap_or(std::ptr::null_mut()),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for UiKitWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::UiKit);
        let mut window_handle = Self::new(
            NonNull::new(value.ptr_1.into()).expect("UIKit native ui view must not be null"),
        );
        window_handle.ui_view_controller = if value.ptr_2.is_null() {
            None
        } else {
            Some(
                NonNull::new(value.ptr_2.into())
                    .expect("UIKit ui_view_controller must not be null"),
            )
        };
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
        Self::new()
    }
}
