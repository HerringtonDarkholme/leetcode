/*
/**
 * Definition for a binary tree node.
 */
class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
  }
}

function createBinaryTree(descriptions: number[][]): TreeNode | null {
  let map = new Map()
  let childs = new Set()
  for (let [p, c, isLeft] of descriptions) {
    let parent = upsertNode(p, map)
    let child = upsertNode(c, map)
    childs.add(c)
    if (isLeft === 1) {
      parent.left = child
    } else {
      parent.right = child
    }
  }
  for (let [i, n] of map) {
    if (childs.has(i)) {
      continue;
    }
    return n
  }
};

function upsertNode(i: number, map: Map<number, TreeNode>): TreeNode {
  let node = map.get(i)
  if (node) {
    return node
  }
  node = new TreeNode(i)
  map.set(i, node)
  return node
}
*/
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut splits = s.split(':');
        let cell1 = splits.next().unwrap();
        let cell2 = splits.next().unwrap();
        let (r1, c1) = parse(cell1);
        let (r2, c2) = parse(cell2);
        let mut ret = vec![];
        for c in c1..=c2 {
            for r in r1..=r2 {
                let r = r as char;
                let c = c as char;
                ret.push(format!("{}{}", c, r));
            }
        }
        ret
    }
}

fn parse(s: &str) -> (u8, u8) {
    let bytes = s.as_bytes();
    let r = bytes[1];
    let c = bytes[0];
    (r, c)
}

impl Solution {
    pub fn minimal_k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        nums.sort();
        let mut k = k as i64;
        let mut i = 1i64;
        let mut j = 0;
        let mut ret = 0i64;
        while k > 0 && j < nums.len() {
            let n = nums[j] as i64;
            if i < n {
                let d = k.min(n - i);
                k -= d;
                ret += (i + (i + d - 1)) * d / 2;
            }
            i = n + 1;
            j += 1;
        }
        if k > 0 {
            ret += (i + (i + k - 1)) * k / 2;
        }
        ret
    }
}

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }
        let mut ret = vec![];
        for mut n in nums.into_iter() {
            while let Some(last) = ret.pop() {
                let g = gcd(n, last);
                if g > 1 {
                    n = lcm(n, last, g);
                } else {
                    ret.push(last);
                    break;
                }
            }
            ret.push(n);
        }
        ret
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        return gcd(b, a);
    }
    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: i32, b: i32, gcd: i32) -> i32 {
    let n = (a as i64) * (b as i64);
    (n / gcd as i64) as i32
}
