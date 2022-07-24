#[macro_use]
mod util;
#[path = "./recursive_remove_adjacent_duplicates.rs"]
pub mod remove_adj;

fn main() {}

#[test]
fn test() {
    // assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
    // assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
}

struct Solution;


impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut occurrence = [false; 26];
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            if occurrence[i] {
                return b as char;
            } else {
                occurrence[i] = true;
            }
        }
        'X' // impossible
    }
}

// use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut first_row = HashMap::new();
        let mut first_col = HashMap::new();
        for i in 0..grid.len() {
            let r = grid[0][i];
            first_row.entry(r).or_insert_with(|| vec![]).push(i);
        }
        for i in 0..grid.len() {
            let c = grid[i][0];
            first_col.entry(c).or_insert_with(|| vec![]).push(i);
        }
        let mut count = 0;
        for i in 0..grid.len() {
            let r = grid[0][i];
            if let Some(rows) = first_col.get(&r) {
                for &row in rows {
                    let mut mismatched = false;
                    for j in 0..grid.len() {
                        if grid[row][j] != grid[j][i] {
                            mismatched = true;
                            break;
                        }
                    }
                    if !mismatched {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

use std::collections::{HashMap, BTreeSet, BTreeMap};

struct FoodRatings {
    cuisine_map: HashMap<String, BTreeMap<i32, BTreeSet<String>>>,
    food_ratings: HashMap<String, i32>,
    food_map: HashMap<String, String>,
}


/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_map = HashMap::new();
        let mut cuisine_map = HashMap::new();
        let mut food_ratings = HashMap::new();

        for i in 0..foods.len() {
            let food = &foods[i];
            let cusine = &cuisines[i];
            let rating = ratings[i];
            food_map.insert(food.clone(), cusine.clone());
            cuisine_map.entry(cusine.clone()).or_insert_with(|| BTreeMap::new()).entry(rating).or_insert_with(|| BTreeSet::new()).insert(food.clone());
            food_ratings.insert(food.clone(), rating);
        }

        Self {
            cuisine_map,
            food_ratings,
            food_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = &self.food_map[&food];
        let mut ranking = self.cuisine_map.get_mut(cuisine).unwrap();
        let rating = self.food_ratings[&food];
        ranking.get_mut(&rating).unwrap().remove(&food);
        ranking.entry(new_rating).or_insert_with(|| BTreeSet::new()).insert(food.clone());
        self.food_ratings.insert(food, new_rating);
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let mut ranking = self.cuisine_map.get_mut(&cuisine).unwrap();
        let (&rating, foods) = ranking.iter_mut().rev().next().unwrap();
        if let Some(food) = foods.iter().next() {
            return food.into()
        }
        drop(foods);
        ranking.remove(&rating);
        self.highest_rated(cuisine)
    }
}

// [null, "kimchi", "ramen", null, "sushi", null, "ramen"]


// [1,2,3]
use std::collections::HashSet;
impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let nums: HashSet<_> = nums.into_iter().collect();
        let total = nums.len();
        let k = k as u32;
        let mut one_count_nums = vec![vec![]; 33];
        for n in nums {
            let ones = n.count_ones() as usize;
            one_count_nums[ones].push(n);
        }
        let mut count = 0;
        for i in 0..33 {
            let nums = &one_count_nums[i];
            if i >= k as usize {
                count +=
                    nums.len() as i64 +  // self
                    (nums.len() * (total - 1) * 2) as i64; // other
                continue;
            }
            let min = (if k >= 32 { k - 32 } else { 0 }) as usize;
            let min = if min >= i { min - i } else { 0 };
            for j in (min.max(i))..(k as usize).min(33) {
                for &other in &one_count_nums[j as usize] {
                    for &n in nums {
                        if (n & other).count_ones() + (n | other).count_ones() >= k {
                            count += if n == other { 1 } else { 2 };
                        }
                    }
                }
            }
        }
        count
    }
}
// 3 bits => and 3 , or 32
