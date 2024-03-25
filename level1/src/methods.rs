use std::io;//methods
#[derive(Debug)]//For printing the whole struct data
struct Data{
    port_l:u32,
    port_r:u32,

}
impl Data{//defining a function in the context of struct also called as methods
    fn modify(&self)->u32{
        self.port_r+self.port_l
    }
    fn can_hold(&self,det: &Data)->bool{
        self.port_l>det.port_l && self.port_r>det.port_r
    }
}
pub fn main(){
    let data1=Data{
        port_l:1247,
        port_r:8088,
    };
    let data2=Data{
        port_l:3234,
        port_r:1232,
    };
    println!("{},{}",data1.modify(),data1.can_hold(&data2));

}