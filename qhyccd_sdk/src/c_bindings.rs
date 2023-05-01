#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const QHYCCD_READ_DIRECTLY: u32 = 8193;
pub const QHYCCD_DELAY_200MS: u32 = 8192;
pub const QHYCCD_PCIE: u32 = 9;
pub const QHYCCD_WINPCAP: u32 = 8;
pub const QHYCCD_QGIGAE: u32 = 7;
pub const QHYCCD_USBSYNC: u32 = 6;
pub const QHYCCD_USBASYNC: u32 = 5;
pub const QHYCCD_COLOR: u32 = 4;
pub const QHYCCD_MONO: u32 = 3;
pub const QHYCCD_COOL: u32 = 2;
pub const QHYCCD_NOTCOOL: u32 = 1;
pub const QHYCCD_SUCCESS: u32 = 0;
pub const QHYCCD_ERROR: u32 = 4294967295;

pub const CONTROL_ID_CONTROL_BRIGHTNESS: CONTROL_ID = 0;
pub const CONTROL_ID_CONTROL_CONTRAST: CONTROL_ID = 1;
pub const CONTROL_ID_CONTROL_WBR: CONTROL_ID = 2;
pub const CONTROL_ID_CONTROL_WBB: CONTROL_ID = 3;
pub const CONTROL_ID_CONTROL_WBG: CONTROL_ID = 4;
pub const CONTROL_ID_CONTROL_GAMMA: CONTROL_ID = 5;
pub const CONTROL_ID_CONTROL_GAIN: CONTROL_ID = 6;
pub const CONTROL_ID_CONTROL_OFFSET: CONTROL_ID = 7;
pub const CONTROL_ID_CONTROL_EXPOSURE: CONTROL_ID = 8;
pub const CONTROL_ID_CONTROL_SPEED: CONTROL_ID = 9;
pub const CONTROL_ID_CONTROL_TRANSFERBIT: CONTROL_ID = 10;
pub const CONTROL_ID_CONTROL_CHANNELS: CONTROL_ID = 11;
pub const CONTROL_ID_CONTROL_USBTRAFFIC: CONTROL_ID = 12;
pub const CONTROL_ID_CONTROL_ROWNOISERE: CONTROL_ID = 13;
pub const CONTROL_ID_CONTROL_CURTEMP: CONTROL_ID = 14;
pub const CONTROL_ID_CONTROL_CURPWM: CONTROL_ID = 15;
pub const CONTROL_ID_CONTROL_MANULPWM: CONTROL_ID = 16;
pub const CONTROL_ID_CONTROL_CFWPORT: CONTROL_ID = 17;
pub const CONTROL_ID_CONTROL_COOLER: CONTROL_ID = 18;
pub const CONTROL_ID_CONTROL_ST4PORT: CONTROL_ID = 19;
pub const CONTROL_ID_CAM_COLOR: CONTROL_ID = 20;
pub const CONTROL_ID_CAM_BIN1X1MODE: CONTROL_ID = 21;
pub const CONTROL_ID_CAM_BIN2X2MODE: CONTROL_ID = 22;
pub const CONTROL_ID_CAM_BIN3X3MODE: CONTROL_ID = 23;
pub const CONTROL_ID_CAM_BIN4X4MODE: CONTROL_ID = 24;
pub const CONTROL_ID_CAM_MECHANICALSHUTTER: CONTROL_ID = 25;
pub const CONTROL_ID_CAM_TRIGER_INTERFACE: CONTROL_ID = 26;
pub const CONTROL_ID_CAM_TECOVERPROTECT_INTERFACE: CONTROL_ID = 27;
pub const CONTROL_ID_CAM_SINGNALCLAMP_INTERFACE: CONTROL_ID = 28;
pub const CONTROL_ID_CAM_FINETONE_INTERFACE: CONTROL_ID = 29;
pub const CONTROL_ID_CAM_SHUTTERMOTORHEATING_INTERFACE: CONTROL_ID = 30;
pub const CONTROL_ID_CAM_CALIBRATEFPN_INTERFACE: CONTROL_ID = 31;
pub const CONTROL_ID_CAM_CHIPTEMPERATURESENSOR_INTERFACE: CONTROL_ID = 32;
pub const CONTROL_ID_CAM_USBREADOUTSLOWEST_INTERFACE: CONTROL_ID = 33;
pub const CONTROL_ID_CAM_8BITS: CONTROL_ID = 34;
pub const CONTROL_ID_CAM_16BITS: CONTROL_ID = 35;
pub const CONTROL_ID_CAM_GPS: CONTROL_ID = 36;
pub const CONTROL_ID_CAM_IGNOREOVERSCAN_INTERFACE: CONTROL_ID = 37;
pub const CONTROL_ID_QHYCCD_3A_AUTOEXPOSURE: CONTROL_ID = 39;
pub const CONTROL_ID_QHYCCD_3A_AUTOFOCUS: CONTROL_ID = 40;
pub const CONTROL_ID_CONTROL_AMPV: CONTROL_ID = 41;
pub const CONTROL_ID_CONTROL_VCAM: CONTROL_ID = 42;
pub const CONTROL_ID_CAM_VIEW_MODE: CONTROL_ID = 43;
pub const CONTROL_ID_CONTROL_CFWSLOTSNUM: CONTROL_ID = 44;
pub const CONTROL_ID_IS_EXPOSING_DONE: CONTROL_ID = 45;
pub const CONTROL_ID_ScreenStretchB: CONTROL_ID = 46;
pub const CONTROL_ID_ScreenStretchW: CONTROL_ID = 47;
pub const CONTROL_ID_CONTROL_DDR: CONTROL_ID = 48;
pub const CONTROL_ID_CAM_LIGHT_PERFORMANCE_MODE: CONTROL_ID = 49;
pub const CONTROL_ID_CAM_QHY5II_GUIDE_MODE: CONTROL_ID = 50;
pub const CONTROL_ID_DDR_BUFFER_CAPACITY: CONTROL_ID = 51;
pub const CONTROL_ID_DDR_BUFFER_READ_THRESHOLD: CONTROL_ID = 52;
pub const CONTROL_ID_DefaultGain: CONTROL_ID = 53;
pub const CONTROL_ID_DefaultOffset: CONTROL_ID = 54;
pub const CONTROL_ID_OutputDataActualBits: CONTROL_ID = 55;
pub const CONTROL_ID_OutputDataAlignment: CONTROL_ID = 56;
pub const CONTROL_ID_CAM_SINGLEFRAMEMODE: CONTROL_ID = 57;
pub const CONTROL_ID_CAM_LIVEVIDEOMODE: CONTROL_ID = 58;
pub const CONTROL_ID_CAM_IS_COLOR: CONTROL_ID = 59;
pub const CONTROL_ID_hasHardwareFrameCounter: CONTROL_ID = 60;
pub const CONTROL_ID_CONTROL_MAX_ID_Error: CONTROL_ID = 61;
pub const CONTROL_ID_CAM_HUMIDITY: CONTROL_ID = 62;
pub const CONTROL_ID_CAM_PRESSURE: CONTROL_ID = 63;
pub const CONTROL_ID_CONTROL_VACUUM_PUMP: CONTROL_ID = 64;
pub const CONTROL_ID_CONTROL_SensorChamberCycle_PUMP: CONTROL_ID = 65;
pub const CONTROL_ID_CAM_32BITS: CONTROL_ID = 66;
pub const CONTROL_ID_CAM_Sensor_ULVO_Status: CONTROL_ID = 67;
pub const CONTROL_ID_CAM_SensorPhaseReTrain: CONTROL_ID = 68;
pub const CONTROL_ID_CAM_InitConfigFromFlash: CONTROL_ID = 69;
pub const CONTROL_ID_CAM_TRIGER_MODE: CONTROL_ID = 70;
pub const CONTROL_ID_CAM_TRIGER_OUT: CONTROL_ID = 71;
pub const CONTROL_ID_CAM_BURST_MODE: CONTROL_ID = 72;
pub const CONTROL_ID_CAM_SPEAKER_LED_ALARM: CONTROL_ID = 73;
pub const CONTROL_ID_CAM_WATCH_DOG_FPGA: CONTROL_ID = 74;
pub const CONTROL_ID_CAM_BIN6X6MODE: CONTROL_ID = 75;
pub const CONTROL_ID_CAM_BIN8X8MODE: CONTROL_ID = 76;
pub const CONTROL_ID_CAM_GlobalSensorGPSLED: CONTROL_ID = 77;
pub const CONTROL_ID_CONTROL_ImgProc: CONTROL_ID = 78;
pub const CONTROL_ID_CONTROL_RemoveRBI: CONTROL_ID = 79;
pub const CONTROL_ID_CONTROL_MAX_ID: CONTROL_ID = 80;
pub const CONTROL_ID_CONTROL_AUTOWHITEBALANCE: CONTROL_ID = 1024;
pub const CONTROL_ID_CONTROL_AUTOEXPOSURE: CONTROL_ID = 1025;
pub type CONTROL_ID = ::std::os::raw::c_uint;

