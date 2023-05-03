use std::fmt;
use std::collections::HashMap;
use derive_more::Display;

#[path = "c_bindings.rs"]
mod c_bindings;

use crate::sdk::{self, ControlId};

pub struct Camera {
    debug_info: bool,
    cam_id: String,
    cam_handle: *mut c_bindings::QhyCcdHandle,
    img_data: Vec<u8>,
    cameras: HashMap<String, CameraInfo>,
    params: CameraParams,
    current_info: CameraInfo,
    last_frame_capture_time: f64,

    cam_init: bool,
    cam_open: bool,
    is_exposing: bool,
    default_set: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinMode {
    Bin1x1 = 1,
    Bin2x2 = 2,
    Bin3x3 = 3,
    Bin4x4 = 4,
}

#[derive(Debug, Clone, Default)]
pub struct ParamLimits {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

#[derive(Debug, Clone, Default)]
pub struct CameraInfo {
    pub id: String,
    pub model: String,
    pub serial_num: String,

    pub overscan_start_x: u32,
    pub overscan_start_y: u32,
    pub overscan_width: u32,
    pub overscan_height: u32,

    pub effective_start_x: u32,
    pub effective_start_y: u32,
    pub effective_width: u32,
    pub effective_height: u32,

    pub chip_width_mm: f64,
    pub chip_height_mm: f64,

    pub pixel_width_um: f64,
    pub pixel_height_um: f64,

    pub max_image_width: u32,
    pub max_image_height: u32,

    pub bpp: u32,

    pub bayer_format: sdk::BayerFormat,
    pub is_color: bool,

    pub has_bin1x1_mode: bool,
    pub has_bin2x2_mode: bool,
    pub has_bin3x3_mode: bool,
    pub has_bin4x4_mode: bool,

    pub gain_limits: ParamLimits,
    pub offset_limits: ParamLimits,
    pub usb_traffic_limits: ParamLimits,
    pub red_wb_limits: ParamLimits,
    pub green_wb_limits: ParamLimits,
    pub blue_wb_limits: ParamLimits,
}

impl CameraInfo {
    pub fn bayer_format_to_string(&self) -> &str {
        match self.bayer_format {
            sdk::BayerFormat::GB => "BAYER_GB",
            sdk::BayerFormat::GR => "BAYER_GR",
            sdk::BayerFormat::BG => "BAYER_BG",
            sdk::BayerFormat::RG => "BAYER_RG",
            sdk::BayerFormat::Mono => "MONO",
        }
    }
}

impl fmt::Display for CameraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bin_modes = format!(
            "{}{}{}{}",
            if self.has_bin1x1_mode { " (1x1)" } else { "" },
            if self.has_bin2x2_mode { " (2x2)" } else { "" },
            if self.has_bin3x3_mode { " (3x3)" } else { "" },
            if self.has_bin4x4_mode { " (4x4)" } else { "" }
        );

        write!(f, "Camera model: {}, Serial: {}, Id: {}\n\
        Overscan  Area startX x startY: {} x {}, sizeX x sizeY : {} x {}\n\
        Effective Area startX x startY: {} x {}, sizeX x sizeY : {} x {}\n\
        Chip      Size width x height: {} x {} [mm]\n\
        Max Image Size width x height: {} x {}\n\
        Pixel     Size width x height: {} x {} [um]\n\
        Bits per Pixel: {}\n\
        Camera is color: {}, Bayer Pattern: {}\n\
        Available Bin modes:{}\n\
        Gain Limits: Min: {}, Max: {}, Step: {}\n\
        Offset Limits: Min: {}, Max: {}, Step: {}\n\
        Usb Traffic Limits: Min: {}, Max: {}, Step: {}",
       self.model,
       self.serial_num,
       self.id,
       self.overscan_start_x,
       self.overscan_start_y,
       self.overscan_width,
       self.overscan_height,
       self.effective_start_x,
       self.effective_start_y,
       self.effective_width,
       self.effective_height,
       self.chip_width_mm,
       self.chip_height_mm,
       self.max_image_width,
       self.max_image_height,
       self.pixel_width_um,
       self.pixel_height_um,
       self.bpp,
       if self.is_color { "Yes" } else { "No" },
       self.bayer_format_to_string(),
       bin_modes,
       self.gain_limits.min,
       self.gain_limits.max,
       self.gain_limits.step,
       self.offset_limits.min,
       self.offset_limits.max,
       self.offset_limits.step,
       self.usb_traffic_limits.min,
       self.usb_traffic_limits.max,
       self.usb_traffic_limits.step)
    }
}

