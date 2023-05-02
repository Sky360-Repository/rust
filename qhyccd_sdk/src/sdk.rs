#[path = "c_bindings.rs"]
mod c_bindings;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct QhyCcd {
    handle: *mut c_bindings::qhyccd_handle,
}

pub enum CameraStatus {
    Idle,
    Waiting,
    Exposing,
    Reading,
    Unknown(u8),
}

#[repr(u32)]
#[derive(Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SdkError {
    Success = 0,
    NotCool = 1,
    Cool = 2,
    Mono= 3,
    Color = 4,
    USBAsync = 5,
    USBSync = 6,
    QGIGAE = 7,
    WinPCap = 8,
    PCIE = 9,
    Delay200ms = 8192,
    ReadDirectly = 8193,
    Error = 4294967295
}

impl Default for SdkError {
    fn default() -> Self {
        SdkError::Error
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum Bayer {
    GB = 1,
    GR = 2,
    BG = 3,
    RG = 4,
    Unknown = 4294967295
}

impl Default for Bayer {
    fn default() -> Self {
        Bayer::Unknown
    }
}

pub enum ControlId {
    ControlBrightness = 0,
    ControlContrast = 1,
    ControlWbr = 2,
    ControlWbb = 3,
    ControlWbg = 4,
    ControlGamma = 5,
    ControlGain = 6,
    ControlOffset = 7,
    ControlExposure = 8,
    ControlSpeed = 9,
    ControlTransferBit = 10,
    ControlChannels = 11,
    ControlUsbTraffic = 12,
    ControlRowNoisere = 13,
    ControlCurTemp = 14,
    ControlCurPwm = 15,
    ControlManulPwm = 16,
    ControlCfwPort = 17,
    ControlCooler = 18,
    ControlSt4Port = 19,
    CamColor = 20,
    CamBin1x1Mode = 21,
    CamBin2x2Mode = 22,
    CamBin3x3Mode = 23,
    CamBin4x4Mode = 24,
    CamMechanicalShutter = 25,
    CamTrigerInterface = 26,
    CamTeCoverProtectInterface = 27,
    CamSingnalClampInterface = 28,
    CamFinetoneInterface = 29,
    CamShutterMotorHeatingInterface = 30,
    CamCalibrateFpnInterface = 31,
    CamChipTemperatureSensorInterface = 32,
    CamUsBreadoutSlowestInterface = 33,
    Cam8bits = 34,
    Cam16bits = 35,
    CamGps = 36,
    CamIgnoreOverscanInterface = 37,
    Qhyccd3aAutoexposure = 39,
    Qhyccd3aAutofocus = 40,
    ControlAmpv = 41,
    ControlVcam = 42,
    CamViewMode = 43,
    ControlCfwSlotsNum = 44,
    IsExposingDone = 45,
    ScreenStretchB = 46,
    ScreenStretchW = 47,
    ControlDdr = 48,
    CamLightPerformanceMode = 49,
    CamQhy5iiGuideMode = 50,
    DdrBufferCapacity = 51,
    DdrBufferReadThreshold = 52,
    DefaultGain = 53,
    DefaultOffset = 54,
    OutputDataActualBits = 55,
    OutputDataAlignment = 56,
    CamSingleFrameMode = 57,
    CamLiveVideoMode = 58,
    CamIsColor = 59,
    HasHardwareFrameCounter = 60,
    ControlMaxIdError = 61,
    CamHumidity = 62,
    CamPressure = 63,
    ControlVacuumPump = 64,
    ControlSensorChamberCyclePump = 65,
    Cam32bits = 66,
    CamSensorUlvoStatus = 67,
    CamSensorPhaseReTrain = 68,
    CamInitConfigFromFlash = 69,
    CamTrigerMode = 70,
    CamTrigerOut = 71,
    CamBurstMode = 72,
    CamSpeakerLedAlarm = 73,
    CamWatchDogFpga = 74,
    CamBin6x6mode = 75,
    CamBin8x8mode = 76,
    CamGlobalSensorGpsLed = 77,
    ControlImgProc = 78,
    ControlRemoveRbi = 79,
    ControlMaxId = 80,
    ControlAutoWhitebalance = 1024,
    ControlAutoExposure = 1025,
}

impl QhyCcd {
    pub fn init_resource() -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::InitQHYCCDResource() };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn release_resource() -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::ReleaseQHYCCDResource() };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn scan() -> u32 {
        unsafe { c_bindings::ScanQHYCCD() }
    }

    pub fn open(id: &str) -> Result<Self, SdkError> {
        let id_cstring = CString::new(id).unwrap();
        let handle = unsafe { c_bindings::OpenQHYCCD(id_cstring.as_ptr() as *mut c_char) };
        if handle.is_null() {
            Err(SdkError::Error)
        } else {
            Ok(Self { handle })
        }
    }

