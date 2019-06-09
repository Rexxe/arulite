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
use std::env::args;
use std::rc::Rc;

fn main() { // Retrieve CR data
    let conn_str = "server=tcp:localhost,1433;user=aschrum;password=Password123".to_owned();
    let crs_shown : Vec<CR> = data::get_crs(&conn_str);
    let application = gtk::Application::new(
                    "CR.Listing",
                    Default::default())
        .expect("GTK Initialization failed...");
    application.connect_startup(move |app| {
        build_ui(app, &crs_shown);
    });
    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

#[repr(i32)]
enum Columns {
    Id,
    CustomerID,
    Summary,
}

fn build_ui(application: &gtk::Application, mut data: &Vec<CR>) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("CR List");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(100, 100);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    window.add(&vbox);

    let label = gtk::Label::new(
        "This is the bug list (note: not based on real data, it would be \
         nice to have a nice ODBC interface to bugzilla or so, though).",
    );
    vbox.add(&label);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let model = Rc::new(create_model(&mut data));
    let treeview = gtk::TreeView::new_with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_hexpand(true);
    treeview.set_search_column(Columns::Summary as i32);

    sw.add(&treeview);

    add_columns(&model, &treeview);

    window.show_all();

    // let model = model.clone();
    // timeout_add(80, move || spinner_timeout(&model));
}

fn add_columns(_model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {
    // Column for CR IDs
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Bug ID");
        column.add_attribute(&renderer, "text", Columns::Id as i32);
        column.set_sort_column_id(Columns::Id as i32);
        treeview.append_column(&column);
    }

    // Column for CustomerIDs
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("CustID");
        column.add_attribute(&renderer, "text", Columns::CustomerID as i32);
        column.set_sort_column_id(Columns::CustomerID as i32);
        treeview.append_column(&column);
    }

    // Column for Summary
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("CR Summary");
        column.add_attribute(&renderer, "text", Columns::Summary as i32);
        column.set_sort_column_id(Columns::Summary as i32);
        treeview.append_column(&column);
    }
}

// fn spinner_timeout(model: &gtk::ListStore) -> Continue {
//     let iter = model.get_iter_first().unwrap();
//     let pulse = model
//         .get_value(&iter, Columns::Pulse as i32)
//         .get::<u32>()
//         .unwrap()
//         .wrapping_add(1);
//
//     model.set_value(&iter, Columns::Pulse as i32 as u32, &pulse.to_value());
//     model.set_value(&iter, Columns::Active as i32 as u32, &true.to_value());
//
//     Continue(true)
// }

fn create_model(data: &Vec<CR>) -> gtk::ListStore {
    let col_types: [gtk::Type; 3] = [
        gtk::Type::U32,
        gtk::Type::U32,
        gtk::Type::String,
    ];

    let store = gtk::ListStore::new(&col_types);
    let col_indices: [u32; 3] = [0, 1, 2];
    for (_d_idx, d) in data.iter().enumerate() {
        let values: [&dyn ToValue; 3] = [
            &d.id,
            &d.customer_id,
            &d.summary,
        ];
        store.set(&store.append(), &col_indices, &values);
    }

    store
}
