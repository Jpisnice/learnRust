#[derive(Debug)]
struct Node{
    next:Option<Box<Node>>,
    data: u32,
}

#[derive(Debug)]
struct Stack{
    size: usize,
    top: Option<Box<Node>>
}

impl Stack{
    fn add(&mut self,data:u32){
        if self.get_size() < self.size{
            let t = &mut self.top;
            let new_node = Box::new(Node{
                next:t.take(),
                data,
            });
            self.top = Some(new_node);
        }else{
            panic!("Stack size limit reached")
        }
        
    }

    fn get_size(&mut self)->usize{
        let mut sample = self.top.as_ref();
        let mut c = 0;

        while let Some(node) = sample{
            c+=1;
            sample = node.next.as_ref();
        }
        return c
    }

    fn pop(&mut self) -> Option<u32> {
        if let Some(mut node) = self.top.take() {
            self.top = node.next.take();
            Some(node.data)
        } else {
            None
        }
    }
}

fn main() {
    let mut s = Box::new(Stack{
        size:10,
        top:None,
    });

    s.add(3);
    s.add(4);
    s.add(53);
    s.add(32);
    s.add(59);
    print!("{:#?}",Some(s.pop()));
    s.add(32);

    print!("{:#?}",s);
    print!(" size {}",s.get_size());
}