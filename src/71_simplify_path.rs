impl Solution {
    pub fn simplify_path(path: String) -> String {
        let components = path.split('/');
        let mut ret = vec![];
        for comp in components {
            if comp == "" || comp == "." {
                continue;
            } else if comp == ".." {
                ret.pop();
            } else {
                ret.push(comp);
            }
        }
        format!("/{}", ret.join("/"))
    }
}
