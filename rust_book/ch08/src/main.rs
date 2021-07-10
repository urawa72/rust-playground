use std::collections::HashMap;

fn main() {
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("third element is {}", third);
        let third: Option<&i32> = v.get(2);
        match third {
            None => println!("None!"),
            Some(e) => println!("Some {}!", e),
        }
        let third: Option<&i32> = v.get(100);
        match third {
            None => println!("None!"),
            Some(e) => println!("Some {}!", e),
        }
    }

    {
        let v = vec![100, 32, 58];
        for i in &v {
            println!("{}", i);
        }
        let mut v = vec![100, 32, 58];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let data = "initial content";
        let s = data.to_string();
        println!("{}", s);
        let mut s = "initial content".to_string();
        s.push_str(" bar");
        println!("{}", s);
    }

    {
        let mut s1 = "foo".to_string();
        let s2 = "bar".to_string();
        s1.push_str(&s2);
        println!("s2 is {}", s2);
    }

    {
        let s1 = "tic".to_string();
        let s2 = "tac".to_string();
        let s3 = "toe".to_string();
        // 参照外し型強制 &String -> &str
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{}", s);
        // let s = format!("{}-{}-{}", s1, s2, s3);
        // println!("{}", s);
    }

    {
        let s1 = "hello".to_string();
        let h = &s1[0..2];
        println!("{}", h);
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:?}", scores);
    }

    {
        let teams = vec!["Blue".to_string(), "Yellow".to_string()];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        let key = "Favorite color".to_string();
        let value = "Blue".to_string();
        let mut map = HashMap::new();
        map.insert(key, value);
        println!("{:?}", map);
        let key = "Favorite color".to_string();
        let m = map.get(&key);
        println!("m is {}", m.unwrap());
    }

    {
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.entry("Yellow".to_string()).or_insert(50);
        scores.entry("Blue".to_string()).or_insert(100);
        println!("{:?}", scores);
    }

    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }

    {
        let text = "helloworld".to_string();
        let mut map = HashMap::new();
        for c in text.chars() {
            let cnt = map.entry(c).or_insert(0);
            *cnt += 1;
        }
        println!("{:?}", map);
    }
}
