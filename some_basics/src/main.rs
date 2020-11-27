mod functions;
fn main() {
    let x=0.2;//floating points
    let y:f32=000.02;
    println!("{},{}",x,y);
    //numbers can be decimals, hex, binary, octals, byte

    let t=true;//boolean type
    let f:bool=false;
    println!("{},{}",t,f);

    let c='h';//character type
    let ch:char='i';
    let e='😁';//support unicode encoding
    println!("{}{}{}",c,ch,e);

    const MAX:u32=100_000;//underscore is viasula operator for space
    println!("{}",MAX);//const requires data type assignment

    let tuple=(1,1.0,2.03);//tuples can contain multi-type
    let tuple2:(u32,i32,f32)=(3,-23,2.0005);
    println!("{},{},{}",tuple.0,tuple.1,tuple.2);
    println!("{},{},{}",tuple2.0,tuple2.1,tuple2.2);

    let array=[1,2,3,4,5];//arrays
    let _array1:[u32;5]=[2,3,1,3,4];
    let array2=[2;3];
    println!("{}",array[1]);//outofbound protection
    println!("{:?}",array2);

    //Functions
    functions::main();

}