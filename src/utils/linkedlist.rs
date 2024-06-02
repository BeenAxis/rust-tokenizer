use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::rc::Rc;
use crate::utils::voids::{IndexVoid, Void};


pub struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T : Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
    }
}

impl<T> Node<T> {
    pub fn new(t: T) -> Node<T> {
        Node {
            value: t,
            next: None,
            prev: None,
        }
    }
}

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<NodeRef<T>>;

pub struct LinkedHashList<T> {
    start: Link<T>,
    tail: Link<T>,
    im_map: ImaginaryHashMap<T>,
    len: usize
}

pub struct ImaginaryHashMap<T> {
    indexes: HashMap<usize, Link<T>>,
    index_void: IndexVoid,
}

impl<T> ImaginaryHashMap<T> {
    pub fn new() -> ImaginaryHashMap<T> {
        ImaginaryHashMap {
            index_void: IndexVoid::new(),
            indexes: HashMap::new()
        }
    }
    
    pub fn insert(&mut self, key: usize, value: Link<T>) {
        self.indexes.insert(key, value);
    }
    
    pub fn clear(&mut self) {
        self.index_void.clear();
        self.indexes.clear();
    }
    
    pub fn len(&self) -> usize {
        self.indexes.len() - self.index_void.del_gap()
    }
    
    pub fn add_void(&mut self, gap: Void) {
        println!("Add void: {gap:?}");
        self.index_void.insert(gap);
    }
    
    pub fn get(&self, im_idx: &usize) -> Option<&Link<T>> {
        let real = &self.index_void.real_index(*im_idx);
        println!("get: {im_idx} -> {real}");
        self.indexes.get(real)
    }
}

impl<T: Debug> Debug for LinkedHashList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{:?}", first.borrow().value)?;
            for node in iter {
                write!(f, " -> {:?}", node.borrow().value)?;
            }
        }
        Ok(())
    }
}

pub struct LinkedHashListIterator<T> {
    current: Link<T>,
}

impl<T> Iterator for LinkedHashListIterator<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.borrow().next.clone();
            node
        })
    }
}



impl<T> LinkedHashList<T> {
    pub fn new() -> LinkedHashList<T> {
        LinkedHashList {
            start: None,
            tail: None,
            im_map: ImaginaryHashMap::new(),
            len: 0
        }
    }
    
    pub fn len(&self) -> usize {
        self.im_map.len()
    }

    pub fn iter(&self) -> LinkedHashListIterator<T> {
        LinkedHashListIterator {
            current: self.start.clone(),
        }
    }

    pub fn reindex(&mut self) {
        self.im_map.clear();

        let mut new_indexes = ImaginaryHashMap::new();
        self.iter().enumerate().for_each(|(idx, node_ref)| {
            new_indexes.insert(idx, Some(node_ref.clone()));
        });
        
        self.im_map = new_indexes;
        self.len = self.im_map.len();
    }

    pub fn replace(&mut self, range: Range<usize>, element: T) {
        //A <-> B <-> C <-> D
        //A <-> [B] <-> [C] <-> D
        //A <->
        let new_node = Node::new(element);
        let refer = Rc::new(RefCell::new(new_node));

        let start_ref = self.im_map.get(&range.start).unwrap();
        let tail_ref = self.im_map.get(&range.end).unwrap();
        if let (Some(start_ref), Some(tail_ref)) = (start_ref, tail_ref) {
            if let Some(prev_node) = &start_ref.borrow().prev {
                prev_node.borrow_mut().next = Some(refer.clone());
            }
            if let Some(next_node) = &tail_ref.borrow().next {
                next_node.borrow_mut().prev = Some(refer.clone());
                refer.borrow_mut().next = Some(next_node.clone());
            }
        }
        self.im_map.add_void(Void::from(range.start + 1 .. range.end));
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node::new(element);
        let refer = Rc::new(RefCell::new(new_node));
        self.im_map.insert(self.len, Some(refer.clone()));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(refer.clone());
                refer.borrow_mut().prev = Some(old_tail);
            }
            None => self.start = Some(refer.clone()),
        }

        self.tail = Some(refer.clone());
        self.len += 1;
    }
}