#[derive(Debug, Clone)]
pub struct CameraParams {
    pub roi_start_x: u32,
    pub roi_start_y: u32,
    pub roi_width: u32,
    pub roi_height: u32,

    pub apply_debayer: bool,
    pub red_wb: f64,
    pub green_wb: f64,
    pub blue_wb: f64,

    pub exposure_time: u32,
    pub contrast: f64,
    pub brightness: f64,
    pub gamma: f64,

    pub stream_mode: sdk::StreamMode,

    pub channels: u32,
    pub usb_traffic: u32,
    pub usb_speed: u32,
    pub gain: u32,
    pub offset: u32,
    pub bin_mode: BinMode,

    pub transfer_bits: u32,
}

impl Default for CameraParams {
    fn default() -> Self {
        CameraParams {
            roi_start_x: 0,
            roi_start_y: 0,
            roi_width: 0,
            roi_height: 0,

            apply_debayer: false,
            red_wb: 1.0,
            green_wb: 1.0,
            blue_wb: 1.0,

            exposure_time: 0,
            contrast: 1.0,
            brightness: 0.0,
            gamma: 1.0,

            stream_mode: sdk::StreamMode::LiveFrame,

            channels: 0,
            usb_traffic: 0,
            usb_speed: 0,
            gain: 0,
            offset: 0,
            bin_mode: BinMode::Bin1x1,

            transfer_bits: 0,
        }
    }
}

#[repr(u32)]
#[derive(Display, Debug, Clone, PartialEq)]
pub enum ControlParam {
    Brightness = ControlId::ControlBrightness as u32,
    Contrast = ControlId::ControlContrast as u32,
    Exposure = ControlId::ControlExposure as u32,
    UsbTraffic = ControlId::ControlUsbTraffic as u32,
    UsbSpeed = ControlId::ControlSpeed as u32,
    Gain = ControlId::ControlGain as u32,
    Offset = ControlId::ControlOffset as u32,
    TransferBits = ControlId::ControlTransferBit as u32,
    RedWB = ControlId::ControlWbr as u32,
    BlueWB = ControlId::ControlWbb as u32,
    GreenWB = ControlId::ControlWbg as u32,
    Gamma = ControlId::ControlGamma as u32,
    Channels = ControlId::ControlChannels as u32,
}

use sdk::QhyCcd;

impl Camera {
    pub fn new() -> Self {
        QhyCcd::enable_message(false);
        QhyCcd::enable_log_file(false);
        Camera {
            debug_info: false,
            cam_id: String::new(),
            cam_handle: std::ptr::null_mut(),
            img_data: vec![0; 1],
            cameras: HashMap::new(),
            params: CameraParams::default(),
            current_info: CameraInfo::default(),
            last_frame_capture_time: 0.0,

            cam_init: false,
            cam_open: false,
            is_exposing: false,
            default_set: false,
        }
    }

    pub fn init(&mut self) -> bool {
        if !self.cam_init {
            let ret = QhyCcd::init_resource();
            match ret {
                Ok(()) => {
                    self.cam_init = true;
                },
                Err(err) => {
                    eprintln!("Cannot initialize SDK resources: {}", err);
                    self.cam_init = false;
                }
            }
        }

        self.cam_init
    }

    pub fn close(&mut self) {
        if self.cam_open
        {
            if self.params.stream_mode == sdk::StreamMode::SingleFrame {
                let _ = QhyCcd::cancel_exposing_and_readout(self.cam_handle);
            } else {
                let _ = QhyCcd::stop_live(self.cam_handle);
            }

            let _ = QhyCcd::close(self.cam_handle);

            self.cam_handle = std::ptr::null_mut();
            self.cam_id = String::new();
            self.cam_open = false;
            self.is_exposing = false;
        }
    }

    pub fn release(&mut self) {
        if self.cam_open {
            self.close();
        }

        if self.cam_init {
            let res = QhyCcd::release_resource();
            if res.is_err() {
                eprintln!("Cannot release SDK resources, error: {}", res.unwrap_err());
            }
            self.cam_init = false;
        }
    }

