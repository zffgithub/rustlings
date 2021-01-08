// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint collections3` if you need
// hints.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 1);
    basket.insert(String::from("tofu"), 3);

    // TODO: Put more fruits in your basket here.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket
            .values()
            .sum::<u32>() >= 5);
    }

    /// https://blog.csdn.net/wowotuo/article/details/74779709
    #[test]
    fn hm() {
        use std::collections::HashMap;

        let mut contacts = HashMap::new();
        for i in 1..10 {
            contacts.insert(i, (i * 100).to_string());
        }
        contacts.insert(2, "222".to_string()); //overwrite <2, "200">
        contacts.remove(&3);

        contacts.entry(10).or_insert("7777".to_string()); // 如果contacts中没有key为10，则插入<10, 7777>；如果有key 10，则不插入，直接跳过；

        let len = contacts.len();
        let if_contain_key = contacts.contains_key(&1);
        let value = contacts.get(&6);

        let keys = contacts.keys();
        println!("keys:{:?}", keys);
        let values = contacts.values();
        println!("values :{:?}", values);

        let value_key = contacts.get(&1).unwrap().clone();
        println!("value_key :{:?}", value_key);

        for (key, value) in contacts {
            println!("key {}, value {}", key, value);
        }

        let mut hash_vec: HashMap<u32, Vec<&str>> = HashMap::new();
        hash_vec.insert(5, vec![]);
        //对于HashMap<u32, Vec<&str>>类型，要对最里面Vec<&str>进行增加元素的操作（假设里面有“5”key），可以：
        hash_vec.get_mut(&5).unwrap().push("value_5");

        let mut hash_map_tmp = HashMap::new();
        hash_map_tmp.insert(1, "value 1");//强制插入，已有key 1，则更新value为value 1；
        hash_map_tmp.entry(1).or_insert("value 2");//有key 1，不更新；没有key 1，插入 1， value 2

        //HashMap =>Vec
        let vec: Vec<(_, _)> = hash_map_tmp.into_iter().collect();
        println!("vec :{:?}", vec);

        let teams = vec![
            "AAA".to_string(),
            "BBB".to_string(),
            "CCC".to_string(),
        ];
        let socres = vec![10, 20, 30];
        let hash_map_scores: HashMap<_, _> = teams.iter().zip(socres.iter()).collect();

        {
            //统计单词个数
            let text = "aaa bbb ccc ddd aaa";
            let mut hmp = HashMap::new();
            for world in text.split_whitespace() {
                let count = hmp.entry(world).or_insert(0);
                *count += 1;
            }
        }


    }
}
