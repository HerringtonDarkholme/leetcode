impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let mut num = num as usize;
        let seps = ["Thousand", "Million", "Billion"];
        let nums = [
            "Zero", "One", "Two", "Three", "Four",
            "Five", "Six", "Seven", "Eight", "Nine",
            "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen",
            "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        ];
        let tens = [
            "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy",
            "Eighty", "Ninety"
        ];
        if num == 0 {
            return "Zero".into()
        }
        let mut s = vec![];
        let mut sep = 0;
        while num != 0 {
            let n = num % 100;
            if n == 0 {
                
            } else if n < 20 {
                s.push(nums[n]);
            } else if n < 100 {
                let ten = n / 10 - 2;
                if n % 10 != 0 {
                    s.push(nums[n % 10]);
                }
                s.push(tens[ten]);
            }
            let hundred = num / 100 % 10;
            if hundred > 0 {
                s.push("Hundred");
                s.push(nums[hundred]);
            } 
            if num >= 1000 {
                if num / 1000 % 1000 != 0 {
                    s.push(seps[sep]);
                }
                sep += 1;
            }
            num /= 1000;
        }
        s.reverse();
        s.join(" ")
    }
}
