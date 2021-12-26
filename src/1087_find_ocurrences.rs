impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut ret = vec![];
        let mut found = 0;
        let mut last_two = "";
        let mut last_one = "";
        for word in text.split(" ") {
            if last_two == first && last_one == second {
                ret.push(word.to_string());
            }
            last_two = last_one;
            last_one = word;
        }
        ret
    }
}
