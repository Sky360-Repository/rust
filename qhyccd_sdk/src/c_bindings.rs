pub type QhyCcdHandle = ::std::os::raw::c_void;

extern "C" {
    pub fn InitQHYCCDResource() -> u32;
    pub fn ReleaseQHYCCDResource() -> u32;
    pub fn ScanQHYCCD() -> u32;
    pub fn GetQHYCCDId(index: u32, id: *mut ::std::os::raw::c_char) -> u32;
    pub fn GetQHYCCDModel(
        id: *mut ::std::os::raw::c_char,
        model: *mut ::std::os::raw::c_char,
    ) -> u32;
    pub fn OpenQHYCCD(id: *mut ::std::os::raw::c_char) -> *mut QhyCcdHandle;
    pub fn CloseQHYCCD(handle: *mut QhyCcdHandle) -> u32;
    pub fn SetQHYCCDStreamMode(handle: *mut QhyCcdHandle, mode: u8) -> u32;
    pub fn InitQHYCCD(handle: *mut QhyCcdHandle) -> u32;
    pub fn IsQHYCCDControlAvailable(handle: *mut QhyCcdHandle, controlId: u32) -> u32;
    pub fn SetQHYCCDParam(handle: *mut QhyCcdHandle, controlId: u32, value: f64) -> u32;
    pub fn GetQHYCCDParam(handle: *mut QhyCcdHandle, controlId: u32) -> f64;
    pub fn GetQHYCCDParamMinMaxStep(
        handle: *mut QhyCcdHandle,
        controlId: u32,
        min: *mut f64,
        max: *mut f64,
        step: *mut f64,
    ) -> u32;
    pub fn SetQHYCCDResolution(
        handle: *mut QhyCcdHandle,
        x: u32,
        y: u32,
        xsize: u32,
        ysize: u32,
    ) -> u32;
    pub fn GetQHYCCDMemLength(handle: *mut QhyCcdHandle) -> u32;
    pub fn ExpQHYCCDSingleFrame(handle: *mut QhyCcdHandle) -> u32;
    pub fn GetQHYCCDSingleFrame(
        handle: *mut QhyCcdHandle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
    pub fn CancelQHYCCDExposing(handle: *mut QhyCcdHandle) -> u32;
    pub fn CancelQHYCCDExposingAndReadout(handle: *mut QhyCcdHandle) -> u32;
    pub fn BeginQHYCCDLive(handle: *mut QhyCcdHandle) -> u32;
    pub fn GetQHYCCDLiveFrame(
        handle: *mut QhyCcdHandle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
    pub fn StopQHYCCDLive(handle: *mut QhyCcdHandle) -> u32;
    pub fn SetQHYCCDBinMode(handle: *mut QhyCcdHandle, wbin: u32, hbin: u32) -> u32;
    pub fn SetQHYCCDBitsMode(handle: *mut QhyCcdHandle, bits: u32) -> u32;
    pub fn ControlQHYCCDTemp(handle: *mut QhyCcdHandle, targettemp: f64) -> u32;
    pub fn GetQHYCCDChipInfo(
        h: *mut QhyCcdHandle,
        chipw: *mut f64,
        chiph: *mut f64,
        imagew: *mut u32,
        imageh: *mut u32,
        pixelw: *mut f64,
        pixelh: *mut f64,
        bpp: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDEffectiveArea(
        h: *mut QhyCcdHandle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDOverScanArea(
        h: *mut QhyCcdHandle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDCurrentROI(
        handle: *mut QhyCcdHandle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDCameraStatus(h: *mut QhyCcdHandle, buf: *mut u8) -> u32;
    pub fn SetQHYCCDDebayerOnOff(h: *mut QhyCcdHandle, onoff: bool) -> u32;
    pub fn GetQHYCCDSDKVersion(
        year: *mut u32,
        month: *mut u32,
        day: *mut u32,
        subday: *mut u32,
    ) -> u32;
    pub fn QHYCCDEqualizeHistogram(
        pdata: *mut u8,
        width: i32,
        height: i32,
        bpp: i32,
    );
}
