use crate::F;
use rustDatabook;
pub fn getdata() -> rustDatabook::dataBook{
    rustDatabook::rdbData(&format!("{}{}",F::DIREC,F::FILER))
    
    
}
pub fn deleteData(item: String){
    rustDatabook::removeData(&format!("{}{}",F::DIREC,F::FILER), &item);

}
pub fn getCont(item: &str) -> Vec<String>{
    let book = getdata();
    let pos = rustDatabook::findValue(item, &book, "_A");
    let o = book.A[pos].clone();
    return o
}
pub fn addData(data: &str){
    rustDatabook::addData(&format!("{}{}",F::DIREC,F::FILER), data);
}