impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by_key(|v| v[1]);
        box_types.reverse();
        let mut r = 0;
        for bt in box_types {
            if truck_size <= 0 {
                break;
            }
            let num = bt[0];
            let unit = bt[1];
            let real_num = num.min(truck_size);
            r += unit * real_num;
            truck_size -= real_num;
        }
        r
    }
}

/*
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|v| -v[1]);
        let mut ret = 0;
        for box_type in box_types.into_iter() {
            let loaded_count = box_type[0].min(truck_size);
            ret += loaded_count * box_type[1];
            truck_size -= loaded_count;
        }
        ret
    }
}
*/
