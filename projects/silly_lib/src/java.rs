pub struct ArrayList<T>{
    data: Vec<T>
}
impl <T> ArrayList<T>{
    pub fn new()->ArrayList<T>{
        ArrayList{
            data: Vec::new()
        }
    }
    pub fn add(&mut self,element:T){
        self.data.push(element);
    }
    pub fn add_at_index(&mut self,index:usize,element:T){
        self.data.insert(index,element);
    }
    pub fn addFirst(&mut self,element:T){
        self.data.insert(0,element);
    }
    pub fn addLast(&mut self,element:T){
        self.data.push(element);
    }
    pub fn clear(&mut self){
        self.data.clear();
    }
//    fn clone(&self)->ArrayList<T>{
//
//    }
    //fn equals(&self,other:&ArrayList<T>)->bool{
    //    self.data == other.data
    //}
    pub fn get(&self,index:usize)->Option<&T>{
        self.data.get(index)
    }
    pub fn getFirst(&self)->Option<&T>{
        self.data.first()
    }
    pub fn getLast(&self)->Option<&T>{
        self.data.last()
    }
    //fn indexOf(&self,element:&T)->Option<usize>{
    //    self.data.iter().position(|e| e == element)
    //}
    pub fn size(&self)->usize{
        self.data.len()
    }
}