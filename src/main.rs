

// Declare moduals
mod window;
mod vulcan; 


use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";


fn main() -> glib::ExitCode {
    
    println!("Hello, world!");

    // vulcan::call_inner_tests();
    let app = Application::builder().application_id(APP_ID).build();


    // run app
    app.run()

}
