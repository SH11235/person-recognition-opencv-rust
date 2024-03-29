use chrono::Local;
use dotenv::dotenv;
use opencv::{core, highgui, imgcodecs, imgproc, objdetect, prelude::*, types, videoio};
use std::env;
use std::{thread, time::Duration};

enum LoopState {
    Continue,
    Break,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    loop {
        match camera_loop()? {
            LoopState::Continue => (),
            LoopState::Break => break,
        }
        let sleep_time_seconds: u64 = env::var("SLEEP_TIME").unwrap_or("30".to_string()).parse().unwrap();
        println!("sleep {} seconds.", sleep_time_seconds);
        thread::sleep(Duration::from_secs(sleep_time_seconds));
    }
    Ok(())
}

fn camera_loop() -> Result<LoopState, Box<dyn std::error::Error>> {
    dotenv().ok();
    let haarcascades_file_type = env::var("HAARCASCADES_FILE")
        .expect("HAARCASCADES_FILE is not set in the environment variable.");

    let haarcascades_file_path = match &*haarcascades_file_type {
        "frontalface" => "haarcascades/haarcascade_frontalface_default.xml",
        "frontalface_alt" => "haarcascades/haarcascade_frontalface_alt.xml",
        "haarcascade_upperbody" => "haarcascades/haarcascade_upperbody.xml",
        "fullbody" => "haarcascades/haarcascade_fullbody.xml",
        _ => "haarcascades/haarcascade_frontalface_alt.xml",
    };

    let window = "video capture";
    let with_window = !env::var("WITH_WINDOW").is_err();

    if with_window {
        highgui::named_window(window, 1)?;
    }

    let (xml, mut cam) = {
        (
            core::find_file(haarcascades_file_path, true, false)?,
            videoio::VideoCapture::new(0, videoio::CAP_ANY)?, // 0 is the default camera
        )
    };
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut face = objdetect::CascadeClassifier::new(&xml)?;

    println!("start monitoring: {}", Local::now().format("%Y%m%d_%H%M%S"));

    'main_loop: loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width == 0 {
            thread::sleep(Duration::from_secs(50));
            continue;
        }

        let mut reduced = Mat::default();
        let mut faces = types::VectorOfRect::new();
        let mut gray = Mat::default();

        if with_window {
            // activate camera window
            highgui::imshow(window, &frame)?;
            if highgui::wait_key(10)? > 0 {
                return Ok(LoopState::Break);
            }
        }

        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        imgproc::resize(
            &gray,
            &mut reduced,
            core::Size {
                width: 0,
                height: 0,
            },
            0.25f64,
            0.25f64,
            imgproc::INTER_LINEAR,
        )?;

        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 30,
                height: 30,
            },
            core::Size {
                width: 0,
                height: 0,
            },
        )?;

        for face in faces {
            let scaled_face = core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0f64, -1f64, -1f64, -1f64),
                1,
                8,
                0,
            )?;
            // Write image using OpenCV
            let now = Local::now();
            let image_name = format!("./image/{}.jpg", now.format("%Y%m%d_%H%M%S"));
            let img_ok = imgcodecs::imwrite(&image_name, &frame, &core::Vector::default());
            match img_ok {
                Ok(_) => {
                    println!("{} saved", image_name);
                    break 'main_loop;
                },
                Err(e) => println!("{} failed: {}", image_name, e),
            }
        }
    }
    Ok(LoopState::Continue)
}
