#[derive(Debug)]//Annotation for making explicit to our struct
struct Dimensions{
    height: u32,
    width: u32,
}
pub fn main(){
    let height=20;
    let width=13;
    let dimensions=(22,10);
    let area1= area_basic(height,width);
    let area2=area_tuples(dimensions);
    let rectangle= Dimensions{
        height:50,
        width:25,
    };
    let area3=area_structure(&rectangle);//Rectangle values are referenced so that they dont go invalid after this line
    println!("area_basic({},{})={},area_tuples({},{})={},area_structure({},{})={}",
    height,width,area1,dimensions.0,dimensions.1,area2,rectangle.height,rectangle.width,area3);
    println!("{:?}",rectangle);// We defined explicitly the trait for the specific trait
}
fn area_basic(h:u32,w:u32)->u32{
    let area=h*w;
    area

}
fn area_tuples(dimen:(u32,u32))->u32{
    let area=dimen.0*dimen.1;
    area
}
fn area_structure(rectangle: &Dimensions)->u32{
    let area=rectangle.height*rectangle.width;
    area
}