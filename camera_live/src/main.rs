extern crate opencv;

use qhyccd_sdk::qhyccd::QHYCCD;

use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    videoio,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = QHYCCD::init_resource();
    println!("Init resource result: {:?}", res);

    let num_devices = QHYCCD::scan();
    println!("Number of devices: {:?}", num_devices);

    if num_devices > 0 {
        let cam_id = QHYCCD::get_id(0);
        println!("Device {:?}: {:?}", 0, cam_id);
    }


    // // Open the default camera (camera index 0)
    // let mut capture = videoio::VideoCapture::from_file("Dahua-20220901-184734.mp4", videoio::CAP_ANY)?; 

    // if !videoio::VideoCapture::is_opened(&capture)? {
    //     panic!("Unable to open the camera");
    // }

    // let window_name = "Webcam";
    // highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;

    // loop {
    //     let mut frame = core::Mat::default();
    //     capture.read(&mut frame)?;

    //     if frame.size()?.width > 0 {
    //         // // Convert frame to grayscale
    //         // let mut gray_frame = core::Mat::default();
    //         // imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

    //         // Show the grayscale frame in the window
    //         highgui::imshow(window_name, &frame)?;

    //         // Break the loop if 'ESC' key is pressed
    //         let key = highgui::wait_key(20)?;
    //         if key == 27 {
    //             break;
    //         }
    //     } else {
    //         println!("No frame received, please check the camera connection.");
    //         break;
    //     }
    // }

    Ok(())
}