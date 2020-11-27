use std::io;
use std::io::Write;
fn main(){
    base_structure();
}
//Structs are similar to tuples.

struct user{
        username: String,
        email: String,
        login_count: u64,
        active: bool,
        blocked: bool,
    }
pub fn base_structure(){
  
    let mut user1=user{
        blocked: false,
        active: true,
        username: String::from("codebreaker"),
        email: String::from("zeroerror.444@gmail.com"),
        login_count: 1,
    };
    user1.email=String::from("sample@gmail.com");
    println!("user1.active={}",user1.active);
    let user2=create_user(String::from("zeroerror.444@gmail.com"), String::from("codebreaker"));
    let user3=user{
        blocked: user2.blocked,
        active: user2.active,
        username: String::from("codebreaker"),
        email: String::from("zeroerror.444@gmail.com"),
        ..user2//To set ramining fields same as the user2 field values 
    };
    struct color(i32, i32, i32);//Tuples as struct
    struct points(i32,i32,i32);
    let white =color(255,255,255);
    let origin=points(0,0,0);//Here color and points are two different type even though they are made up of same type of values

}
struct uploads{

}//Unit like struct which have no fields.
fn create_user(email: String, username: String)->user{
    user{
        email,//Short hands for email: email because the parameter name and struct field are same
        username,
        active: true,
        blocked: true,
        login_count: 1,
    }
   

}
/*
Final Notes
You can reference field values to other variables as:
struct user{
    email: &str,
}
but it will not because we have specify lifetime which is not finished in this file
, As to my understanding this is because of the scopes. Struct values are valid untill struct is valid
which means all the data is owned by Struct. Then what about the scope of other variable reference which 
may go out of scope.
*/