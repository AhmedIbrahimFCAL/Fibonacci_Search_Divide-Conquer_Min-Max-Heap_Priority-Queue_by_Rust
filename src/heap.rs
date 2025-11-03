use std::{fmt::{Debug, Display}, mem::swap};

use colored::*;


#[derive(Debug)]
pub enum Heap_Type{
    Max,
    Min,
}

#[derive(Debug,Clone,Copy)]
enum Shown<T: Display + Clone>{
    Value(T),
    Leaf(T),
    Dash,
    Slash,
    BackSlash,
    Empty,
}

#[derive(PartialEq)]
enum Cell<T:Display + Clone+PartialEq>{
    Value(T),
    Leaf(T),
    Empty,
}
impl<T:Display + Clone + PartialEq> Display for Cell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Value(value) => write!(f,"{}",value),
            Cell::Leaf(value) => write!(f,"{}",value),
            Cell::Empty =>{
                if f.alternate() {
                    write!(f,"Empty")
                }else{
                    write!(f,"")
                }
            }
        }
    }
}

/* len: 7
    0
 1     2
3 4   5 6
*/
#[derive(Debug)]
pub struct Heap<T: PartialOrd>{
    arr:Vec<T>,
    heap_type:Heap_Type,
}

impl<T: PartialOrd + Copy + Debug + Display> Heap<T>{
    fn new(arr:Vec<T>,heap_type:Heap_Type)->Self{
        Self{
            arr,
            heap_type,
        }
    }
    fn compare(&self, i:usize, j:usize)->bool{
        self.compare_type(self.arr[i], self.arr[j])
    }
    fn compare_type(&self,x:T,y:T)->bool{
        match self.heap_type {
            Heap_Type::Max => x > y,
            Heap_Type::Min => x < y
        }
    }
    fn size(&self)->usize{
        self.arr.len()
    }
    fn left_child_index(i:usize)->usize{
        Self::right_child_index(i)-1
    }
    fn right_child_index(i:usize)->usize{
        (i+1)<<1
    }
    fn parent_index(i:usize)->usize{
        (i-1)>>1
    }
    fn swap(&mut self, x:usize, y:usize){
        let z = self.arr[x];
        self.arr[x] = self.arr[y];
        self.arr[y] = z;
    }
    
    fn clear(&mut self){
        self.arr.clear();
    }

    // from root to leaf
    fn heapify(&mut self,mut i:usize, n:usize){
        loop{
            let l = Self::left_child_index(i);
            let r = l + 1;
            let mut largest= i;
            if l<n && self.compare(l, largest) {
                largest = l;
            }
            if r<n && self.compare(r, largest) {
                largest = r;
            }
            if largest == i {
                break;
            }
            self.swap(i,largest);
            i = largest;
        }
    }
    fn build_heap(&mut self){
        let n = self.arr.len();
        for i in (0..n>>1).rev() {
            self.heapify(i,n);
        }
    }
    fn sort(&mut self){
        self.build_heap();
        // println!("{:?}",self.arr);
        for i in (1..self.arr.len()).rev(){
            self.swap(0, i);
            self.heapify(0,i);
        }
    }
    fn get_root(&self)->Option<T>{
        if self.arr.len()==0 {
            return None;
        }
        Some(self.arr[0].clone())
    }
    fn get_leaf(&self)->Option<T>{
        let n = self.arr.len();
        if n == 0 {
            return None;
        }
        let mut leaf = self.arr[n>>1];
        for i in (n>>1)..n {
            if self.compare_type(leaf, self.arr[i]){
                leaf = self.arr[i];
            }
        } 
        Some(leaf)
    }
    fn extract_root(&mut self)->Option<T>{
        let n = self.arr.len(); 
        if n == 0{
            return None;
        }
        self.swap(0, n-1);
        self.heapify(0, n-1);    
        self.arr.pop()
    }
    fn insert(&mut self,value:T){
        self.arr.push(value);
        let mut i = self.arr.len()-1;
        while i>0 {
            let par = Self::parent_index(i);
            if self.compare(par, i){
                break;
            }
            self.swap(par, i);
            i=par;
        }
    }
    fn increase_key(&mut self,mut i:usize,key:T)->Result<(),String>{
        if key <= self.arr[i]{
            return  Err(String::from("new key is smaller than current key"));
        }
        self.arr[i] = key;
        while i>0 {
            let par = Self::parent_index(i);
            if self.compare(par, i){
                break;
            }
            self.swap(par, i);
            i=par;
        }
        Ok(())
    }

}