    fn scan_cameras(&mut self) -> bool {
        self.init();

        self.cameras.clear();

        let cam_count = QhyCcd::scan();
        if cam_count == 0 {
            return false
        }
        for index in 0..cam_count {
            let cam_id_res = QhyCcd::get_id(index);
            if cam_id_res.is_ok() {
                let cam_id = cam_id_res.unwrap();
                let ci = self.fill_camera_info(&cam_id);
                if ci.is_some() {
                    self.cameras.insert(cam_id.clone(), ci.unwrap());
                }
            }
        }

        if self.cameras.is_empty() {
            self.release();
            return false
        }

        true
    }

    fn fill_camera_info(&mut self, cam_id: &String) -> Option<CameraInfo> {
        let res = QhyCcd::open(cam_id);
        if res.is_err() {
            eprintln!("OpenQHYCCD failure, camera id: {}", cam_id);
            return None
        }
        let handle = res.unwrap();

        let pos_dash = cam_id.find("-").unwrap();
        let model = &cam_id[..pos_dash];
        let serial_num = &cam_id[pos_dash + 1..];

        let overscan = QhyCcd::get_overscan_area(handle).unwrap();
        let effective = QhyCcd::get_effective_area(handle).unwrap();
        let chip_info = QhyCcd::get_chip_info(handle).unwrap();
        let bayer_format = QhyCcd::is_control_available(handle, &sdk::ControlId::CamColor).unwrap_err();
        let has_bin1x1_mode = QhyCcd::is_control_available(handle, &sdk::ControlId::CamBin1x1Mode).unwrap();
        let has_bin2x2_mode = QhyCcd::is_control_available(handle, &sdk::ControlId::CamBin2x2Mode).unwrap();
        let has_bin3x3_mode = QhyCcd::is_control_available(handle, &sdk::ControlId::CamBin3x3Mode).unwrap();
        let has_bin4x4_mode = QhyCcd::is_control_available(handle, &sdk::ControlId::CamBin4x4Mode).unwrap();
        let gain_limits = QhyCcd::get_param_min_max_step(handle, &sdk::ControlId::ControlGain).unwrap();
        let offset_limits = QhyCcd::get_param_min_max_step(handle, &sdk::ControlId::ControlOffset).unwrap();
        let usb_traffic_limits = QhyCcd::get_param_min_max_step(handle, &sdk::ControlId::ControlUsbTraffic).unwrap();
        let red_wb_limits = QhyCcd::get_param_min_max_step(handle, &sdk::ControlId::ControlWbr).unwrap();
        let green_wb_limits = QhyCcd::get_param_min_max_step(handle, &sdk::ControlId::ControlWbg).unwrap();
        let blue_wb_limits = QhyCcd::get_param_min_max_step(handle, &sdk::ControlId::ControlWbb).unwrap();

        let ci = CameraInfo {
            id: cam_id.to_string(),
            model: model.to_string(),
            serial_num: serial_num.to_string(),
            overscan_start_x: overscan.0,
            overscan_start_y: overscan.1,
            overscan_width: overscan.2,
            overscan_height: overscan.3,
            effective_start_x: effective.0,
            effective_start_y: effective.1,
            effective_width: effective.2,
            effective_height: effective.3,
            chip_width_mm: chip_info.0,
            chip_height_mm: chip_info.1,
            max_image_width: chip_info.2,
            max_image_height: chip_info.3,
            pixel_width_um: chip_info.4,
            pixel_height_um: chip_info.5,
            bpp: chip_info.6,
            is_color: bayer_format != sdk::BayerFormat::Mono,
            bayer_format,
            has_bin1x1_mode,
            has_bin2x2_mode,
            has_bin3x3_mode,
            has_bin4x4_mode,
            gain_limits: ParamLimits { max: gain_limits.1, min: gain_limits.0, step: gain_limits.2 },
            offset_limits: ParamLimits { max: offset_limits.1, min: offset_limits.0, step: offset_limits.2 },
            usb_traffic_limits: ParamLimits { max: usb_traffic_limits.1, min: usb_traffic_limits.0, step: usb_traffic_limits.2 },
            red_wb_limits: ParamLimits { max: red_wb_limits.1, min: red_wb_limits.0, step: red_wb_limits.2 },
            green_wb_limits: ParamLimits { max: green_wb_limits.1, min: green_wb_limits.0, step: green_wb_limits.2 },
            blue_wb_limits: ParamLimits { max: blue_wb_limits.1, min: blue_wb_limits.0, step: blue_wb_limits.2 },
        };

        let _ = QhyCcd::close(handle);
        if self.debug_info {
            println!("{}", ci);
        }

        Some(ci)
    }

