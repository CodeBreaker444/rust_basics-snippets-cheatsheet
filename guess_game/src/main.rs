use std::io;
use rand::Rng; //Rng Is a Trait
use std::cmp::Ordering;
use std::io::Write;
fn main(){
    let secret_number = rand::thread_rng().gen_range(1,100);//by default variables are immutable
    let masked="*****";
    let mut count=0;
    println!("----My First Game In RUST----");//Macro keywords are denoted with ! in end
    println!("Guess a Number, Present Secret Number: {}", masked);
    loop{//Endless untill break
        count=count+1;
        print!("Please input your guess, Attempt {}:",count);
        let _=io::stdout().flush();
        let mut guess=String::new();//instance
        io::stdin()
        .read_line(&mut guess)//associated function
        .expect("Failed to read Line");
        //let some:u32=21;
        //println!("{}",some);
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=> {println!("Enter Only Number!");continue;},
        };//matching the return values and shadowing
        match guess.cmp(&secret_number){//Exception handling
            Ordering::Less=>println!("Too Small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{println!("Hurray, You won! in {} attempts.",count);break;},
        }
        //println!("You guessed: {}", guess);
}
}