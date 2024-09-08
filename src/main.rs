use dirs::home_dir;
use image::{ImageBuffer, Rgba};
use std::io::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, ptr};
use x11::xlib::*;

fn main() -> Result<()> {
    // Connect to X11 server.
    let display: *mut _XDisplay = unsafe { XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("failed to connect to X11 server with the default display");
        return Ok(());
    };

    // Get values.
    let screen: i32 = unsafe { XDefaultScreen(display) };
    let root_window: u64 = unsafe { XRootWindow(display, screen) };
    let width: i32 = unsafe { XDisplayWidth(display, screen) };
    let height: i32 = unsafe { XDisplayHeight(display, screen) };

    // Get image form X11
    let image: *mut XImage = unsafe {
        XGetImage(
            display,
            root_window,
            0,
            0,
            width as u32,
            height as u32,
            XAllPlanes(),
            ZPixmap,
        )
    };
    if image.is_null() {
        eprintln!("failed to get image from X11 server");
        unsafe { XCloseDisplay(display) };
        return Ok(());
    }

    let img = extract_image_buffer(image);

    // Saving img.
    let home = home_dir().expect("failed to find home directory!!");
    let screenshots_dir = home.join("Pictures").join("Screenshots");
    if screenshots_dir.exists() {
        fs::create_dir_all(&screenshots_dir)?;
    }

    let timestamps = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get time stamp");
    let filename = format!("duck-shot-{}.png", timestamps.as_secs());
    let filepath = screenshots_dir.join(&filename);

    img.save(&filepath).expect("failed to save image");

    // Clearing out
    unsafe {
        XDestroyImage(image);
        XCloseDisplay(display);
    }

    Ok(())
}

fn extract_image_buffer(image: *mut XImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Getting Imgae data.
    let bytes_per_pixel: i32 = unsafe { (*image).bits_per_pixel / 8 };
    let img_data = unsafe {
        std::slice::from_raw_parts(
            (*image).data as *const u8,
            ((*image).width * (*image).height * bytes_per_pixel) as usize,
        )
    };

    // Creating image buffer.
    let bytes_per_pixel_usize: usize = bytes_per_pixel
        .try_into()
        .expect("failed to convert bytes_per_pixel to usize!!");
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(unsafe { (*image).width as u32 }, unsafe {
        (*image).height as u32
    });

    // Filling img buffer with img data.
    unsafe {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let index = (y * (*image).width as u32 + x) as usize * bytes_per_pixel_usize;
            let chunk = &img_data[index..index + bytes_per_pixel_usize];
            *pixel = Rgba([chunk[2], chunk[1], chunk[0], 255])
        }
    };

    img
}
