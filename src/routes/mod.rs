pub mod hello_user;
pub use hello_user::*;

pub mod home;
pub use home::*;

fn logging(path:&str){
    println!("{}",path);
}