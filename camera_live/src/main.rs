extern crate opencv;

//use qhyccd_sdk::sdk::QhyCcd;
use qhyccd_sdk::camera::{Camera, ControlParam};

 use opencv::{
    // core,
    highgui,
//     imgproc,
     prelude::*,
//     videoio,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let has_camera = open_qhy_camera();
    if has_camera.is_none() {
        println!("No camera to work with");
        return Ok(())
    }
    let mut camera = has_camera.unwrap();

    camera.set_debug_info(true);
    camera.set_control(&ControlParam::Exposure, 200.0, false);

    let window_name = "Webcam";
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;

    loop {
        let mut frame = Mat::default();
        let _ = camera.get_frame(&mut frame, true);
    
        if frame.size()?.width > 0 {
            // // Convert frame to grayscale
            // let mut gray_frame = core::Mat::default();
            // imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

            // Show the grayscale frame in the window
            highgui::imshow(window_name, &frame)?;

            // Break the loop if 'ESC' key is pressed
            let key = highgui::wait_key(20)?;
            if key == 27 {
                break;
            }
        } else {
            println!("No frame received, please check the camera connection.");
            break;
        }
    }

    Ok(())
}

fn open_qhy_camera() -> Option<Camera> {
    let mut camera = Camera::new();
    let cameras = camera.get_cameras();
    if !cameras.is_empty() {
        for (key, value) in cameras.iter() {
            println!("Key: {}, Value: {}", key, value);
        }
    } else {
        return None
    }

    let res = camera.open("");
    if !res {
        println!("Could not open camera");
        return None
    }

    camera.set_control(&ControlParam::RedWB, 180.0, false);
    camera.set_control(&ControlParam::GreenWB, 128.0, false);
    camera.set_control(&ControlParam::BlueWB, 190.0, false);

    return Some(camera)
}

// fn test_sdk_directly() {
//     let res = QhyCcd::init_resource();
//     println!("Init resource result: {:?}", res);

//     let num_devices = QhyCcd::scan();
//     println!("Number of devices: {:?}", num_devices);

//     if num_devices > 0 {
//         let cam_id = QhyCcd::get_id(0);
//         println!("Device {:?}: {:?}", 0, cam_id);
//     }
// }