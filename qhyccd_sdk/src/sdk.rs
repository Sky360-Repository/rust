#[path = "c_bindings.rs"]
mod c_bindings;

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

impl QhyCcd {
    pub fn init_resource() -> Result<(), u32> {
        let ret = unsafe { c_bindings::InitQHYCCDResource() };
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn release_resource() -> Result<(), u32> {
        let ret = unsafe { c_bindings::ReleaseQHYCCDResource() };
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn scan() -> u32 {
        unsafe { c_bindings::ScanQHYCCD() }
    }

    // Other functions can be wrapped in a similar way

    pub fn open(id: &str) -> Result<Self, u32> {
        let id_cstring = CString::new(id).unwrap();
        let handle = unsafe { c_bindings::OpenQHYCCD(id_cstring.as_ptr() as *mut c_char) };

        if handle.is_null() {
            Err(c_bindings::QHYCCD_ERROR as u32)
        } else {
            Ok(Self { handle })
        }
    }

    pub fn close(&mut self) -> Result<(), u32> {
        let ret = unsafe { c_bindings::CloseQHYCCD(self.handle) };
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn get_id(index: u32) -> Result<String, u32> {
        let mut id = vec![0 as c_char; 256]; // Assuming the maximum ID length is 256
    
        let ret = unsafe { c_bindings::GetQHYCCDId(index, id.as_mut_ptr()) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            let c_str = unsafe { CStr::from_ptr(id.as_ptr()) };
            c_str.to_str().map(|s| s.to_owned()).map_err(|_| ret)
        } else {
            Err(ret)
        }
    }
    
    pub fn get_model(id: &str) -> Result<String, u32> {
        let c_id = CString::new(id).unwrap();
        let mut model = vec![0 as c_char; 256]; // Assuming the maximum model length is 256
    
        let ret = unsafe { c_bindings::GetQHYCCDModel(c_id.as_ptr() as *mut c_char, model.as_mut_ptr()) };
    
        if ret == 0 {
            let c_str = unsafe { CStr::from_ptr(model.as_ptr()) };
            c_str.to_str().map(|s| s.to_owned()).map_err(|_| ret)
        } else {
            Err(ret)
        }
    }
    
    pub fn init(handle: *mut c_bindings::qhyccd_handle) -> Result<(), u32> {
        let ret = unsafe { c_bindings::InitQHYCCD(handle) };
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }    
    
    pub fn set_stream_mode(handle: *mut c_bindings::qhyccd_handle, mode: u8) -> Result<(), u32> {
        let ret = unsafe { c_bindings::SetQHYCCDStreamMode(handle, mode) };
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn is_control_available(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID) -> Result<(), u32> {
        let ret = unsafe { c_bindings::IsQHYCCDControlAvailable(handle, control_id) };
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn set_param(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID, value: f64) -> Result<(), String> {
        let control_available_result = Self::is_control_available(handle, control_id);
        match control_available_result {
            Ok(()) => {
                let ret = unsafe { c_bindings::SetQHYCCDParam(handle, control_id, value) };
                if ret == c_bindings::QHYCCD_SUCCESS as u32 {
                    Ok(())
                } else {
                    Err(format!("Error setting Control with ID {}.", control_id))
                }
            },
            Err(error_code) => Err(format!("Control with ID {:?} is not available. Error code: {}", control_id, error_code))
        }
    }

    pub fn get_param(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID) -> Result<f64, String> {
        let control_available_result = Self::is_control_available(handle, control_id);
        match control_available_result {
            Ok(()) => {
                let value = unsafe { c_bindings::GetQHYCCDParam(handle, control_id) };
                Ok(value)
            },
            Err(error_code) => Err(format!("Control with ID {:?} is not available. Error code: {}", control_id, error_code))
        }
    }

    pub fn get_param_min_max_step(handle: *mut c_bindings::qhyccd_handle, control_id: c_bindings::CONTROL_ID) -> Result<(f64, f64, f64), u32> {
        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
        let mut step: f64 = 0.0;
    
        let ret = unsafe {
            c_bindings::GetQHYCCDParamMinMaxStep(handle, control_id, &mut min as *mut f64, &mut max as *mut f64, &mut step as *mut f64)
        };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((min, max, step))
        } else {
            Err(ret)
        }
    }

    pub fn set_resolution(
        handle: *mut c_bindings::qhyccd_handle,
        x: u32,
        y: u32,
        xsize: u32,
        ysize: u32,
    ) -> Result<(), u32> {
        let ret = unsafe { c_bindings::SetQHYCCDResolution(handle, x, y, xsize, ysize) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn get_mem_length(handle: *mut c_bindings::qhyccd_handle) -> Result<u32, u32> {
        let ret = unsafe { c_bindings::GetQHYCCDMemLength(handle) };
    
        if ret != 0 {
            Ok(ret)
        } else {
            Err(ret)
        }
    }
    
    pub fn exp_single_frame(handle: *mut c_bindings::qhyccd_handle) -> Result<(), u32> {
        let ret = unsafe { c_bindings::ExpQHYCCDSingleFrame(handle) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn get_single_frame(handle: *mut c_bindings::qhyccd_handle, buffer: &mut [u8]) -> Result<(u32, u32, u32, u32), u32> {
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
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((w, h, bpp, channels))
        } else {
            Err(ret)
        }
    }

    pub fn cancel_exposing(handle: *mut c_bindings::qhyccd_handle) -> Result<(), u32> {
        let ret = unsafe { c_bindings::CancelQHYCCDExposing(handle) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn cancel_exposing_and_readout(handle: *mut c_bindings::qhyccd_handle) -> Result<(), u32> {
        let ret = unsafe { c_bindings::CancelQHYCCDExposingAndReadout(handle) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn begin_live(handle: *mut c_bindings::qhyccd_handle) -> Result<(), u32> {
        let ret = unsafe { c_bindings::BeginQHYCCDLive(handle) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn stop_live(handle: *mut c_bindings::qhyccd_handle) -> Result<(), u32> {
        let ret = unsafe { c_bindings::StopQHYCCDLive(handle) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn set_bin_mode(handle: *mut c_bindings::qhyccd_handle, wbin: u32, hbin: u32) -> Result<(), u32> {
        let ret = unsafe { c_bindings::SetQHYCCDBinMode(handle, wbin, hbin) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn set_bits_mode(handle: *mut c_bindings::qhyccd_handle, bits: u32) -> Result<(), u32> {
        let ret = unsafe { c_bindings::SetQHYCCDBitsMode(handle, bits) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }
    
    pub fn set_control_temp(handle: *mut c_bindings::qhyccd_handle, targettemp: f64) -> Result<(), u32> {
        let ret = unsafe { c_bindings::ControlQHYCCDTemp(handle, targettemp) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn get_chip_info(handle: *mut c_bindings::qhyccd_handle) -> Result<(f64, f64, u32, u32, f64, f64, u32), u32> {
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
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((chipw, chiph, imagew, imageh, pixelw, pixelh, bpp))
        } else {
            Err(ret)
        }
    }

    pub fn get_effective_area(handle: *mut c_bindings::qhyccd_handle) -> Result<(u32, u32, u32, u32), u32> {
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

        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((start_x, start_y, size_x, size_y))
        } else {
            Err(ret)
        }
    }

    pub fn get_overscan_area(handle: *mut c_bindings::qhyccd_handle) -> Result<(u32, u32, u32, u32), u32> {
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

        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((start_x, start_y, size_x, size_y))
        } else {
            Err(ret)
        }
    }

    pub fn get_current_roi(handle: *mut c_bindings::qhyccd_handle) -> Result<(u32, u32, u32, u32), u32> {
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

        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((start_x, start_y, size_x, size_y))
        } else {
            Err(ret)
        }
    }
    
    pub fn get_camera_status(handle: *mut c_bindings::qhyccd_handle) -> Result<CameraStatus, u32> {
        let mut buf = [0u8; 4];
        let ret = unsafe { c_bindings::GetQHYCCDCameraStatus(handle, buf.as_mut_ptr()) };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            let status = match buf[0] {
                0 => CameraStatus::Idle,
                1 => CameraStatus::Waiting,
                2 => CameraStatus::Exposing,
                3 => CameraStatus::Reading,
                unknown => CameraStatus::Unknown(unknown),
            };
            Ok(status)
        } else {
            Err(ret)
        }
    }
    
    pub fn set_debayer_on_off(handle: *mut c_bindings::qhyccd_handle, onoff: bool) -> Result<(), u32> {
        let ret = unsafe { c_bindings::SetQHYCCDDebayerOnOff(handle, onoff) };

        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub fn get_sdk_version() -> Result<(u32, u32, u32, u32), u32> {
        let mut year: u32 = 0;
        let mut month: u32 = 0;
        let mut day: u32 = 0;
        let mut subday: u32 = 0;
    
        let ret = unsafe {
            c_bindings::GetQHYCCDSDKVersion(&mut year as *mut u32, &mut month as *mut u32, &mut day as *mut u32, &mut subday as *mut u32)
        };
    
        if ret == c_bindings::QHYCCD_SUCCESS as u32 {
            Ok((year, month, day, subday))
        } else {
            Err(ret)
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