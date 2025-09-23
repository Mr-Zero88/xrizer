use openvr as vr;

#[derive(macros::InterfaceImpl)]
#[interface = "IVRChaperoneSetup"]
#[versions(006, 005, 004)]
pub struct ChaperoneSetup {
    vtables: Vtables,
}

impl ChaperoneSetup {
    pub fn new() -> Self {
        Self {
            vtables: Default::default(),
        }
    }
}

impl vr::IVRChaperoneSetup006_Interface for ChaperoneSetup {
    fn CommitWorkingCopy(&self, _config_file: vr::EChaperoneConfigFile) -> bool {
        todo!()
    }
    fn RevertWorkingCopy(&self) {
        todo!()
    }
    fn GetWorkingPlayAreaSize(&self, _x: *mut f32, _z: *mut f32) -> bool {
        todo!()
    }
    fn GetWorkingPlayAreaRect(&self, _rect: *mut vr::HmdQuad_t) -> bool {
        todo!()
    }
    fn GetWorkingCollisionBoundsInfo(
        &self,
        _quads_buffer: *mut vr::HmdQuad_t,
        _quads_count: *mut u32,
    ) -> bool {
        todo!()
    }
    fn GetLiveCollisionBoundsInfo(
        &self,
        _quads_buffer: *mut vr::HmdQuad_t,
        _quads_count: *mut u32,
    ) -> bool {
        todo!()
    }
    fn GetWorkingSeatedZeroPoseToRawTrackingPose(
        &self,
        _seated_zero_pose_to_raw_tracking_pose: *mut vr::HmdMatrix34_t,
    ) -> bool {
        todo!()
    }
    fn GetWorkingStandingZeroPoseToRawTrackingPose(
        &self,
        _standing_zero_pose_to_raw_tracking_pose: *mut vr::HmdMatrix34_t,
    ) -> bool {
        todo!()
    }
    fn SetWorkingPlayAreaSize(&self, _x: f32, _z: f32) {
        todo!()
    }
    fn SetWorkingCollisionBoundsInfo(&self, _quads_buffer: *mut vr::HmdQuad_t, _quads_count: u32) {
        todo!()
    }
    fn SetWorkingPerimeter(&self, _point_buffer: *mut vr::HmdVector2_t, _point_count: u32) {
        todo!()
    }
    fn SetWorkingSeatedZeroPoseToRawTrackingPose(
        &self,
        _seated_zero_pose_to_raw_tracking_pose: *const vr::HmdMatrix34_t,
    ) {
        todo!()
    }
    fn SetWorkingStandingZeroPoseToRawTrackingPose(
        &self,
        _standing_zero_pose_to_raw_tracking_pose: *const vr::HmdMatrix34_t,
    ) {
        todo!()
    }
    fn ReloadFromDisk(&self, _config_file: vr::EChaperoneConfigFile) {
        todo!()
    }
    fn GetLiveSeatedZeroPoseToRawTrackingPose(
        &self,
        _seated_zero_pose_to_raw_tracking_pose: *mut vr::HmdMatrix34_t,
    ) -> bool {
        todo!()
    }
    fn ExportLiveToBuffer(
        &self,
        _buffer: *mut ::std::os::raw::c_char,
        _buffer_length: *mut u32,
    ) -> bool {
        todo!()
    }
    fn ImportFromBufferToWorking(
        &self,
        _buffer: *const ::std::os::raw::c_char,
        _import_flags: u32,
    ) -> bool {
        todo!()
    }
    fn ShowWorkingSetPreview(&self) {
        todo!()
    }
    fn HideWorkingSetPreview(&self) {
        todo!()
    }
    fn RoomSetupStarting(&self) {
        todo!()
    }
}

impl vr::IVRChaperoneSetup005On006 for ChaperoneSetup {
    fn SetWorkingCollisionBoundsTagsInfo(&self, _tags_buffer: *mut u8, _tag_count: u32) {
        todo!()
    }
    fn GetLiveCollisionBoundsTagsInfo(&self, _tags_buffer: *mut u8, _tag_count: *mut u32) -> bool {
        todo!()
    }
    fn SetWorkingPhysicalBoundsInfo(
        &self,
        _quads_buffer: *mut vr::HmdQuad_t,
        _quads_count: u32,
    ) -> bool {
        todo!()
    }
    fn GetLivePhysicalBoundsInfo(
        &self,
        _quads_buffer: *mut vr::HmdQuad_t,
        _quads_count: *mut u32,
    ) -> bool {
        todo!()
    }
}
