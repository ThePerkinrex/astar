use std::{collections::HashMap, usize};

pub struct Graph<T>(Vec<GraphNode<T>>);

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn add_node(&mut self, data: T) -> usize {
        self.add_node_with_children(data, Vec::new())
    }

    pub fn add_node_with_children(&mut self, data: T, children: Vec<(usize, usize)>) -> usize {
        let r = self.0.len();
        self.0.push(GraphNode(data, children));
        r
    }

    pub fn add_child(&mut self, parent: usize, distance: usize, data: T) -> usize {
        self.add_child_with_children(parent, distance, data, Vec::new())
    }

    pub fn add_child_with_children(
        &mut self,
        parent: usize,
        distance: usize,
        data: T,
        children: Vec<(usize, usize)>,
    ) -> usize {
        assert!(parent < self.0.len(), "the parent must be in the graph");
        let index = self.add_node_with_children(data, children);
        self.0[parent].1.push((distance, index));
        index
    }

    pub fn add_children_to_node(&mut self, parent: usize, child: usize, distance: usize) {
        assert!(parent < self.0.len(), "the parent must be in the graph");
        assert!(child < self.0.len(), "the parent must be in the graph");
        self.0[parent].1.push((distance, child));
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn astar_path_find<E: Fn(usize, &T) -> bool, H: Fn(usize, &T) -> usize>(
        &self,
        start: usize,
        end_condition: E,
        heuristic: H, // Ignored for now (using dijkstra alone for now)
    ) -> Option<Vec<usize>>{
        let mut done: HashMap<usize, (usize, Parent)> = Default::default();
        let mut stack: Stack<(usize, usize, Parent)> = Stack::new();
        stack.push((start, 0, Parent::Top), heuristic(start, &self.0[start].0));

        let end_node = loop {
			
            let ((idx, d, parent), _d_and_h) = stack.pop();
			println!("Checking {}", idx);
            let GraphNode(node_data, node_children) = &self.0[idx];
            

            for (distance, child) in node_children {
				let new_d = *distance + d;
				let new_d_and_h = new_d + heuristic(*child, &self.0[*child].0);
				stack.remove(|(index, _d, _parent), d1| {
					index == child && d1 > &new_d_and_h
				}); // Remove the child if present and has a higher distance, then replace
				stack.push((*child, new_d, Parent::Parent(idx)), new_d_and_h)
			}
			done.insert(idx, (d, parent));
			if end_condition(idx, node_data) {
                break Some(idx)
            }
			if stack.len() == 0 {
				break None
			}
        };
		println!("Finished: {:?}", end_node);
		end_node.map(|mut idx| {
			let mut path = Vec::new();
		
			loop {
				path.push(idx);
				println!("{}", idx);
				match done[&idx].1 {
					Parent::Parent(p) => idx = p,
					Parent::Top => break,
				}
			}
			println!("Path: {:?}", path);
			path
		})
		
    }
}

pub enum Parent {
    Parent(usize),
    Top,
}
/// Where (distance, node_id)
pub struct GraphNode<T>(T, Vec<(usize, usize)>);

pub struct Stack<T> {
    pub elements: Vec<(T, usize)>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    pub fn push(&mut self, data: T, sorting_cond: usize) {
        // Bubble from the bottom to its position
        let mut index = 0;
        for (i, s) in self
            .elements
            .iter()
            .enumerate()
            .map(|(i, (_, s))| (i, s))
            .rev()
        {
            if s <= &sorting_cond {
                index = i;
                break;
            }
        }
        self.elements.insert(index, (data, sorting_cond))
    }
    pub fn pop(&mut self) -> (T, usize) {
        assert_ne!(self.elements.len(), 0, "Stack is empty");
        self.elements.remove(0)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn remove<F: Fn(&T, &usize) -> bool>(&mut self, f: F) -> Option<(T, usize)> {
		let mut index = None;
		for i in 0..self.elements.len() {
			let e = &self.elements[i];
			if f(&e.0, &e.1) {
				index = Some(i);
				break
			}
		}
		index.map(|i| self.elements.remove(i))
	}
}