pub const BAYER_ID_BAYER_GB: BAYER_ID = 1;
pub const BAYER_ID_BAYER_GR: BAYER_ID = 2;
pub const BAYER_ID_BAYER_BG: BAYER_ID = 3;
pub const BAYER_ID_BAYER_RG: BAYER_ID = 4;
pub type BAYER_ID = ::std::os::raw::c_uint;

pub type qhyccd_handle = ::std::os::raw::c_void;

extern "C" {
    pub fn InitQHYCCDResource() -> u32;
}
extern "C" {
    pub fn ReleaseQHYCCDResource() -> u32;
}
extern "C" {
    pub fn ScanQHYCCD() -> u32;
}
extern "C" {
    pub fn GetQHYCCDId(index: u32, id: *mut ::std::os::raw::c_char) -> u32;
}
extern "C" {
    pub fn GetQHYCCDModel(
        id: *mut ::std::os::raw::c_char,
        model: *mut ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn OpenQHYCCD(id: *mut ::std::os::raw::c_char) -> *mut qhyccd_handle;
}
extern "C" {
    pub fn CloseQHYCCD(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDStreamMode(handle: *mut qhyccd_handle, mode: u8) -> u32;
}
extern "C" {
    pub fn InitQHYCCD(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn IsQHYCCDControlAvailable(handle: *mut qhyccd_handle, controlId: CONTROL_ID) -> u32;
}
extern "C" {
    pub fn SetQHYCCDParam(handle: *mut qhyccd_handle, controlId: CONTROL_ID, value: f64) -> u32;
}
extern "C" {
    pub fn GetQHYCCDParam(handle: *mut qhyccd_handle, controlId: CONTROL_ID) -> f64;
}
extern "C" {
    pub fn GetQHYCCDParamMinMaxStep(
        handle: *mut qhyccd_handle,
        controlId: CONTROL_ID,
        min: *mut f64,
        max: *mut f64,
        step: *mut f64,
    ) -> u32;
}
extern "C" {
    pub fn SetQHYCCDResolution(
        handle: *mut qhyccd_handle,
        x: u32,
        y: u32,
        xsize: u32,
        ysize: u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDMemLength(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn ExpQHYCCDSingleFrame(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn GetQHYCCDSingleFrame(
        handle: *mut qhyccd_handle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn CancelQHYCCDExposing(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn CancelQHYCCDExposingAndReadout(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn BeginQHYCCDLive(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn GetQHYCCDLiveFrame(
        handle: *mut qhyccd_handle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn StopQHYCCDLive(handle: *mut qhyccd_handle) -> u32;
}
extern "C" {
    pub fn SetQHYCCDBinMode(handle: *mut qhyccd_handle, wbin: u32, hbin: u32) -> u32;
}
extern "C" {
    pub fn SetQHYCCDBitsMode(handle: *mut qhyccd_handle, bits: u32) -> u32;
}
extern "C" {
    pub fn ControlQHYCCDTemp(handle: *mut qhyccd_handle, targettemp: f64) -> u32;
}
extern "C" {
    pub fn GetQHYCCDChipInfo(
        h: *mut qhyccd_handle,
        chipw: *mut f64,
        chiph: *mut f64,
        imagew: *mut u32,
        imageh: *mut u32,
        pixelw: *mut f64,
        pixelh: *mut f64,
        bpp: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDEffectiveArea(
        h: *mut qhyccd_handle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDOverScanArea(
        h: *mut qhyccd_handle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDCurrentROI(
        handle: *mut qhyccd_handle,
        startX: *mut u32,
        startY: *mut u32,
        sizeX: *mut u32,
        sizeY: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn GetQHYCCDCameraStatus(h: *mut qhyccd_handle, buf: *mut u8) -> u32;
}
extern "C" {
    pub fn SetQHYCCDDebayerOnOff(h: *mut qhyccd_handle, onoff: bool) -> u32;
}
extern "C" {
    pub fn GetQHYCCDSDKVersion(
        year: *mut u32,
        month: *mut u32,
        day: *mut u32,
        subday: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn QHYCCDEqualizeHistogram(
        pdata: *mut u8,
        width: i32,
        height: i32,
        bpp: i32,
    );
}