    pub fn set_debug_info(&mut self, enable: bool) {
        self.debug_info = enable;
    }

    pub fn get_cameras(&mut self) -> &HashMap<String, CameraInfo> {
        self.scan_cameras();
        &self.cameras
    }

    pub fn open(&mut self, camera_id: &str) -> bool {
        if !self.cam_init && !self.init() {
            return false
        }
        let mut cam_id = camera_id.to_string();
        if !self.cam_open
        {
            if cam_id.is_empty()
            {
                if !self.scan_cameras()
                {
                    return false
                }
                let camera_iter = self.cameras.iter().next();
                cam_id = camera_iter.unwrap().1.id.clone();
            }
            // else if self.cam_id.is_some() && cam_id.as_str() != self.cam_id.map(|s| s.as_str())
            else if self.cam_id != cam_id
            {
                self.default_set = false;
            }
            self.cam_id = cam_id.clone();
            let has_info = self.cameras.get(&cam_id);
            if has_info.is_none() {
                return false
            }
            self.current_info = has_info.unwrap().clone();

            let has_cam_handle = QhyCcd::open(&cam_id);
            if has_cam_handle.is_err()
            {
                self.cam_handle = std::ptr::null_mut();
                self.cam_id = String::new();
                self.current_info = CameraInfo::default();
                eprintln!("OpenQHYCCD failure, camera id: {}", cam_id);
                return false
            }
            self.cam_handle = has_cam_handle.unwrap();

            self.set_default_params();
            self.cam_open = true;
        }

        self.cam_open
    }

    fn set_default_params(&mut self) {
        if !self.default_set
        {
            self.set_debayer(false);
            // setControl(RedWB, 180.0, true);
            // setControl(GreenWB, 128.0, true);
            // setControl(BlueWB, 190.0, true);
            // setControl(Exposure, 2000, true);
            self.set_stream_mode(&sdk::StreamMode::LiveFrame);
            // setControl(UsbTraffic, 5, true);
            // setControl(UsbSpeed, 0, true);
            // setControl(Gain, 30, true);
            // setControl(Offset, 0, true);
            self.set_resolution(0, 0, self.current_info.max_image_width, self.current_info.max_image_height);
            // setControl(TransferBits, 8, true);
            // setControl(Channels, 1, true);
            self.set_bin_mode(BinMode::Bin1x1);
            // setControl(Contrast, 0.0, true);
            // setControl(Brightness, 0.0, true);
            // setControl(Gamma, 1.0, true);

            self.default_set = true;
        }
        else
        {
            // applyParams();
        }
    }

    pub fn set_debayer(&mut self, enable: bool) -> bool {
        let res = QhyCcd::set_debayer_on_off(self.cam_handle, enable);
        if res.is_err()
        {
            eprintln!("set_debayer failure, error: {}", res.unwrap_err());
            return false
        }
        let _ = self.aloc_buffer_memory();
        self.params.apply_debayer = enable;

        true
    }

    pub fn set_bin_mode(&mut self, bin_mode: BinMode) -> bool {
        let bin_value = bin_mode.clone() as u32;
        let res = QhyCcd::set_bin_mode(self.cam_handle, bin_value, bin_value);
        if res.is_err()
        {
            eprintln!("set_bin_mode failure, error: {}", res.unwrap_err());
            return false
        }
        let _ = self.aloc_buffer_memory();
        self.params.bin_mode = bin_mode;

        true
    }

