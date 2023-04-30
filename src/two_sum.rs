pub fn two_sum_setup(){
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let r = two_sum(nums, target);

    let l = r.len();
    for i in 0..l {
        let x = r[i];
        println!("{x}");
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let l = nums.len();

    for i in 0..l {
        for j in i+1..l { 
            if nums[i]+nums[j] == target {
                let a:i32 = i as i32;
                let b:i32 = j as i32;
                return vec![a,b];
            }
        }
    }
    vec![]
}