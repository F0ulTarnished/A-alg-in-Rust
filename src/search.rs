use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::usize;


/*
This code defines functions for search prossess
*/

pub fn search_func(map:&Vec<Vec<i8>>,begin:Rc<RefCell<Node>>,end:Rc<RefCell<Node>>,  path:&mut Vec<(i32,i32)>)
{

    let begin_coord=begin.clone().as_ref().borrow().coord;
    let end_coord=end.clone().as_ref().borrow().coord;
    let end_coord_str=tuple_to_string(end_coord);
    
    let neighbor=vec![(1,0),(0,-1),(-1,0),(-1,0),(0,1),(0,1),(1,0),(1,0)];//reletive place 
    let cost_per_step=vec![10,14,10,14,10,14,10,14];//cost 
    let mut flag:bool=false;

    //let temp=open.pop().unwrap();
    //let node=temp.as_ref().borrow_mut();                                             
    let mut visited:HashMap<String,Rc<RefCell<Node>> >=HashMap::new();
    visited.insert(tuple_to_string(begin.clone().as_ref().borrow().coord.clone()),begin.clone());
    let mut open=vec![begin];//open list  no close list,use a close tag instead

    'outer:while !open.is_empty()  //continue to search until open is empty
    {
        //arrange in decrease order
        open.sort_by(|a,b| b.borrow().f.cmp(&a.borrow().f));

        //get node with min f ,and move to close
        let temp=open.pop().unwrap().clone();//is this clone prevent from disappearance?
        //let mut node=temp.as_ref().borrow_mut();
        //node.close=1;             
        temp.as_ref().borrow_mut().close=1;
        //check neighbors
        let (mut x,mut y)=temp.as_ref().borrow().coord;
        for i in 0..8
        {
            x=x+neighbor[i].0;
            y=y+neighbor[i].1;
            println!("({},{})",x,y);
            if x >= 0 && (x as usize) < map.len()  && y >= 0 && (y as usize)< map[0].len() && map[x as usize][y as usize] ==0
            {   
                let key_str=tuple_to_string((x,y));
                //if neighbor hasn' been visited
                if !visited.contains_key(&key_str)
                {
                    //the new node isn't the end
                    if tuple_to_string((x,y))!=end_coord_str// may find this "if" logic irregular,try consider the often case
                    {
                        //creat a new node
                        let new_node_wrapped=Node::new(0, 0, 0, (x,y), Some(temp.clone()));
                        let mut new_node=new_node_wrapped.as_ref().borrow_mut();
                        new_node.g=new_node.g();
                        new_node.h=new_node.h(end.clone());
                        new_node.f=new_node.g+new_node.h;
                        new_node.coord=(x,y);
                        new_node.parent=Some(temp.clone());
        
                        //add to open
                        open.push(new_node_wrapped.clone());
                        visited.insert(key_str, new_node_wrapped.clone());
                    }
                    //new node is the end
                    else 
                    {
                        let mut end_node=end.as_ref().borrow_mut();
                        end_node.parent=Some(temp.clone());
                        flag=true;
                        break 'outer;    
                    }
                }
                //if visited
                else  
                {
                    let visited_node_wrapped=visited.get(&key_str).unwrap();
                    let mut visited_node=visited_node_wrapped.as_ref().borrow_mut();
                    if visited_node.close==0
                    {
                        //better path found, update it
                        let cur_g=temp.as_ref().borrow().g+cost_per_step[i];
                        if cur_g<visited_node.g
                        {
                            visited_node.parent=Some(temp.clone());
                            visited_node.g=cur_g;
                            visited_node.f=visited_node.h+cur_g;
                            
                        }
                    }  
                     
                }
            }
        }
    }
    if flag
    {
         //acquire the path
    let mut cur_coord=end_coord;
    let mut cur_node=end.clone();
    path.insert(0, end_coord);
    while cur_coord!=begin_coord
    {
        cur_node=cur_node.clone().as_ref().borrow().parent.clone().expect("Ops");
        cur_coord=cur_node.as_ref().borrow().coord;
        path.insert(0, cur_coord);
        
    }
    }
    
   
    
}

//allow hashmap to work with Key
fn tuple_to_string(t: (i32, i32)) -> String {
    format!("{}_{}", t.0, t.1)
}












//defination of linknode  choose the way of finging path that searches from the end point 
#[derive(Debug,PartialEq, Eq)]
pub struct Node {
    pub f: i32,                           //cost f=g+h
    pub g: i32,                           //actual cost
    pub h: i32,                           //predicted cost
    pub close:i8,                       //in closelist tag
    pub coord:(i32,i32),                //current node place
    pub parent: Option<Rc<RefCell<Node>>>,   //parent node
}

/*#[derive(Debug)]
pub struct SearchGraph {
    head: Option<Rc<RefCell<Node>>>,
    size: usize,
}
//new a list
impl SearchGraph
{
    pub fn new() -> Self {
        SearchGraph {
            head: None,
            size: 0,
        }
    }
}
    */
//new a node
impl Node
{
    pub fn new(_f:i32,_g:i32,_h:i32,_coord:(i32,i32),_parent:Option<Rc<RefCell<Node>>>) ->Rc<RefCell<Node>>{
        let pointer=Rc::new(RefCell::new(Node{
            f:_f,
            g:_g,
            h:_h,
            close:0,
            coord:_coord,
            parent:_parent 

        }));
        Rc::clone(&pointer)
    }
    /*pub fn f(&self, end: Rc<RefCell<Node>>) -> i32
    {
        self.g()+self.f(end)
    }*/

    pub fn g(&self) -> i32
    {
        let n = self;
        let tmp=self;
        let m = tmp.parent.as_ref().unwrap().borrow();
        
        let parent_g = m.g;
        if n.coord.0 - m.coord.0 == 0 || n.coord.1 - m.coord.1 == 0 {
            parent_g + 10
        } else {
            parent_g + 14
        }
    }
    pub fn h(& self,end:Rc<RefCell<Node>>)->i32
    {
        let n=self;
        let e=end.borrow();
        //apply manhadun
        (n.coord.0-e.coord.0).abs()*10+(n.coord.1-e.coord.1).abs()*14
    }
}


