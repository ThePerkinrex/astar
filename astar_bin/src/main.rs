use astar_lib::Graph;

fn main() {
    println!("Hello, world!");
    let mut g = Graph::new();
    let start = g.add_node('S');
    g.add_child(start, 7, 'A');
    let b = g.add_child(start, 5, 'B');
    let c = g.add_child(start, 2, 'C');
    let end = g.add_child(b, 2, 'E');
    g.add_children_to_node(c, end, 7);
    g.add_children_to_node(c, b, 2);
    /* 
        (S)
       / | \
      A  B<-C
          \ /
          (E)
        
        
        S>A: 7
        S>B: 5
        S>C: 2

        B>E: 2

        C>B: 2
        C>E: 7
    */
    println!("{}", g.size());
    g.astar_path_find(start, |e, _| e == end, |_, _| 0);
}
