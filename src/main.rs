#![windows_subsystem = "windows"]

/* Preincluded cosas */

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::needless_update)]

mod encoder;

use std::env;
use std::thread;
use std::path::Path;
use std::ffi::OsStr;
use encoder::Encoder;
use bindings::Windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, MessageBoxA, MB_OK, MB_ICONERROR,
                                                        SM_CXFULLSCREEN, SM_CYFULLSCREEN};
use std::process::exit;
use fltk::app::*;
use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;
use std::net::Incoming;
use wfd::{DialogParams, DialogError, OpenDialogResult};
use std::borrow::Borrow;
use image::GenericImageView;


fn Comienzo(width: i32, height: i32) -> (i32,i32) {
    unsafe {
        let x0 = GetSystemMetrics(SM_CXFULLSCREEN);
        let y0 = GetSystemMetrics(SM_CYFULLSCREEN);
        return ((x0-width)/2, (y0-height)/2);
    }
}

fn ObtenerNuevo(fullp: String) -> String {
    let mut p0 = Path::new(&fullp).file_stem().and_then(OsStr::to_str).unwrap().to_owned();
    let borrowed_string: &str = "_wave.wav";
    p0.push_str(borrowed_string);
    return p0;
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gleam);
    let mut wind = Window::new(Comienzo(700,320).0, Comienzo(700,320).1, 700, 320, "Robot 36 encoder");
    let mut editar = Input::new(80, 55, 465, 75, None);
    let mut escogar = Button::new(585, 70, 55, 55, "...");
    let mut prg = Progress::new(45, 225, 545, 45, None);
    prg.set_selection_color(Color::by_index(221));
    prg.set_minimum(0.0);
    prg.set_maximum(100.0);
    let mut gp1 = Group::new(35, 25, 620, 150, "Escoge un archivo");
    gp1.end();
    let mut gp2 = Group::new(25, 190, 630, 120, "Progreso");
    gp2.end();
    /////////// Handle Messages //////////////
    escogar.handle(move |widget, ev: Event| {
        match ev  {
            Event::Push => {
                let params = DialogParams {
                    title: "Select an image file to open",
                    file_types: vec![("JPEG Files", "*.jpg"), ("PNG Files", "*.png"), ("GIF Files", "*.gif")],
                    default_extension: "jpg",
                    ..Default::default()
                };
                match wfd::open_dialog(params) {
                    Ok(r) => unsafe {
                        editar.set_value(r.selected_file_path.into_os_string().to_str().unwrap());
                        let image = image::open(editar.value()).unwrap();

                        Box::leak(editar.value().into_boxed_str());

                        if(image.width() > 770 || image.height() > 770) {
                            unsafe {
                                MessageBoxA(None, "Resolution of picture is too large!", "Error", MB_ICONERROR | MB_OK);
                            }
                        } else {
                            let mut encoder = Encoder::new(image, 48000);
                            prg.set_value(15.0);
                            let samples = encoder.encode().unwrap();
                            prg.set_value(25.0);
                            let siz = samples.len() as f64;
                            let per: f64 = 70.0 / siz;
                            let mut cnt = 1.0;
                            let spec = hound::WavSpec {
                                channels: 1,
                                sample_rate: 48000,
                                bits_per_sample: 16,
                                sample_format: hound::SampleFormat::Int,
                            };
                            let mut writer = hound::WavWriter::create(ObtenerNuevo(editar.value()), spec).unwrap();
                            prg.set_value(30.0);
                            for sample in samples.iter() {
                                writer.write_sample(*sample).unwrap();
                                prg.set_value(30.0 + cnt*per);
                                cnt+=1.0;
                            }
                        }
                    },
                    _ => {}
                }
                true
            },
            _ => false,
        }
    });

    //////////////////////////////////////////
    wind.end();
    wind.show();
    app.run().unwrap();
}