use std::collections::HashSet;
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut context = build_context(n, edges);
        let mut infos = vec![Info::new(); n as usize];
        traverse_graph(&mut context, &mut infos);
        infos.into_iter().map(|i| i.sum as i32).collect()
    }
}
struct Context {
    graph: Vec<HashSet<usize>>,
    leaves: Vec<usize>,
}
fn build_context(n: i32, edges: Vec<Vec<i32>>) -> Context {
    let mut graph = vec![HashSet::new(); n as usize];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].insert(b);
        graph[b].insert(a);
    }
    let mut leaves = vec![];
    for i in 0..(n as usize) {
        if graph[i].len() == 1 {
            leaves.push(i);
        }
    }
    Context {
        graph,
        leaves,
    }
}
fn traverse_graph(context: &mut Context, infos: &mut [Info])  {
    // pop up leaf
    let Some((leaf, other)) = pop_up_leaf(context) else {
        return;
    };
    // collapsed into another node
    collapse_node(leaf, other, context, infos);
    // recurse
    traverse_graph(context, infos);
    // expand_node
    expand_node(leaf, other, context, infos);
}
fn pop_up_leaf(context: &mut Context) -> Option<(usize, usize)> {
    let leaf = context.leaves.pop()?;
    let other = context.graph[leaf].drain().next()?;
    context.graph[other].remove(&leaf);
    if context.graph[other].len() == 1 {
        context.leaves.push(other);
    }
    Some((leaf, other))
}
fn collapse_node(leaf: usize, into: usize, context: &mut Context, infos: &mut [Info]) {
    let my_sum = infos[leaf].sum;
    let my_cnt = infos[leaf].cnt;
    infos[into].sum += my_sum + my_cnt;
    infos[into].cnt += my_cnt;
}
fn expand_node(leaf: usize, from: usize, context: &mut Context, infos: &mut [Info]) {
    infos[leaf].sum = infos[from].sum + infos[from].cnt - infos[leaf].cnt * 2;
    infos[leaf].cnt = infos[from].cnt;
}
#[derive(Clone)]
struct Info {
    sum: usize,
    cnt: usize,
}
impl Info {
    fn new() -> Self {
        Self {
            sum: 0,
            cnt: 1,
        }
    }
}
// collapse: (my_sum + your_sum + your_node, your_node + my_node)
// expand: 
//  exclude_sum = your_sum - my_sum - my_node
//  exclude_node = total_node - my_node
//  final_sum = my_sum + exclude_sum + exclude_node 
//            = your_sum - my_node + your_node - my_node
//            = your_sum + total - 2 * my_node
// 0: (8, 6)
// 2: (6, 6)