    pub fn close(&mut self) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::CloseQHYCCD(self.handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn get_id(index: u32) -> Result<String, SdkError> {
        let mut id = vec![0 as c_char; 256]; // Assuming the maximum ID length is 256
    
        let ret = unsafe { c_bindings::GetQHYCCDId(index, id.as_mut_ptr()) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => {
                let c_str = unsafe { CStr::from_ptr(id.as_ptr()) };
                c_str.to_str().map(|s| s.to_owned()).map_err(|_| error_result)
            },
            _ => Err(error_result)
        }
    }
    
    pub fn get_model(id: &str) -> Result<String, SdkError> {
        let c_id = CString::new(id).unwrap();
        let mut model = vec![0 as c_char; 256]; // Assuming the maximum model length is 256
    
        let ret = unsafe { c_bindings::GetQHYCCDModel(c_id.as_ptr() as *mut c_char, model.as_mut_ptr()) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => {
                let c_str = unsafe { CStr::from_ptr(model.as_ptr()) };
                c_str.to_str().map(|s| s.to_owned()).map_err(|_| error_result)
            },
            _ => Err(error_result)
        }
    }
    
    pub fn init(handle: *mut c_bindings::qhyccd_handle) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::InitQHYCCD(handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }    
    
    pub fn set_stream_mode(handle: *mut c_bindings::qhyccd_handle, mode: u8) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::SetQHYCCDStreamMode(handle, mode) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }
    
    pub fn is_control_available(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID) -> Result<bool, Bayer> {
        let ret = unsafe { c_bindings::IsQHYCCDControlAvailable(handle, control_id) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(true),
            SdkError::Error => Ok(false),
            _ => {
                let bayer = Bayer::try_from(error_result as u32).unwrap_or_default();
                Err(bayer)
            }
        }
    }
    
    pub fn set_param(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID, value: f64) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::SetQHYCCDParam(handle, control_id, value) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn get_param(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID) -> f64 {
        unsafe { c_bindings::GetQHYCCDParam(handle, control_id) }
    }

