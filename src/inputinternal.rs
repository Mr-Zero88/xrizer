use crate::openxr_data::{self, OpenXrData};
// use log::{debug, info, trace, warn};
use openvr as vr;
// use openxr as xr;

use std::sync::Arc;

// IVRInputInternal_002
#[derive(macros::InterfaceImpl)]
#[interface = "IVRInputInternal"]
#[versions(002)]
pub struct InputInternal<C: openxr_data::Compositor> {
    // openxr: Arc<OpenXrData<C>>,
    vtables: Vtables<C>,
}

impl<C: openxr_data::Compositor> InputInternal<C> {
    pub fn new(_openxr: Arc<OpenXrData<C>>) -> Self {
        Self {
            // openxr,
            vtables: Default::default(),
        }
    }
}

impl<C: openxr_data::Compositor> vr::IVRInputInternal002_Interface for InputInternal<C> {
    fn UknFunc001(&self) {
        todo!()
    }
    fn UknFunc002(&self) {
        todo!()
    }
    fn UknFunc003(
        &self,
        p_internal_object: *mut ::std::os::raw::c_void,
        reserved: *mut usize,
    ) -> bool {
        crate::warn_unimplemented!("UknFunc003");
        log::warn!("UknFunc003(pInternalObject: {p_internal_object:p}, reserved: {reserved:p})");
        // todo!()
        return true;
    }
    fn UknFunc004(&self, handle: usize, p_out_internal_handle: *mut usize) -> u32 {
        crate::warn_unimplemented!("UknFunc004");
        // log::warn!("UknFunc004(handle: {handle:#x})");

        if !p_out_internal_handle.is_null() {
            unsafe {
                // Initialisiere den Wert. Wenn du ein passendes Objekt
                // aus UknFunc003 hast, müsste hier ggf. dessen Handle rein.
                *p_out_internal_handle = 0;
            }
        }

        0 // EVRInputError_None
    }
    fn UknFunc005(&self) {
        todo!()
    }
    fn UknFunc006(&self) {
        todo!()
    }
    fn UknFunc007(&self) {
        todo!()
    }
    fn UknFunc008(&self) {
        todo!()
    }
    fn UknFunc009(&self) {
        todo!()
    }
    fn UknFunc010(&self) {
        todo!()
    }
    fn UknFunc011(&self) {
        todo!()
    }
    fn UknFunc012(&self) {
        todo!()
    }
    fn UknFunc013(&self) {
        todo!()
    }
    fn UknFunc014(&self) {
        todo!()
    }
    fn UknFunc015(&self) {
        todo!()
    }
    fn UknFunc016(&self) {
        todo!()
    }
    fn UknFunc017(&self) {
        todo!()
    }
    fn UknFunc018(&self) {
        todo!()
    }
    fn UknFunc019(&self) {
        todo!()
    }
    fn UknFunc020(&self) {
        todo!()
    }
}
