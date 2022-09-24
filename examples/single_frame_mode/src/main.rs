// QHYCCD SingleFrameMode Example
// Optimized for QHY600

//Standard Lib
use std::os::raw::c_char;
use std::process::exit;
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};

//QHY Lib
use qrusthy::sdk::qhyccd::*;

//Fitsio Lib
use fitsio::FitsFile;
use fitsio::images::{ImageDescription, ImageType};
use std::fs::remove_file;
use std::path::Path;

fn main() {

    let cam_handle: *mut qhyccd_handle = quick_start(); // Initialize SDK and open camera

    // Some parameters
    let read_mode: u32 = 1;
    let stream_mode: u32 = 0;
    let usb_traffic: u32 = 10;
    let roi_start_x: u32 = 0;
    let roi_start_y: u32 = 0;
    let roi_size_x: u32 = 9600;
    let roi_size_y: u32 = 6422;
    let cam_bin_x: u32 = 1;
    let cam_bin_y: u32 = 1;
    let bit_resolution: u32 = 16;

    quick_first_parameters(cam_handle, read_mode, stream_mode, usb_traffic, 
        roi_start_x, roi_start_y, roi_size_x, roi_size_y, cam_bin_x, 
        cam_bin_y, bit_resolution); // Set first few settings

    // Gain, Offset, Exposure Time, Save Path, and Temperature
    let gain_setting: u32 = 56;
    let offset_setting: u32 = 20;
    let exposure_time: f64 = 5.0; // In seconds

    let save_path: String = "/home/qhyImg_".to_string(); // Edit this        

    let temp_setting: f64 = 20.0; // In Celsius
    let temp_error: f64 = 10.0;

    quick_main_settings(cam_handle, gain_setting, offset_setting, exposure_time); // Set gain, offset, and exposure time

    quick_filter_wheel_control(cam_handle, 4);

    quick_temp_control(cam_handle, temp_setting, temp_error); // Set temperature

    quick_image_capture(cam_handle, roi_size_x, roi_size_y, 
        bit_resolution, gain_setting, offset_setting, 
        exposure_time, temp_setting, save_path); // Take image and save to file

    quick_exit(cam_handle); // Close camera handle and release SDK resources

}


// Initialize SDK and open camera to get camera handle
pub fn quick_start() -> *mut qhyccd_handle {

    // Initialize SDK
    let mut retval = unsafe {InitQHYCCDResource()};
    println!(" ");
    if retval == QHYCCD_SUCCESS {
        println!("SDK resources initialized.");
        println!(" ");
    } else {
        println!("Cannot initialize SDK resources, received error: {}. Quitting Program.", retval);
        exit(1);
    }

    //Scan For Cameras
    let cam_count: u32 = unsafe {ScanQHYCCD()};
    if cam_count > 0 {
        println!("Number of QHYCCD cameras found: {}.", cam_count);
        println!(" ");
    } else {
        println!("No QHYCCD camera found, please check USB or power.");
        exit(1);
    }

    // Open Camera 1 -- Change number in GetQHYCCDId to change camera
    let mut qhy_cam_id = "QHY600M-3edd8843b44b4def2".to_string(); // Camera Serial Number -- will change for each camera

    let id: *mut c_char = qhy_cam_id.as_mut_ptr() as *mut c_char; // Set camera ID

    retval = unsafe{GetQHYCCDId(0, id)}; // Confirm we can connect to the camera
    if retval == QHYCCD_SUCCESS {
        println!("Connected to QHYCCD Camera successfully with Camera ID: {}.", qhy_cam_id);
    } else {
        println!("The detected camera is not able to be connected. Please ensure camera is from QHYCCD. Error: {}.", retval);
        exit(1);
    }

    // Get Camera Handle
    let cam_handle: *mut qhyccd_handle = unsafe {OpenQHYCCD(id)}; // Get Camera Handle

    if !cam_handle.is_null() {
        println!("Camera opened successfully!");
        println!(" ");
    } else {
        println!("Camera could not be opened. Program exiting. Error: {}.", retval);
        exit(1);
    }

    // Return Camera Handle
    return cam_handle;
}

