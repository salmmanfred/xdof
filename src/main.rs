use cursive;
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
    LinearLayout, SelectView};
use cursive::traits::*;
//use std::{thread, time};
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
use dotenv::dotenv;
//use std::env;
use serde_json::Value;
use attohttpc;
use rustDatabook;
use std::path::Path;
use std::fs;
mod X;
mod D;
mod O;
mod F;


fn main() {
    //print!("{}",O::getStockChart("TSLA"));
    let mut siv = cursive::default();

    siv.add_layer(Dialog::text("Welcome")
        .title("XDOF")
        .button("Start", sign));

    siv.run();
}
fn sign(s: &mut Cursive){
    /*s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("name")

            .fixed_width(10))
            
        .title("Enter password")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    //name = view.get_content().clone().to_string();

                }).unwrap();
                //login(s,"t");
                sl(s);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));*/
    s.add_layer(Dialog::text("Starting")
        .title("Loading")
        );
   
    if !Path::new(&format!("{}{}",F::DIREC,F::FILER)).exists(){
        fs::create_dir_all(F::DIREC).expect("er");
        openfile::writeFile(&format!("{}{}",F::DIREC,F::FILER), "");
    }
    let x = X::getdata();
    let mut letthrou = true;
    if F::VALDI{
        for f in x.A{
            println!("{}",f.len());
            if f.len() != 4{
                letthrou = false;
                didl(s,&format!("{}",f.len()));
            }
        }
    }
    if letthrou{
        s.pop_layer();
        sl(s);
    }else{
        didl(s, ".rdb ERROR");
    }
    
    
    
}
pub fn refresh(s: &mut Cursive){
    s.pop_layer();
    selit(s, "Stocks");
}


fn sl(s: &mut Cursive){
        s.pop_layer();
        

        fn ok(s: &mut Cursive, name: &str) {
            s.call_on_name("select", |view: &mut SelectView<String>| {
                view.add_item_str(name)
            });
            s.pop_layer();
        }
        println!("test");
        let mut select = SelectView::<String>::new()
            .on_submit(selit)
            .with_name("select")
            .fixed_size((12, 5));
            
        select.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str("Stocks");
            if F::STOCK{
                view.add_item_str("Stock price");
            }
            if F::GRAPH{
                view.add_item_str("Graph");
            }


            //view.add_item_str("Settings");
                
            });
        //let mut name: String;
        let buttons = LinearLayout::vertical()
     
            .child(DummyView)
            .child(Button::new("Quit", Cursive::quit));
    
        s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title(format!("Main Menue")));
    
        
}
fn selit(s: &mut Cursive,item: &str){
    if item == "Stock price"{
        s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("STOCK")

            .fixed_width(10))
            
        .title("Enter stock name")
        .button("Ok", |s| {
            let mut name = "".to_string();
                s.call_on_name("STOCK", |view: &mut EditView| {
                    name = view.get_content().clone().to_string();

                }).unwrap();
                let pr = getStock(&name.clone());
                didl(s, &pr.to_string());
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    }
    if item == "Stocks"{
        s.pop_layer();

     
        let book = X::getdata();

        fn ok(s: &mut Cursive, name: &str) {
            s.call_on_name("select", |view: &mut SelectView<String>| {
                view.add_item_str(name)
            });
            s.pop_layer();
        }
        println!("test");
        let mut select = SelectView::<String>::new()
            .on_submit(loaddata)
            .with_name("select")
            .fixed_size((10, book.A.len()));
            
        select.call_on_name("select", |view: &mut SelectView<String>| {
            let len = book.AN.len();
           
            for x in 1..len{
                view.add_item_str(book.AN[x].clone());
            }

            });
        //let mut name: String;
        let buttons = LinearLayout::vertical()
            .child(Button::new("Main Menue", |s|{sl(s)}))
            .child(Button::new("Add new", |s|{
                D::dataAdder(s);
            }))

            .child(DummyView)
            .child(Button::new("Quit", Cursive::quit));
    
        s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title(format!("Main Menue")));
    }
    if item == "Graph"{
        s.add_layer(Dialog::around(EditView::new()
            //.on_submit(ok)
            .with_name("D")

            .fixed_width(10))
            
        .title("Enter stock name")
        .button("Ok", |s| {
            let mut name = "".to_string();
                s.call_on_name("D", |view: &mut EditView| {
                    name = view.get_content().clone().to_string();

                }).unwrap();
                let pr = O::getStockChart(&name.clone());
                openfile::writeFile(&format!("{}{}",F::DIREC,F::COOL2), &pr);
                
                s.add_layer(Dialog::text(&pr)
                    .title("Graph")
                    .button("OK", |s| {
                        s.pop_layer();
                }));
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
    }
    

}





fn loaddata(s: &mut Cursive,item: &str){
    //didl(s,&format!("{}",item));
    let o = X::getCont(item);
    //let o = ["x","x","x","x"];
    let fitem = item.to_owned().clone();
    let fitem2 = item.to_owned().clone();

    fn del(item: String){
        rustDatabook::removeData(F::FILER, &item);

    }
    fn gs(s: &mut Cursive,f: String){
        let o = X::getCont(&f);

        let pr = getStock(&o[1]);
        didl(s,&format!("Price: {}",pr));
    }

    s.add_layer(Dialog::text(format!("Name: {}\nStock: {}\nAmount: {}\nBuy price: {}\n",o[0],o[1],o[2],o[3]))
    .title("Stock")
    .button("OK", |s| {
        s.pop_layer();
    })
    .button("Check Price", move |s| {
        if F::STOCK{
            gs(s,fitem2.clone());
        }else{
            didl(s,"Error Feature disabled ");
        }


    })
    .button("DELETE", move |s| {
        X::deleteData(fitem.clone());
        s.pop_layer();
        refresh(s);
    })
);
}
fn didl(s: &mut Cursive,text: &str){
    s.add_layer(Dialog::text(text)
        .title("Note")
        .button("OK", |s| {
            s.pop_layer();
        }));

   
}
fn clears(string: String, rem: char) -> String{
    let mut news = String::from("");
    for x in string.chars(){
        if x != rem{
            news.push(x);
        }
    }
    return news
}
fn getStock(typ: &str) -> f64 {
    //Last Refreshed
    dotenv().ok();
    //let typ = getin("stock: ");
    let api = dotenv!("API");
  //  let typ = "GME";
    let newurl = format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=5min&apikey={}",typ,api);
    //newurl.push_str(&id);
    //println!("{}",newurl);
    let resp = attohttpc::get(newurl)
                .header("get", "test")
                .send().expect("error");
    
    println!("Status: {:?}", resp.status());
    
    let text = &resp.text().expect("error");
            //println!("Headers:\n{:#?}", resp.headers());
    //println!("Body:\n{}", text);
    let js: Value = serde_json::from_str(text).expect("error");
    let lr = &js["Meta Data"]["3. Last Refreshed"];
    let price = clears(js["Time Series (5min)"][clears(lr.to_string(),'"')]["2. high"].to_string(),'"'); //,clears(lr.to_string(),'"');
    //println!("{} lr:{}",);
    price.parse::<f64>().expect("parse error")
    //price
}

