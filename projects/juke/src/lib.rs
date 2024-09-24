
pub mod strings {

    pub fn remove_char_at(s:&str,postions:Vec<usize>)->String{
        let mut result:String = String::new();
        for (i,c) in s.chars().enumerate(){
            if !postions.contains(&i){
                result.push(c);
            }
        }
        result
    }
    //String::new("hello world"), let e = "hello".to_string()
    
    pub fn spilt_string(s:&str)->Vec<String>{
        let mut result = Vec::new();
        let mut temp = String::new();
        for c in s.chars(){
            if c.is_alphabetic(){
                temp.push(c);
            }else{
                if !temp.is_empty(){
                    result.push(temp.clone());
                    temp.clear();
                }
            }
        }
        if !temp.is_empty(){
            result.push(temp);
        }
        result
    }
}





pub mod java;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ArrayList_test(){
        let mut list = java::ArrayList::<i32>::new();
        list.add(1);
        list.addFirst(1)
    }
}
