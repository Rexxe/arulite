extern crate gio;
extern crate gtk;
extern crate futures;
extern crate futures_state_stream;
extern crate tokio;
extern crate tokio_core;
extern crate tiberius;
mod data;
use crate::data::model::CR;
use gio::prelude::*;
use gtk::prelude::*;

fn main() {
    // let mut core = Core::new().unwrap();
    let conn_str = "server=tcp:localhost,1433;user=aschrum;password=Password123".to_owned();
    let bugs_shown : Vec<CR> = data::get_crs(&conn_str);
    for cr in bugs_shown {
        println!("{}", cr);
    }
}
