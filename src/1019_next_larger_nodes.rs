impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut h = &head;        
        let mut ret = vec![];
        let mut stack: Vec<(usize, i32)> = vec![];
        while let Some(ref n) = h {
            while !stack.is_empty() && stack.last().unwrap().1 < n.val {
                let (pos, _) = stack.pop().unwrap();
                ret[pos] = n.val;
            }
            stack.push((ret.len(), n.val));
            ret.push(0);
            h = &n.next;
        }
        ret
    }
}
