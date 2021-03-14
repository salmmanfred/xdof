use cursive;
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
    LinearLayout, SelectView};
use cursive::traits::*;
//use std::{thread, time};
use rustDatabook;
use std::path::Path;
use std::fs;
mod X;
mod D;
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

    s.add_layer(Dialog::text("Starting")
        .title("Loading")
        );
   
    if !Path::new(&format!("{}{}",F::DIREC,F::FILER)).exists(){
        fs::create_dir_all(F::DIREC).expect("er");
        openfile::writeFile(&format!("{}{}",F::DIREC,F::FILER), "");
    }
    
    s.pop_layer();
    sl(s);
    
    //didl(s, ".rdb ERROR");
    
    
    
    
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
   

    s.add_layer(Dialog::text(format!("Name: {}\nStock: {}\nAmount: {}\nBuy price: {}\n",o[0],o[1],o[2],o[3]))
    .title("Stock")
    .button("OK", |s| {
        s.pop_layer();
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