/* len: 7
    0
 1     2
3 4   5 6
*/
fn print_as_tree<T:Display + Clone+PartialEq+Copy+Debug>(data:&Vec<Cell<T>>){
    let n = data.len();
    let height = ((n+1) as f32).log2().ceil() as usize;// as usize *2 -1;
    let width = (1 << height)-1;
    let height = height*2 -1;
    let mut show:Vec<Vec<Shown<T>>> = vec![vec![Shown::Empty;width];height];
    navigate(data,&mut show, n, height,0, 0, (width>>1) as i32, ((width+1)>>2) as i32);    
    let mut spaces = 0;
    for i in 0..n{
        spaces = spaces.max(format!("{}",data[i]).len());
    }
    print_2d_tree(&show, width, height, spaces);
}
fn set_dash<T:Display+Clone>(show:&mut Vec<Vec<Shown<T>>>,i:usize,mut j:i32,mut l:i32){
    if j>l{
        swap(&mut j,&mut l);
    }
    for k in j+1 .. l{
        show[i][k as usize]=Shown::Dash;
    }
}
fn print_2d_tree<T:Display+Clone>(show:&Vec<Vec<Shown<T>>>,rows:usize,height:usize,spaces:usize){
    /*    ______50______        
         /              \       
      __20__          __90      
     /      \        /
    10      30      80
              \
              40
    */
    println!();
    for i in 0..height{
        for j in 0..rows{
            match &show[i][j]{
                Shown::Dash  => print!("{:_>spaces$}", '_'),
                Shown::Empty => print!("{: >spaces$}", ' '),
                Shown::Slash => print!("{: >spaces$}", '/'),
                Shown::BackSlash => print!("{: <spaces$}", '\\'),
                Shown::Value(value)=> print!("{:_^spaces$}",value),
                Shown::Leaf(value) => print!("{:^spaces$}",value),
            }        
        }
        println!();
    }
}
fn navigate<T:Display + Clone + PartialEq+Copy>(data:&[Cell<T>],show:&mut Vec<Vec<Shown<T>>>,
    n:usize, height:usize, current:usize, i:usize, j:i32, s:i32){
    if current >= n {
        return;
    }
    match data[current] {
        Cell::Value(value) => show[i][j as usize]=Shown::Value(value),
        Cell::Leaf(value)  => show[i][j as usize]=Shown::Leaf(value),
        Cell::Empty => return
    }
    let mut child= (current<<1) +1;
    if child<n && data[child]!=Cell::Empty{
        set_dash(show,i as usize,j ,j-s);
        show[(i+1) as usize][(j-s) as usize]=Shown::Slash;
        navigate(data,show,n,height,child,i+2,j-s,s>>1);
    }
    child+=1; // go to right child
    if child <n && data[child]!=Cell::Empty{
        set_dash(show,i as usize,j,j+s);
        show[(i+1) as usize][(j+s) as usize]=Shown::BackSlash;
        navigate(&data,show,n,height,child,i+2,j+s,s>>1);
    }
}

impl<T:PartialOrd + Display + Copy + Clone + Debug> Display for Heap<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let mut data:Vec<Cell<T>> = Vec::new();
            let n = self.arr.len();
            for i in 0..n/2{
                data.push(Cell::Value(self.arr[i]));
            }
            for i in n/2..n{
                data.push(Cell::Leaf(self.arr[i]));
            }
            let height = ((n+1) as f32).log2().ceil() as usize;// as usize *2 -1;
            let width = (1 << height)-1;
            let height = height*2 -1;
            let mut show:Vec<Vec<Shown<T>>> = vec![vec![Shown::Empty;width];height];
            navigate(&data,&mut show, n, height,0, 0, (width>>1) as i32, ((width+1)>>2) as i32);    
            let mut spaces = 0;
            for i in 0..n{
                spaces = spaces.max(format!("{}",data[i]).len());
            }
             
            /*      ______10______
                   /              \
                __9___          __8___
               /      \        /      \
              7_      5       3       1
             /
            0
            */
            let formated = "\x1b[1;32m"; // Bold, Green
            let reset = "\x1b[0m";
            write!(f,"\n")?;
            for i in 0..height{
                for j in 0..width{
                    match show[i][j]{
                        Shown::Dash  => write!(f,"{:_>spaces$}", "_".repeat(spaces).truecolor(165, 42, 42).bold())?,
                        Shown::Empty => write!(f,"{: >spaces$}", ' ')?,
                        Shown::Slash => write!(f,"{: >spaces$}", "/".truecolor(165,42,42).bold())?,
                        Shown::BackSlash => write!(f,"{: <spaces$}", "\\".truecolor(165,42,42).bold())?,
                        Shown::Value(value)=> {
                            write!(f,"{formated}{:_^spaces$}{reset}",value)?
                        },
                        Shown::Leaf(value) => {
                            write!(f,"{formated}{: ^spaces$}{reset}",value)?},
                    }
                }
                write!(f,"\n")?;
            }
            Ok(())
        }else{
            match self.heap_type {
                Heap_Type::Max => write!(f, "Max: {:?}",self.arr),
                Heap_Type::Min => write!(f, "Min: {:?}",self.arr),
            }
        }
    }
}

