use std::collections::HashMap;

pub mod functions{
    pub fn median(x: &Vec<i32>) -> i32 {
        let mut v: Vec<i32> = vec![];
        v.clone_from(x);
        v.sort();
        v[v.len()/2]
    }
    
    pub fn mode(v: &Vec<i32>) -> i32 {
        let mut map = super::HashMap::new();
        for number in v {
            let count = map.entry(*number).or_insert(0);
            *count += 1;
        }

        let mut values: Vec<(&i32, &i32)> = map.iter().collect();
        values.sort_by(|a, b| b.1.cmp(a.1));

        *values[0].0
    }
}
