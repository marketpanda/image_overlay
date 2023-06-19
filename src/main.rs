extern crate tinyfiledialogs;
extern crate fltk;
extern crate image as image_crate; 

use std::cell::RefCell;
use std::rc::Rc; 

use fltk::*;  
use fltk::enums::ColorDepth;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::prelude::GroupExt; 
use fltk::prelude::WindowExt;
use fltk::{button::Button, frame::Frame, enums::Color, dialog::FileChooser, dialog, image::RgbImage};
   
   
use image_crate::{DynamicImage, imageops };   
 

fn select_background(frm: &mut Frame, dynamic_background:  &Rc<RefCell<DynamicImage>>) -> image_crate::DynamicImage {
    let mut file_path = FileChooser::new(
        "./src",
        "*{.jpg, .png}",
        dialog::FileChooserType::Create,
        "Select your background"
    );

    file_path.show();
    while file_path.shown() {
        app::wait();
    }

    let selected = file_path.value(1).unwrap(); 
     
    let img = image_crate::open(&selected).expect("Could not open file!");

    let mut borrow_backgrou = dynamic_background.borrow_mut();
    *borrow_backgrou = image_crate::open(&selected).expect("Could not open background file");
     
    
    let fltk_image = RgbImage::new(
        &img.to_rgba8().into_raw(),
        img.width() as i32,
        img.height() as i32,
        ColorDepth::Rgba8,
    ).unwrap();
 
 
    frm.set_image(Some(fltk_image));

    img
}

fn  select_foreground(frm: &mut Frame, dynamic_foreground: &Rc<RefCell<DynamicImage>> ) -> image_crate::DynamicImage {
    let mut file_path = FileChooser::new(
        "./src",
        "*{.jpg, .png}",
        dialog::FileChooserType::Create,
        "Select your selfie"
    );

    file_path.show();
     
    while file_path.shown() {
        app::wait();
    }
    println!("{}", file_path.value(1).unwrap());

    let selected_path = file_path.value(1).unwrap(); 
    let mut borrow_dynfore = dynamic_foreground.borrow_mut();
     
 
    let img = image_crate::open(&selected_path).expect("Could not open file!");  

    *borrow_dynfore = image_crate::open(&selected_path).expect("Could not open foreground file");
    
      
    let fltk_image = RgbImage::new(
        &img.to_rgba8().into_raw(),
        img.width() as i32,
        img.height() as i32,
        ColorDepth::Rgba8
    ).unwrap();
 
    frm.set_image(Some(fltk_image));
    println!("{:?}", img);

    img
     
}


 

fn overlay_images_buffer(base: &DynamicImage, foreground: &DynamicImage) -> DynamicImage {
   
    
    let mut img1_c = base.clone();
    let img2_c = foreground.clone();
    image_crate::imageops::overlay(&mut img1_c, &img2_c, 0, 0);
    img1_c
}

fn save_overlay_image(back_img: &Rc<RefCell<DynamicImage>>, fore_img:  &Rc<RefCell<DynamicImage>>, ) {

      
    let fore_ref = fore_img.borrow_mut();
    let back_ref = back_img.borrow_mut();
    let get_img = overlay_images_buffer(&back_ref,  &fore_ref);
 
    //get_img.save("overlayed.jpg");

    let mut dialog2 = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);

    dialog2.show();
 
    let file_name = Some(dialog2.filename());

    if let Some(file_name) = file_name {
        println!("File name {:?}", file_name);
        get_img.save(&file_name).unwrap();
    } else {
        println!("Nothing here");
    }   
}

fn main()   { 

    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut wind = window::Window::default().with_size(900,500).center_screen().with_label("Video Call Background Creator");


    let mut frm = Frame::new(0, 0, 300, 300, ""); 
    let mut frm2 = Frame::new(300, 0, 300, 300, ""); 
    let mut button_background = Button::new(200, 420, 120, 40, "Select Background");    
    let mut button_foreground = Button::new(500, 420, 120, 40, "Select Foreground"); 

    let mut button_overlay = Button::new(350, 420, 120, 40, "Overlay Images");
    
    frm.set_color(Color::White);
    frm.set_frame(enums::FrameType::BorderBox); 
    frm2.set_color(Color::White);
    frm2.set_frame(enums::FrameType::BorderBox); 

    let dynamic_image_fore:DynamicImage = DynamicImage::new_rgba8(300, 300); 
    let dynamic_image_back:DynamicImage = DynamicImage::new_rgba8(900, 900);
  
    let dyn_fore_rc = Rc::new(RefCell::new(dynamic_image_fore));
    
    let dyn_back_rc = Rc::new(RefCell::new(dynamic_image_back));
    let callback_background = Rc::clone(&dyn_back_rc);
    let callback_foreground = Rc::clone(&dyn_fore_rc);
    let callback_overlay_f = Rc::clone(&dyn_fore_rc);
    let callback_overlay_b = Rc::clone(&dyn_back_rc);

    button_background.set_callback(move |_| {
        select_background(&mut frm, &callback_background );
    });
    button_foreground.set_callback(move |_| { 
         
        select_foreground(&mut frm2, &callback_foreground); 
    });
 


    button_overlay.set_callback(move |_| { 
        save_overlay_image(&callback_overlay_b, &callback_overlay_f);
    });
    
    
  
    wind.end();
    wind.make_resizable(true);  
    wind.show();    
     
    wind.set_color(Color::DarkCyan);  
 
    app.run().unwrap(); 
         
    
}