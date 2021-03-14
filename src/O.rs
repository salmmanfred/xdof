use cursive;
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
    LinearLayout, SelectView};
use cursive::traits::*;
//use std::{thread, time};
/*#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;*/
use dotenv::dotenv;
//use std::env;
use serde_json::Value;
use attohttpc;
use rustDatabook;
use crate::X;
use crate::refresh;
use chrono::prelude::*;
use crate::F;

pub fn getStockChart(typ: &str) -> String {
    fn clears(string: String, rem: char) -> String{
        let mut news = String::from("");
        for x in string.chars(){
            if x != rem{
                news.push(x);
            }
        }
        return news
    }
    //Last Refreshed
    dotenv().ok();
    //let typ = getin("stock: ");
    let api = dotenv!("API");
  //  let typ = "GME";
    //let newurl = format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=5min&apikey={}",typ,api);
    let newurl = format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}",typ,api);
    //newurl.push_str(&id);
    //println!("{}",newurl);
    let resp = attohttpc::get(newurl)
                .header("get", "test")
                .send().expect("error");
    
    println!("Status: {:?}", resp.status());
    
    let text = &resp.text().expect("error");
            //println!("Headers:\n{:#?}", resp.headers());
    //println!("Body:\n{}", text);
    let js: Value = serde_json::from_str(text).unwrap();
    let lr = &js["Meta Data"]["3. Last Refreshed"];
    //clears(lr.to_string(),'"')]["2. high"
    let mut ves: Vec<String> = Vec::new();
    //let ddn = NaiveDate::parse_from_str(now()).unwrap();
    let duration = now().signed_duration_since(thisweek());
    println!("{:?}",duration.num_days());
 

    for x in thisweek().iter_days().take(duration.num_days() as usize){
        let d = format!("{:?}",x);

        println!("working,{:?}",x);
        let price = clears(js["Time Series (Daily)"][d]["2. high"].to_string(),'"'); //,clears(lr.to_string(),'"');
        ves.push(price);
    }
    //println!("{} lr:{}",);
    //price.parse::<f64>().expect("parse error");
    openfile::writeFile(&format!("{}{}",F::DIREC,F::COOL1), &ves.join(" "));
    asemb(ves)


    //price
}

fn thisweek() -> chrono::NaiveDate {
    let current_year = chrono::offset::Local::now().year();
    let week: DateTime<Local> = Local::now();
    //week.iso_week();
    let mon = NaiveDate::from_isoywd(current_year, week.iso_week().week(), Weekday::Mon);
    
    mon
    
}
fn now() -> chrono::NaiveDate{
    let current_year = chrono::offset::Local::now().year();
    let week: DateTime<Local> = Local::now();
    //week.iso_week();
    let mon = NaiveDate::from_isoywd(current_year, week.iso_week().week(), week.weekday());
    
    mon
}


fn asemb(ves: Vec<String>) -> String{
    //getDial(ves.clone());
    let strin  = "         #";
    let strin2 = "        ##";
    let strin3 = "       ###";
    let com = getDial(ves.clone());
    let mut refS: String = "".to_string();
    for x in 0..com[0].len(){
        for a in 0..com.len(){
            refS.push(com[a].chars().nth(x).unwrap());
        }
        refS.push_str("\n");
    }
    //print!("{}",refS);
    refS
}
fn getDial(price1: Vec<String>) -> Vec<String>{
    let mut globalModif = 0;
    let mut Hei = 10;
    let maxhei = 50;
    let mut sizes: Vec<i64> = Vec::new();
    for price1 in price1{
        let mut price = price1.chars().nth(0).unwrap().to_string();
        let pp = price.parse::<i64>().expect("parse error");
        let pp2 = price1.len() as i64;
        let pp3s = price1.parse::<f64>().expect("s")  / pp2 as f64;
        let pp3e = pp3s.to_string().chars().nth(0).unwrap().to_string();
        let pp3 = pp3e.parse::<i64>().expect("s");
        let mut sizep = pp * pp2 * pp3 as i64;
        let mut size = 0;
        if sizep.to_string().len() >= 2{
            let mut wtf = sizep.to_string().chars().nth(0).unwrap().to_string();
            wtf.push(sizep.to_string().chars().nth(1).unwrap());
            size = wtf.parse::<i64>().unwrap();

        }else{
            let wtf = sizep.to_string().chars().nth(0).unwrap().to_string();
            size = wtf.parse::<i64>().unwrap();
        }
        size -= globalModif;
        if size > Hei && !size > maxhei{
            Hei = size;
        }
        while size > maxhei{
            size -= 1;
            globalModif += 1;
        }
        sizes.push(size);
        
        //println!("size{}",size);

       
    }
    let mut fins: Vec<String> = Vec::new();
    for x in sizes.clone(){
        let mut tempS = "".to_string();
        for i in 0..Hei{
            let mut yes = 0;
            for x in sizes.clone(){
                if x> i+1{
                    yes += 1;
                }
            }
            if yes < sizes.len(){
                if x> i{
                    tempS.push_str("#");
                }else{
                    tempS.push_str(" ");
                }
            }
        }
        tempS = tempS.chars().rev().collect();
        //println!("size{}",tempS);
        fins.push(tempS);

    }
    for x in 0..Hei{

    }
    
    fins

    

}
