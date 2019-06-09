#[path = "model.rs"]
mod model;

extern crate futures;
extern crate futures_state_stream;
extern crate tokio;
extern crate tokio_core;
extern crate tiberius;
extern crate tokio_current_thread;
use futures::Future;
use futures_state_stream::StateStream;
use tiberius::SqlConnection;
use crate::data::model::CR;

pub fn get_crs(conn_str : &str) -> Vec<CR> {
    let mut bugs_shown : Vec<CR> = Vec::new();
    let future = SqlConnection::connect(conn_str)
        .and_then(|conn| {
            conn.simple_query("Select ID, CustomerID, Summary
                                    From ACCRequest.dbo.Bugs
                                    Where AssignedTo = 215
                                    Order By ID Desc").
                for_each(|row| {
                    let id: i32 = row.get(0);
                    let customer_id: i32 = row.get(1);
                    let ret : &str = row.get(2);
                    let summary : String = String::from(ret);
                    let cr = CR { id, customer_id, summary };
                    bugs_shown.push(cr);
                    Ok(())
                })
        });
    tokio_current_thread::block_on_all(future).unwrap();
    bugs_shown
}