#[derive(Debug)]
pub struct Priority_Queue<T:PartialOrd>{
    max_heap:Heap<T>,
}
impl<T: PartialOrd + Copy + Debug + Display> Priority_Queue<T>{
    fn new()->Self{
        Self{
            max_heap: Heap::new(Vec::new(), Heap_Type::Max)
        }
    }
    fn push(&mut self,value:T){
        self.max_heap.insert(value);
    }
    fn pop(&mut self)->Option<T>{
        self.max_heap.extract_root()
    }
    fn front(&mut self)->Option<T>{
        self.max_heap.get_root()
    }
    fn size(&self)->usize{
        self.max_heap.size()
    }
    fn clear(&mut self){
        self.max_heap.clear();
    }

}
impl<T:PartialOrd + Display + Debug + Copy> Display for Priority_Queue<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.max_heap,f)
        // if f.alternate() {
        //     write!(f,"{:#}",self.max_heap)
        // }else{
        //     write!(f,"{}",self.max_heap)
        // }
    }
}
pub fn test_heap(){
    println!("Implement Heap as Max-Heap:");
    let mut arr = vec![5,3,1,7,8,9,0];
    println!("Create max heap with values: {:?}",arr);
    let mut heap = Heap::new(arr, Heap_Type::Max);
    heap.build_heap();
    println!("after building heap: {}",heap);
    println!("max: {}, min: {}",heap.get_root().unwrap(), heap.get_leaf().unwrap());
    println!("The heap as a tree:{:#}",heap);
    let value = 10;
    heap.insert(value);
    println!("The heap after inserting {value} is {}",heap);
    println!("Heap size is {}",heap.size());
    heap.sort();
    println!("The heap after sorting is {}",heap);
    println!();

    println!("Implement Heap as Min-Heap:");
    let mut arr = vec![5,3,1,7,8,9,0];
    println!("Create min heap with values: {:?}",arr);
    let mut heap = Heap::new(arr, Heap_Type::Min);
    heap.build_heap();
    println!("after building heap: {}",heap);
    println!("min: {}, max: {}",heap.get_root().unwrap(), heap.get_leaf().unwrap());
    let value = 10;
    heap.insert(value);
    println!("The heap after inserting {value} is {}",heap);
    print!("The heap as a tree: {:#}",heap);
    println!("Heap size is {}",heap.size());
    heap.sort();
    println!("The heap after sorting is {}",heap);
    print!("The heap as a tree: {:#}",heap);
    println!();


    println!("The Priority Queue:");
    let mut priority_queue = Priority_Queue::new();
    let values = vec![1,5,3,7,2,9];
    priority_queue.push(values[0]);
    println!("The priority queue after insert {} is {}",values[0],priority_queue);
    priority_queue.push(values[1]);
    println!("The priority queue after insert {} is {}",values[1],priority_queue);
    println!("the front is {}",priority_queue.front().unwrap());
    priority_queue.pop();
    println!("The priority queue after poping: {}",priority_queue);
    priority_queue.push(values[2]);
    println!("The priority queue after insert {} is {}",values[2],priority_queue);
    priority_queue.push(values[3]);
    println!("The priority queue after insert {} is {}",values[3],priority_queue);
    print!("The prioriy queue as a tree: {:#}",priority_queue);
    priority_queue.pop();
    println!("The priority queue after poping: {}",priority_queue);
    println!("the front is {}",priority_queue.front().unwrap());
    priority_queue.push(values[4]);
    println!("The priority queue after insert {} is {}",values[4],priority_queue);
    priority_queue.push(values[5]);
    println!("The priority queue after insert {} is {}",values[5],priority_queue);
    println!("the front is {}",priority_queue.front().unwrap());
    println!("The priority queue size: {}",priority_queue.size());
    priority_queue.clear();
    println!("The priority queue after clear: {}",priority_queue);

}
