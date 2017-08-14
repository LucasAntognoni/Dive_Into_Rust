pub fn raindrops(drops: u32) -> String {
    
    let mut ret = String::new();

    if drops % 3 == 0{
        ret.push_str("Pling");
    }
    if drops % 5 == 0{
        ret.push_str("Plang");
    }
    if drops % 7 == 0{
        ret.push_str("Plong");
    }

    if ret == "" {
        return drops.to_string();
    }
    else {
        return ret;
    }
} 