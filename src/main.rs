use std::{io, sync::Arc, sync::Mutex, thread};

use anyhow::{Context, Result};
use clap::{command, Parser};
use console::{self, Term};
use gphoto2::{widget::RadioWidget, Context as CameraContext};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about)]
/// JPEG preview stream from Canon EOS cameras.
///
/// Use +/- keys to adjust focus
struct Args {}

fn main() -> Result<()> {
    let _ = Args::parse();
    let mut out = io::stdout();

    let ctx = CameraContext::new()?;
    let camera = ctx
        .autodetect_camera()
        .wait()
        .with_context(|| "Failed to discover a camera")?;

    let focus = camera
        .config_key::<RadioWidget>("manualfocusdrive")
        .wait()?;

    let camera = Arc::new(Mutex::new(camera));

    let cam = camera.clone();
    thread::spawn(move || -> Result<()> {
        let term = Term::stderr();
        loop {
            let res = match term.read_char()? {
                '+' => {
                    focus.set_choice("Near 1")?;
                    cam.lock().unwrap().set_config(&focus).wait()
                }
                '-' => {
                    focus.set_choice("Far 1")?;
                    cam.lock().unwrap().set_config(&focus).wait()
                }
                _ => Ok(()),
            };
            if let Err(err) = res {
                eprintln!("set focus: {}", err)
            }
        }
    });

    let cam = camera.clone();
    loop {
        let frame = cam.lock().unwrap().capture_preview().wait()?;
        let data = frame.get_data(&ctx).wait()?;
        out.write_all(&data)?;
    }
}
