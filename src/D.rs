use cursive;
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
    LinearLayout, SelectView};
use cursive::traits::*;
//use std::{thread, time};
#[macro_use]
//extern crate dotenv_codegen;
//extern crate dotenv;
use dotenv::dotenv;
//use std::env;
use serde_json::Value;
use attohttpc;
use rustDatabook;
use crate::X;
use crate::refresh;
pub fn dataAdder(s: &mut Cursive){
    
    first(s);
    fn first(s: &mut Cursive){
        s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("1")

            .fixed_width(10))
            
        .title("Enter name")
        .button("Ok", move |s| {
            let mut name = "".to_string();
                s.call_on_name("1", |view: &mut EditView| {
                    name = view.get_content().clone().to_string();

                }).unwrap();
             sec(s,name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    }
    fn sec(s: &mut Cursive, o: String){
        s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("2")

            .fixed_width(10))
            
        .title("Enter stock name")
        .button("Ok", move |s| {
            let mut name = "".to_string();
                s.call_on_name("2", |view: &mut EditView| {
                    name = view.get_content().clone().to_string();

                }).unwrap();
                trs(s,o.clone(),name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    }
    fn trs(s: &mut Cursive, o: String,t: String){
        s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("3")

            .fixed_width(10))
            
        .title("Enter amount")
        .button("Ok", move |s| {
            let mut name = "".to_string();
                s.call_on_name("3", |view: &mut EditView| {
                    name = view.get_content().clone().to_string();

                }).unwrap();
             frt(s,o.clone(),t.clone(),name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    }
    fn frt(s: &mut Cursive, o: String,t: String,tt: String){
        s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("4")

            .fixed_width(10))
            
        .title("Enter stock price")
        .button("Ok", move |s| {
            let mut name = "".to_string();
                s.call_on_name("4", |view: &mut EditView| {
                    name = view.get_content().clone().to_string();

                }).unwrap();
            let ad = format!("|{}_A|{},{},{},{}",o,o,t,tt,name);
            X::addData(&ad);
            s.pop_layer();
            s.pop_layer();
            s.pop_layer();
            s.pop_layer();
            refresh(s);

        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    }
    
}
