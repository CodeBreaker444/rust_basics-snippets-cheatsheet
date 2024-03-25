use std::io;
use std::io::Write;
mod data_structures;
mod area_example;
mod methods;
fn main() {
    //Ownerships
    /*
    Stack memory is static and heap memory is dynamic.
    Their is no Grabage collector in RUST to free up memory space of unused variable at runtime.
    So, it uses scope to clear out the memory of variables efficiently.
    */
    let mut s="hello"; //Assigned in stack
    s="zero";
    println!("{}",s);
    let mut t=String::from("Hello");//Assigne in heap
    t.push_str(", World!");
    println!("t-println!={}",t);
    print!("t-print!={}",t);
    let temp=10;
    let temp1=temp;//stack-copy
    println!("temp={},temp1={}",temp,temp1);
    let t1=t;//move
    //println!("{},{}",t,t1);
    /*
    The above line raises error because of double free error avoidance system in RUST.
    Instead of copy the heap which is memory expensive it just moves the data to t1 and invalidates the t.
    */
    let t2=t1.clone();//deep-copy
    println!("t1->t2={},t1={}",t2,t1);
    let num=5;
    let result:u32=fibinocci(num);
    println!("Fibinocci of {} is {}",num,result);
    let mut s=String::from("ownership");
    let s1=gives_ownership(s.clone());//only clone creates a copy of s
    println!("s={},s1={}",s,s1);
    let (s2,ret_value)=tuples(s1);//use tuple to return the string
    println!("s1=Cannot Print Dropped!,s2={},ret_value={}",s2,ret_value);
    let len=param_reference(&s2,&mut s);//s2 as reference instead of moving & Pointer of s2 is passed as parameter
    /*
    Their cannot exist two mutable references in the same scope and
    references cannot by as immutable and mutable in the same scope reference
    Data races can be avoided by these restrictions.
    Only One mutable reference or Any number of immutable refereces
    */
    println!("s2={},len={},s={}",s2,len,s);
    //Dangling pointers never exist with some exceptions
    //Slice type
    let word=first_word(&s[..]);
    let word=first_word(&s);//shadowing
    /*
    Basically &str literal accepts onlyt slices of String's values but not
    the whole STRING at leat what i think but here i can pass both str and String to &str
    whithout any compile time error
    */
    println!("word={}",word);
    //Call for data_structers
    data_structures::base_structure();
    //Call for area examples
    area_example::main();
    //Call for methods file
    methods::main();



}//Calls drop to clear memory which are out of scope.

fn fibinocci(num:u32)->u32{
    println!("num={}",num);
    if num<2{
    return num}
    dbg!(fibinocci(dbg!(num)-2)) + dbg!(fibinocci(dbg!(dbg!(num)-1)))//debugging is super easiy with dbg
}
fn gives_ownership(s:String)->String{
    s
}
fn tuples(s:String)->(String,usize){
    let length=s.len();
    (s,length)
}
fn param_reference(s:&String,s1:&mut String)->usize{
   s1.push_str(" Is referenced and Changed the string");//changed the referenced value by making it immutable
   s.len()
}
fn first_word(s:&str)->&str{
    let bytes=s.as_bytes();//space ascii code number is 32
    println!("s in bytes={:?}",bytes);
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[..i];
        }
    }
    &s[..]
}
