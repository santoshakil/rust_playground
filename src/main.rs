fn main() {
    find_common_last();
    find_common_first();
}

fn find_common_first() {
    let s1 = "x/b/a/b/c.txt";
    let s2 = "x/b/a/x/b/c.txt";

    let mut common = String::new();

    for (_, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
        if c1 == c2 {
            common.push(c1);
        } else {
            break;
        }
    }

    println!("Common first part: {}", common);
}

fn find_common_last() {
    let s1 = "x/b/a/b/c.txt";
    let s2 = "x/b/a/x/b/c.txt";

    let mut common = String::new();

    for (_, (c1, c2)) in s1.chars().rev().zip(s2.chars().rev()).enumerate() {
        if c1 == c2 {
            common.push(c1);
        } else {
            break;
        }
    }

    common = common.chars().rev().collect::<String>();

    println!("Common last part: {}", common);
}
