use std::io;
use std::io::Write;
pub fn main(){
    println!("Functions!");
    let x = five();
    sum(x, 7);
    cond();
    loops();
    temp();
}
fn sum(x:u32,y:u32){
    println!("Sum of {} and {} is {}",x,y,x+y);
}
fn five()->u32{
    5
}
//If Expression
fn cond(){
    let max=10;
    if max<50{
        println!("Value is lesser!");
    }
    else if max==10{
        println!("Values Is equal!");
    }else{
        println!("Value is Greater!")
    }
    let con:bool=max>50;
    let number:u32= if max<20 {5} else {0};
    println!("If Condition:{},{}",con,number);
}
//loops: loop, for, while
fn loops(){
    let mut count=0;
    let result =loop{
        count+=1;
        if count==5{
            break count*2;
        }
    };
    println!("loop:{}",result);
    let sample=[14,32,12,55,67];
    let mut index=0;
    while count!=0{
        count-=1;
        println!("Through While-Element {}:{}",index,sample[index]);
        index+=1;
    }
    println!("while:{}",count);
    for element in sample.iter(){
        println!("For Loop- Element:{}",element);
    }
    for i in (1..=5).rev(){
        println!("{}!",i)
    }

}
fn temp(){
    //Celcius to fahrenheit converter
     //let mut f;
    let celcius:f32=loop{
    let mut c=String::new();
    print!("Enter Temperature in celcius : ");
    io::stdout().flush().expect("Cannot Flust Input/Output");
    io::stdin().read_line(&mut c)
    .expect("Cannot Read Line!");
    let c:u32= match c.trim().parse(){
        Ok(n)=>n,
        Err(_)=>{println!("Enter Only Numbers..");continue;},
    };
    break c as f32;
};
     println!("Temperature in Fahrenheit : {}",(celcius*(9 as f32/5 as f32))+32 as f32);

}