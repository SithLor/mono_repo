//make java array api
pub mod link;
pub mod color;
pub mod car;
pub mod kong;
pub fn has_java() -> bool {
    use std::env;
    let java_home: Result<String, env::VarError> = env::var("JAVA_HOME");
    return java_home.is_ok();
}
#[macro_export]
macro_rules! NOT_IMPLEMENTED {
    () => {
        panic!(Text!{
            "Not implemented",
            "Please implement this function"
        });
    };
}
#[macro_export]
macro_rules! Text {
    ($($x:expr),*) => {
        concat!($(stringify!($x), "\n",)*)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    fn marco_text(){
       let text = Text! {
            "loo"
       }; 
       println!("{}",text);
    }
}
