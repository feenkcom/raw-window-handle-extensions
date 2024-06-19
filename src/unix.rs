use raw_window_handle::{
    DrmDisplayHandle, DrmWindowHandle, GbmDisplayHandle, GbmWindowHandle, WaylandDisplayHandle,
    WaylandWindowHandle, XcbDisplayHandle, XcbWindowHandle, XlibDisplayHandle, XlibWindowHandle,
};
use std::ffi::{c_int, c_ulong};
use std::num::NonZeroU32;
use std::ptr::NonNull;

use crate::{RawDisplayHandleType, RawWindowHandleType, VeryRawDisplayHandle, VeryRawWindowHandle};

impl From<XlibWindowHandle> for VeryRawWindowHandle {
    fn from(value: XlibWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Xlib,
            ptr_1: std::ptr::null_mut(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: value.window as u64,
            id_2: value.visual_id as u64,
        }
    }
}

impl From<VeryRawWindowHandle> for XlibWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Xlib);
        let mut window_handle = Self::new(value.id_1 as c_ulong);
        window_handle.visual_id = value.id_2 as c_ulong;
        window_handle
    }
}

impl From<XlibDisplayHandle> for VeryRawDisplayHandle {
    fn from(value: XlibDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Xlib,
            ptr_1: value
                .display
                .map(|ptr| ptr.as_ptr())
                .unwrap_or(std::ptr::null_mut()),
            id_1: value.screen as u64,
        }
    }
}

impl From<VeryRawDisplayHandle> for XlibDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Xlib);

        let window_handle = Self::new(NonNull::new(value.ptr_1), value.id_1 as c_int);
        window_handle
    }
}

impl From<XcbWindowHandle> for VeryRawWindowHandle {
    fn from(value: XcbWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Xcb,
            ptr_1: std::ptr::null_mut(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: value.window.get() as u64,
            id_2: value.visual_id.map(|id| id.get()).unwrap_or(0) as u64,
        }
    }
}

impl From<VeryRawWindowHandle> for XcbWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Xcb);
        let mut window_handle = Self::new(NonZeroU32::new(value.id_1 as u32).unwrap());
        window_handle.visual_id = NonZeroU32::new(value.id_2 as u32);
        window_handle
    }
}

impl From<XcbDisplayHandle> for VeryRawDisplayHandle {
    fn from(value: XcbDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Xcb,
            ptr_1: value
                .connection
                .map(|value| value.as_ptr())
                .unwrap_or(std::ptr::null_mut()),
            id_1: value.screen as u64,
        }
    }
}

impl From<VeryRawDisplayHandle> for XcbDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Xcb);
        let window_handle = Self::new(NonNull::new(value.ptr_1), value.id_1 as c_int);
        window_handle
    }
}

impl From<WaylandWindowHandle> for VeryRawWindowHandle {
    fn from(value: WaylandWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Wayland,
            ptr_1: value.surface.as_ptr(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for WaylandWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Wayland);
        let window_handle = Self::new(NonNull::new(value.ptr_1).unwrap());
        window_handle
    }
}

impl From<WaylandDisplayHandle> for VeryRawDisplayHandle {
    fn from(value: WaylandDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Wayland,
            ptr_1: value.display.as_ptr(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for WaylandDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Wayland);
        let window_handle = Self::new(NonNull::new(value.ptr_1).unwrap());
        window_handle
    }
}

impl From<DrmWindowHandle> for VeryRawWindowHandle {
    fn from(value: DrmWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Drm,
            ptr_1: std::ptr::null_mut(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: value.plane as u64,
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for DrmWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Drm);
        let window_handle = Self::new(value.id_1 as u32);
        window_handle
    }
}

impl From<DrmDisplayHandle> for VeryRawDisplayHandle {
    fn from(value: DrmDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Drm,
            ptr_1: std::ptr::null_mut(),
            id_1: value.fd as u64,
        }
    }
}

impl From<VeryRawDisplayHandle> for DrmDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Drm);
        let window_handle = Self::new(value.id_1 as i32);
        window_handle
    }
}

impl From<GbmWindowHandle> for VeryRawWindowHandle {
    fn from(value: GbmWindowHandle) -> Self {
        Self {
            handle_type: RawWindowHandleType::Gbm,
            ptr_1: value.gbm_surface.as_ptr(),
            ptr_2: std::ptr::null_mut(),
            ptr_3: std::ptr::null_mut(),
            id_1: Default::default(),
            id_2: Default::default(),
        }
    }
}

impl From<VeryRawWindowHandle> for GbmWindowHandle {
    fn from(value: VeryRawWindowHandle) -> Self {
        assert_eq!(value.handle_type, RawWindowHandleType::Gbm);
        let window_handle = Self::new(NonNull::new(value.ptr_1).unwrap());
        window_handle
    }
}

impl From<GbmDisplayHandle> for VeryRawDisplayHandle {
    fn from(value: GbmDisplayHandle) -> Self {
        Self {
            handle_type: RawDisplayHandleType::Gbm,
            ptr_1: value.gbm_device.as_ptr(),
            id_1: Default::default(),
        }
    }
}

impl From<VeryRawDisplayHandle> for GbmDisplayHandle {
    fn from(value: VeryRawDisplayHandle) -> Self {
        assert_eq!(value.handle_type, RawDisplayHandleType::Gbm);
        let window_handle = Self::new(NonNull::new(value.ptr_1).unwrap());
        window_handle
    }
}
