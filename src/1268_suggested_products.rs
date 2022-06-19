impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        let mut products = &products[..];
        let mut ret = vec![];
        let mut query = String::new();
        for c in search_word.chars() {
            query.push(c);
            let start = bsearch(&query, products);
            let mut sub_ret = vec![];
            for i in start..products.len().min(start + 3) {
                if products[i].len() < query.len() || !products[i].starts_with(&query){
                    break;
                }
                sub_ret.push(products[i].clone());
            }
            ret.push(sub_ret);
            products = &products[start..];
        }
        ret
    }
}

fn bsearch(need: &String, arr: &[String]) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if &arr[mid] < need {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