    pub fn get_param_min_max_step(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID) -> Result<(f64, f64, f64), SdkError> {
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
        let mut step: f64 = 0.0;
    
        let ret = unsafe {
            c_bindings::GetQHYCCDParamMinMaxStep(handle, control_id, &mut min as *mut f64, &mut max as *mut f64, &mut step as *mut f64)
        };
    
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((min, max, step)),
            _ => Err(error_result)
        }
    }

    pub fn set_resolution(
        handle: *mut c_bindings::qhyccd_handle,
        x: u32,
        y: u32,
        xsize: u32,
        ysize: u32,
    ) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::SetQHYCCDResolution(handle, x, y, xsize, ysize) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn get_mem_length(handle: *mut c_bindings::qhyccd_handle) -> Result<u32, SdkError> {
        let ret = unsafe { c_bindings::GetQHYCCDMemLength(handle) };  
        if ret != 0 {
            Ok(ret)
        } else {
            Err(SdkError::Error)
        }
    }
    
    pub fn exp_single_frame(handle: *mut c_bindings::qhyccd_handle) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::ExpQHYCCDSingleFrame(handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn get_single_frame(handle: *mut c_bindings::qhyccd_handle, buffer: &mut [u8]) -> Result<(u32, u32, u32, u32), SdkError> {
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        let mut bpp: u32 = 0;
        let mut channels: u32 = 0;
    
        let ret = unsafe {
            c_bindings::GetQHYCCDSingleFrame(
                handle,
                &mut w as *mut u32,
                &mut h as *mut u32,
                &mut bpp as *mut u32,
                &mut channels as *mut u32,
                buffer.as_mut_ptr(),
            )
        };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((w, h, bpp, channels)),
            _ => Err(error_result)
        }
    }

    pub fn cancel_exposing(handle: *mut c_bindings::qhyccd_handle) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::CancelQHYCCDExposing(handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }
    
    pub fn cancel_exposing_and_readout(handle: *mut c_bindings::qhyccd_handle) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::CancelQHYCCDExposingAndReadout(handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }
    
    pub fn begin_live(handle: *mut c_bindings::qhyccd_handle) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::BeginQHYCCDLive(handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn stop_live(handle: *mut c_bindings::qhyccd_handle) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::StopQHYCCDLive(handle) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }
    
    pub fn get_live_frame(handle: *mut c_bindings::qhyccd_handle, buffer: &mut [u8]) -> Result<(u32, u32, u32, u32), SdkError> {
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        let mut bpp: u32 = 0;
        let mut channels: u32 = 0;

        let ret = unsafe {
            c_bindings::GetQHYCCDLiveFrame(
                handle,
                &mut w as *mut u32,
                &mut h as *mut u32,
                &mut bpp as *mut u32,
                &mut channels as *mut u32,
                buffer.as_mut_ptr(),
            )
        };
    
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((w, h, bpp, channels)),
            _ => Err(error_result)
        }
    }

    pub fn set_bin_mode(handle: *mut c_bindings::qhyccd_handle, wbin: u32, hbin: u32) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::SetQHYCCDBinMode(handle, wbin, hbin) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }
    
    pub fn set_bits_mode(handle: *mut c_bindings::qhyccd_handle, bits: u32) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::SetQHYCCDBitsMode(handle, bits) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }
    
    pub fn set_control_temp(handle: *mut c_bindings::qhyccd_handle, targettemp: f64) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::ControlQHYCCDTemp(handle, targettemp) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn get_chip_info(handle: *mut c_bindings::qhyccd_handle) -> Result<(f64, f64, u32, u32, f64, f64, u32), SdkError> {
        let mut chipw: f64 = 0.0;
        let mut chiph: f64 = 0.0;
        let mut imagew: u32 = 0;
        let mut imageh: u32 = 0;
        let mut pixelw: f64 = 0.0;
        let mut pixelh: f64 = 0.0;
        let mut bpp: u32 = 0;
    
        let ret = unsafe {
            c_bindings::GetQHYCCDChipInfo(
                handle,
                &mut chipw as *mut f64,
                &mut chiph as *mut f64,
                &mut imagew as *mut u32,
                &mut imageh as *mut u32,
                &mut pixelw as *mut f64,
                &mut pixelh as *mut f64,
                &mut bpp as *mut u32,
            )
        };
    
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((chipw, chiph, imagew, imageh, pixelw, pixelh, bpp)),
            _ => Err(error_result)
        }
    }

    pub fn get_effective_area(handle: *mut c_bindings::qhyccd_handle) -> Result<(u32, u32, u32, u32), SdkError> {
        let mut start_x: u32 = 0;
        let mut start_y: u32 = 0;
        let mut size_x: u32 = 0;
        let mut size_y: u32 = 0;

        let ret = unsafe {
            c_bindings::GetQHYCCDEffectiveArea(
                handle,
                &mut start_x as *mut u32,
                &mut start_y as *mut u32,
                &mut size_x as *mut u32,
                &mut size_y as *mut u32,
            )
        };

        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((start_x, start_y, size_x, size_y)),
            _ => Err(error_result)
        }
    }

    pub fn get_overscan_area(handle: *mut c_bindings::qhyccd_handle) -> Result<(u32, u32, u32, u32), SdkError> {
        let mut start_x: u32 = 0;
        let mut start_y: u32 = 0;
        let mut size_x: u32 = 0;
        let mut size_y: u32 = 0;

        let ret = unsafe {
            c_bindings::GetQHYCCDOverScanArea(
                handle,
                &mut start_x as *mut u32,
                &mut start_y as *mut u32,
                &mut size_x as *mut u32,
                &mut size_y as *mut u32,
            )
        };

        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((start_x, start_y, size_x, size_y)),
            _ => Err(error_result)
        }
    }

    pub fn get_current_roi(handle: *mut c_bindings::qhyccd_handle) -> Result<(u32, u32, u32, u32), SdkError> {
        let mut start_x: u32 = 0;
        let mut start_y: u32 = 0;
        let mut size_x: u32 = 0;
        let mut size_y: u32 = 0;

        let ret = unsafe {
            c_bindings::GetQHYCCDCurrentROI(
                handle,
                &mut start_x as *mut u32,
                &mut start_y as *mut u32,
                &mut size_x as *mut u32,
                &mut size_y as *mut u32,
            )
        };

        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((start_x, start_y, size_x, size_y)),
            _ => Err(error_result)
        }
    }
    
    pub fn get_camera_status(handle: *mut c_bindings::qhyccd_handle) -> Result<CameraStatus, SdkError> {
        let mut buf = [0u8; 4];
        let ret = unsafe { c_bindings::GetQHYCCDCameraStatus(handle, buf.as_mut_ptr()) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();

        match error_result {
            SdkError::Success => {
                let status = match buf[0] {
                    0 => CameraStatus::Idle,
                    1 => CameraStatus::Waiting,
                    2 => CameraStatus::Exposing,
                    3 => CameraStatus::Reading,
                    unknown => CameraStatus::Unknown(unknown),
                };
                Ok(status)
            },
            _ => Err(error_result)
        }
    }
    
    pub fn set_debayer_on_off(handle: *mut c_bindings::qhyccd_handle, onoff: bool) -> Result<(), SdkError> {
        let ret = unsafe { c_bindings::SetQHYCCDDebayerOnOff(handle, onoff) };
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok(()),
            _ => Err(error_result)
        }
    }

    pub fn get_sdk_version() -> Result<(u32, u32, u32, u32), SdkError> {
        let mut year: u32 = 0;
        let mut month: u32 = 0;
        let mut day: u32 = 0;
        let mut subday: u32 = 0;
    
        let ret = unsafe {
            c_bindings::GetQHYCCDSDKVersion(&mut year as *mut u32, &mut month as *mut u32, &mut day as *mut u32, &mut subday as *mut u32)
        };
    
        let error_result = SdkError::try_from(ret).unwrap_or_default();
        match error_result {
            SdkError::Success => Ok((year, month, day, subday)),
            _ => Err(error_result)
        }
    }

    pub fn qhyccd_equalize_histogram(pdata: &mut [u8], width: i32, height: i32, bpp: i32) {
        unsafe {
            c_bindings::QHYCCDEqualizeHistogram(pdata.as_mut_ptr(), width, height, bpp);
        }
    }
}

impl Drop for QhyCcd {
    fn drop(&mut self) {
        let _ = self.close();
    }
}