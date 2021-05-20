impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let files = paths.into_iter().flat_map(parse).collect();
        collect(files)
    }
}

struct File {
    path: String,
    content: String,
}

fn collect(files: Vec<File>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();
    for f in files {
        let e = map.entry(f.content).or_insert(vec![]);
        e.push(f.path);
    }
    map.values().filter(|v| v.len() > 1).cloned().collect()
}

fn parse(s: String) -> Vec<File> {
    let mut parts = s.split_ascii_whitespace();
    let dir_path = parts.next().unwrap().to_owned();
    parts.map(|f| {
        let (fname, content) = extract(f);
        let path = dir_path.clone() + "/" + fname;
        File {
            path, content,
        }
    }).collect()
}

fn extract(f: &str) -> (&str, String) {
    let i = f.find('(').unwrap();
    let (a, b) = f.split_at(i);
    (a, b[..b.len() - 1].to_string())
}