    pub fn set_resolution(&mut self, start_x: u32, start_y: u32, width: u32, height: u32) -> bool {
        let res = QhyCcd::set_resolution(self.cam_handle, start_x, start_y, width, height);
        if res.is_err()
        {
            eprintln!("set_resolution failure, error: {}", res.unwrap_err());
            return false
        }
        self.params.roi_start_x = start_x;
        self.params.roi_start_y = start_y;
        self.params.roi_width = width;
        self.params.roi_height = height;

        if self.cam_open {
            let _ = self.aloc_buffer_memory();
            self.close();
            let _ = self.open(self.cam_id.clone().as_str());
        }

        true
    }

    pub fn set_stream_mode(&mut self, mode: &sdk::StreamMode) -> bool {
        let res = QhyCcd::set_stream_mode(self.cam_handle, mode);
        if res.is_err()
        {
            eprintln!("set_bin_mode failure, error: {}", res.unwrap_err());
            return false
        }
        self.params.stream_mode = mode.clone();

        let _ = QhyCcd::init(self.cam_handle);

        true
    }

    fn aloc_buffer_memory(&mut self) -> bool {
        let has_size = QhyCcd::get_mem_length(self.cam_handle);
        if has_size.is_err()
        {
            eprintln!("Cannot get memory needed for frame.");
            return false
        }
        let new_size = has_size.unwrap() as usize;
        let mut new_buffer = Vec::with_capacity(new_size);
        unsafe {
            new_buffer.set_len(new_size);
        }
        self.img_data = new_buffer;

        return true;
    }

    pub fn set_control(&mut self, control_param: &ControlParam, value: f64, force: bool) -> bool {
        let control_id = ControlId::try_from(control_param.clone() as u32).unwrap();
        let is_available = QhyCcd::is_control_available(self.cam_handle, &control_id);
        if !is_available.is_err() && is_available.unwrap() {
            let change = self.check_force(control_param, value, force);
            if change {
                let res = QhyCcd::set_param(self.cam_handle, &control_id, value);
                if res.is_ok() {
                    self.change_internal_param(control_param, value);
                    self.apply_side_effects_of_change_param(control_param);
                }
            }
        } else if self.debug_info {
            eprintln!("Control not available to change: {}", control_param);
        }

        true
    }

    fn check_force(&mut self, control_param: &ControlParam, value: f64, force: bool) -> bool {
        if !force {
            let value_to_check = match control_param {
                ControlParam::RedWB => self.params.red_wb,
                ControlParam::GreenWB => self.params.green_wb,
                ControlParam::BlueWB => self.params.blue_wb,
                ControlParam::Brightness => self.params.brightness,
                ControlParam::Channels => self.params.channels as f64,
                ControlParam::Contrast => self.params.contrast,
                ControlParam::Exposure => self.params.exposure_time as f64,
                ControlParam::UsbTraffic => self.params.usb_traffic as f64,
                ControlParam::UsbSpeed => self.params.usb_speed as f64,
                ControlParam::Gain => self.params.gain as f64,
                ControlParam::Offset => self.params.offset as f64,
                ControlParam::TransferBits => self.params.transfer_bits as f64,
                ControlParam::Gamma => self.params.gamma,
            };
           return  value_to_check != value
        }

        true
    }

    fn change_internal_param(&mut self, control_param: &ControlParam, value: f64) {
        match control_param {
            ControlParam::RedWB => self.params.red_wb = value,
            ControlParam::GreenWB => self.params.green_wb = value,
            ControlParam::BlueWB => self.params.blue_wb = value,
            ControlParam::Brightness => self.params.brightness = value,
            ControlParam::Channels => self.params.channels = value as u32,
            ControlParam::Contrast => self.params.contrast = value,
            ControlParam::Exposure => self.params.exposure_time = value as u32,
            ControlParam::UsbTraffic => self.params.usb_traffic = value as u32,
            ControlParam::UsbSpeed => self.params.usb_speed = value as u32,
            ControlParam::Gain => self.params.gain = value as u32,
            ControlParam::Offset => self.params.offset = value as u32,
            ControlParam::TransferBits => self.params.transfer_bits = value as u32,
            ControlParam::Gamma => self.params.gamma = value,
        };
    }

    fn apply_side_effects_of_change_param(&mut self, control_param: &ControlParam) {
        match control_param {
            ControlParam::Channels => {
                self.aloc_buffer_memory();
            },
            ControlParam::TransferBits => {
                self.aloc_buffer_memory();
                self.close();
                self.open(&self.cam_id.clone());
            },
            _ => {}
        };
    }
}