// Set Readmode
pub fn set_readmode(
    cam_handle: *mut qhyccd_handle, 
    read_mode: u32) {

    let retval = unsafe {SetQHYCCDReadMode(cam_handle, read_mode)}; // Set readmode

    if retval == QHYCCD_SUCCESS {
        println!("Camera readmode set to {}.", read_mode);
    } else {
        println!("Camera readmode could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set Steam Mode
pub fn set_streammode(
    cam_handle: *mut qhyccd_handle, 
    stream_mode: u32) {

    let stream_mode = stream_mode as u8;

    let retval = unsafe {SetQHYCCDStreamMode(cam_handle, stream_mode)}; // Set stream mode

    if retval == QHYCCD_SUCCESS {
        println!("Camera stream mode set to Single Frame Mode.");
        println!(" ");
    } else {
        println!("Camera stream mode could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Initialize Camera (must be done after setting readmode and stream mode)
pub fn initialize_camera(
    cam_handle: *mut qhyccd_handle) {

    let retval = unsafe {InitQHYCCD(cam_handle)}; // Initialize Camera

    if retval == QHYCCD_SUCCESS {
        println!("QHY Camera successfully initialized.");
        println!(" ");
    } else {
        println!("Camera could not be initialized. Error: {}.", retval);
        exit(1);
    }
}

// Set USB Traffic Setting
pub fn set_usbtraffic(
    cam_handle: *mut qhyccd_handle, 
    usb_traffic: u32) {

    let usb_traffic = usb_traffic as f64;

    let retval = unsafe {SetQHYCCDParam(cam_handle,CONTROL_ID_CONTROL_USBTRAFFIC,usb_traffic)}; // Set USB Traffic

    if retval == QHYCCD_SUCCESS {
        println!("USB traffic setting set to {}.", usb_traffic);
    } else {
        println!("USB traffic setting could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set Image Resolution
pub fn set_resolution(
    cam_handle: *mut qhyccd_handle, 
    roi_start_x: u32, 
    roi_start_y: u32, 
    roi_size_x: u32,
    roi_size_y: u32) {

    let retval = unsafe {SetQHYCCDResolution(cam_handle,roi_start_x, roi_start_y, roi_size_x, roi_size_y)}; // Set Image Resolution

    if retval == QHYCCD_SUCCESS {
        println!("Image resolution set to {}x{}.", roi_size_x, roi_size_y);
    } else {
        println!("Image resolution could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set Binning Mode
pub fn set_binningmode(
    cam_handle: *mut qhyccd_handle, 
    cam_bin_x: u32, 
    cam_bin_y: u32) {

    let retval = unsafe {SetQHYCCDBinMode(cam_handle, cam_bin_x, cam_bin_y)}; // Set Cam Binning Mode

    if retval == QHYCCD_SUCCESS {
        println!("Binning mode set to {}x{}.", cam_bin_x, cam_bin_y);
    } else {
        println!("Binning mode could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set Bit Resolution
pub fn set_bitresolution(
    cam_handle: *mut qhyccd_handle, 
    bits: u32) {

    let retval = unsafe {SetQHYCCDBitsMode(cam_handle, bits)}; // Set Bit Resolution

    if retval == QHYCCD_SUCCESS {
        println!("Camera bit resolution set to {}.", bits);
    } else {
        println!("Camera bit resolution could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set first few settings, such as readmode, stream mode, initialize camera, usb traffic, 
// image resolution, binning mode, and bit resolution
pub fn quick_first_parameters(
    cam_handle: *mut qhyccd_handle,
    read_mode: u32,
    stream_mode: u32,
    usb_traffic: u32, 
    roi_start_x: u32,
    roi_start_y: u32,
    roi_size_x: u32,
    roi_size_y: u32,
    cam_bin_x: u32,
    cam_bin_y: u32,
    bits: u32) {
        
        set_readmode(cam_handle, read_mode); // Set Readmode (do this before initializing camera)
        set_streammode(cam_handle, stream_mode); // Set Stream Mode (do this before initializing camera)
        
        initialize_camera(cam_handle); // Initialize Camera

        set_usbtraffic(cam_handle, usb_traffic); // Set USB Traffic
        set_resolution(cam_handle, roi_start_x, roi_start_y, roi_size_x, roi_size_y); // Set image resolution
        set_binningmode(cam_handle, cam_bin_x, cam_bin_y); // Set binning mode
        set_bitresolution(cam_handle, bits); // Set bit resolution

}

// Set Gain Setting
pub fn set_gain(
    cam_handle: *mut qhyccd_handle, 
    gain_setting: u32) {

    let gain_setting = gain_setting as f64;

    let retval = unsafe {SetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_GAIN, gain_setting)}; // Set gain setting

    if retval == QHYCCD_SUCCESS {
        println!("Gain set to {}.", gain_setting);
    } else {
        println!("Gain setting could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set Offset Setting
pub fn set_offset(
    cam_handle: *mut qhyccd_handle, 
    offset_setting: u32) {

    let offset_setting = offset_setting as f64;

    let retval = unsafe {SetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_OFFSET, offset_setting)}; // Set offset setting

    if retval == QHYCCD_SUCCESS {
        println!("Offset set to {}.", offset_setting);
    } else {
        println!("Offset setting could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set Exposure Time
pub fn set_exposure(
    cam_handle: *mut qhyccd_handle, 
    exposure_time: f64) {

    let exp_time_in_micros = exposure_time * 1000000.0; // Convert to microseconds

    let retval = unsafe {SetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_EXPOSURE, exp_time_in_micros)}; // Set exposure time

    if retval == QHYCCD_SUCCESS {
        println!("Exposure time set to {} seconds.", exposure_time);
        println!(" ");
    } else {
        println!("Exposure time could not be set. Error: {}.", retval);
        exit(1);
    }
}

// Set gain and offset settings and exposure time
pub fn quick_main_settings(
    cam_handle: *mut qhyccd_handle,
    gain_setting: u32,
    offset_setting: u32,
    exposure_time: f64) {

        set_gain(cam_handle, gain_setting); // Set gain setting
        set_offset(cam_handle, offset_setting); // Set offset setting
        set_exposure(cam_handle, exposure_time); // Set exposure time
}

// Control filter wheel
pub fn quick_filter_wheel_control(
    cam_handle: *mut qhyccd_handle,
    fw_pos: u32) {

        let mut retval = unsafe {IsQHYCCDCFWPlugged(cam_handle)};

        if retval == QHYCCD_SUCCESS {

            let fw_position: String = fw_pos.to_string(); // Convert wanted FW Position to String
            let fw_position_char = fw_position.as_ptr() as *mut c_char; // Create pointer

            let status: String = "0".to_string(); // Status string
            let status_char = status.as_ptr() as *mut c_char; // Create pointer

            retval = unsafe {GetQHYCCDCFWStatus(cam_handle, status_char)}; // Check where is filter wheel
            if retval == QHYCCD_SUCCESS {
                println!("Filter wheel is plugged in and is at position: {}.", status);
                println!(" ");
            } else {
                println!("Error getting filter wheel status. Error: {}.", retval);
                exit(1);
            }
            
            // If filter wheel is not already where we want it to be
            if status != fw_position {

                // Send order to filter wheel
                retval = unsafe {SendOrder2QHYCCDCFW(cam_handle, fw_position_char, 1)};
                if retval == QHYCCD_SUCCESS {
                    println!("Filter wheel is moving to position {}.", fw_position);
                } else {
                    println!("Sending order to filter wheel failed. Error: {}.", retval);
                }

                // Check whether filter wheel is moving or not
                retval = unsafe{ GetQHYCCDCFWStatus(cam_handle, status_char)}; 
                if retval != QHYCCD_SUCCESS {
                    println!("Error getting filter wheel status. Error: {}.", retval);
                    exit(1);
                }

                // If filter wheel needs to go to position 0 (slot 1)
                if status == fw_position {

                    let mut sleeper = 0; // To print progress on screen

                    while sleeper < 11 {
                        thread::sleep(time::Duration::from_millis(1000)); // Sleep for 1 second
                        println!("Filter wheel is still moving");
                        sleeper = sleeper + 1;
                    }

                } else {

                    // While camera is moving
                    while status != fw_position {

                        thread::sleep(time::Duration::from_millis(1000)); // Sleep for 1 second
                        retval = unsafe{ GetQHYCCDCFWStatus(cam_handle, status_char)}; 
                        if retval == QHYCCD_SUCCESS {
                            println!("Filter wheel is still moving.");
                        } else {
                            println!("Error getting filter wheel status. Error: {}.", retval);
                            exit(1);
                        }

                    }
                }

                // Final result
                println!(" ");
                println!("Filter wheel has been moved to position {}.", fw_pos);
            }

        } else { // If filter wheel is not detected

            println!("No filter wheel detected.");
            println!(" ");
        }   
}

// Camera Temperature Regulation
pub fn quick_temp_control(
    cam_handle: *mut qhyccd_handle,
    temp_setting: f64,
    temp_error: f64) {

        // Get current temperature
        let mut current_temp = unsafe {GetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_CURTEMP)};

        // Set temperature to temp_setting
        let retval = unsafe {SetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_COOLER, temp_setting)};

        // Get temperature difference
        let mut temp_difference = (current_temp - temp_setting).abs();

        // Temperature Control Loop

        // If temperature difference is not within the error range
        if temp_difference > temp_error {

            // Set temperature loop value
            let mut temp_loop = 0; 

            // While loop to check for temperature 3 times to avoid over/under-shooting
            while temp_loop < 3 {

                // Sleep for 1 second to get new temperature
                thread::sleep(time::Duration::from_millis(1000));

                // Get current temperature again to loop
                current_temp = unsafe {GetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_CURTEMP)};

                // Get temperature difference again
                temp_difference = (current_temp - temp_setting).abs();

                // While the temperature is outside of the error range
                while temp_difference > temp_error {

                    // Get cooler PWM
                    let pwm_value = unsafe {GetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_CURPWM)};

                    // Report temperature progress and cooler PWM to screen
                    if (current_temp - temp_setting) > temp_error {
                        println!("Current Temperature: {:.1} || You Want: {:.1}. Camera is cooling down.", current_temp, temp_setting);
                        println!("Cooler PWM is {}, running at {:.2}% of full power.", pwm_value, pwm_value / 255.0 * 100.0);
                        println!(" ");
                    } else {
                        println!("Current Temperature: {:.1} || You Want: {:.1}. Camera is heating up.", current_temp, temp_setting);
                        println!("Cooler PWM is {}, running at {:.2}% of full power.", pwm_value, pwm_value / 255.0 * 100.0);
                        println!(" ");
                    }

                    // Try again in 2 seconds
                    thread::sleep(time::Duration::from_millis(2000));

                    // Get current temperature again to loop
                    current_temp = unsafe {GetQHYCCDParam(cam_handle, CONTROL_ID_CONTROL_CURTEMP)};

                    // Get temperature difference again
                    temp_difference = (current_temp - temp_setting).abs();

                }

                // Sleep for 1 second again before running
                thread::sleep(time::Duration::from_millis(1000));

                // Increment temp_loop to run again
                temp_loop = temp_loop + 1; 

            }

        }

        if retval == QHYCCD_SUCCESS {
            println!("Camera temperature set to {} C.", temp_setting);
            println!(" ");
        } else {
            println!("Camera temperature could not be set. Error: {}.", retval);
            exit(1);
        }
}

// Take an image and save to file
pub fn quick_image_capture(
    cam_handle: *mut qhyccd_handle,
    mut roi_size_x: u32,
    mut roi_size_y: u32,
    mut bits: u32,
    gain_setting: u32,
    offset_setting: u32,
    exposure_time: f64,
    temp_setting: f64,
    save_path: String,) {

        println!("Camera will now take the image.");
        println!(" ");

        //Convert roi_size_x and roi_size_y to usize
        let roi_x: usize = roi_size_x as usize;
        let roi_y: usize = roi_size_y as usize;

        // Channel of image
        let mut channels: u32 = 16;

        // Start single frame exposure
        let mut retval = unsafe {ExpQHYCCDSingleFrame(cam_handle)};

        if retval == QHYCCD_SUCCESS {
            println!("Camera exposure started.");
        } else {
            println!("Camera exposure failed. Error: {}.", retval);
            exit(1);
        }

        // Sleep for 1 second to get new temperature
        thread::sleep(time::Duration::from_millis(1000));

        // Create image buffer
        let mut image_buffer: Vec<u16> = vec![0u16; roi_x*roi_y as usize];

        // Create mutable pointer to image buffer
        let image_buffer_ptr: *mut u16 = image_buffer.as_mut_ptr();

        // Take single frame
        retval = unsafe {GetQHYCCDSingleFrame(cam_handle, &mut roi_size_x, &mut roi_size_y, &mut bits, &mut channels, image_buffer_ptr)};

        if retval == QHYCCD_SUCCESS {
            println!("Successfully got image of size: {}x{}.", roi_size_x, roi_size_y);
            println!(" ");
        } else {
            println!("Could not get the image. Error: {}.", retval);
            exit(1);
        }

        // Image Processing to .fits file
        quick_save_to_fits(image_buffer, save_path, roi_x, roi_y, gain_setting, offset_setting, exposure_time, temp_setting);

        // Cancel exposure and readout
        let retval = unsafe {CancelQHYCCDExposingAndReadout(cam_handle)};

        if retval == QHYCCD_SUCCESS {
            println!("Image taken and exposure successfully stopped.");
            println!(" ");
        } else {
            println!("Could not cancel readout and exposure. Error: {}.", retval);
            exit(1);
        }

}

// Function to save image to file as a fits file
pub fn quick_save_to_fits(
    image_buffer: Vec<u16>,
    save_path: String,
    roi_x: usize,
    roi_y: usize,
    gain_setting: u32,
    offset_setting: u32,
    exposure_time: f64,
    temp_setting: f64) {

        // Dimensions of image
        let shape = vec![roi_y,roi_x];

        // Set image data type and dimensions
        let img_desc = ImageDescription {
            data_type: ImageType::UnsignedShort,
            dimensions: shape.as_slice(),
        };

        //Get UNIX time in seconds
        let unix_time = &*SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards, please check your spatial and temporal dimensions.")
            .as_secs()
            .to_string();

        // Set name of the fits file
        let full_path = save_path + unix_time + "_exp_" + &*exposure_time.to_string() + "s_temp_" + &*temp_setting.to_string() + "_offset_" + &*offset_setting.to_string() + "_gain_" + &*gain_setting.to_string()+ ".fits";

        // Remove file if it already exists
        if Path::new(&full_path).is_file() {
            remove_file(&full_path).expect("Could not remove pre-existing file on disk.");
        }

        // Create new fits file with the specific description and dimensions and open file
        let mut fits_file = FitsFile::create(full_path)
            .with_custom_primary(&img_desc)
            .open().unwrap();

        // Open fits file to write into it
        let hdu = fits_file.primary_hdu().unwrap();

        // Write to fits file from image buffer
        hdu.write_image(&mut fits_file, &image_buffer).expect("Could not write to fits file");

        // Report progress to file
        println!("Image with temperature {} C, exp {} sec, offset {}, gain {}, saved successfully to disc.", temp_setting, exposure_time, offset_setting, gain_setting);
        println!(" ");

}

// Close camera handle and release SDK resources
pub fn quick_exit(cam_handle: *mut qhyccd_handle) {

    // Close camera handle
    let mut retval = unsafe {CloseQHYCCD(cam_handle)};

    if retval == QHYCCD_SUCCESS {
        println!("Camera handle closed successfully.");
    } else {
        println!("Camera handle could not be closed. Error: {}.", retval);
    }

    // Release SDK Resource
    retval = unsafe {ReleaseQHYCCDResource()};

    if retval == QHYCCD_SUCCESS {
        println!("SDK resources released successfully.");
    } else {
        println!("SDK resources not released successfully. Error: {}.", retval);
    }

}
