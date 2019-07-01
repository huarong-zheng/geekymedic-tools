use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::btree_map::BTreeMap;
use std::str::pattern::Pattern;

pub struct DivisonCode {
    pub id: i32,
    pub name: String,
    pub lists: Vec<DivisonCode>,
}

fn parse_divison_code(file: &str) -> std::io::Result<BTreeMap<i32, DivisonCode>> {
    let special_codes = vec![
        DivisonCode {
            id: 710000,
            name: "台湾省".to_string(),
            lists: vec![],
        },
        DivisonCode {
            id: 810000,
            name: "香港特别行政区".to_string(),
            lists: vec![],
        },
        DivisonCode {
            id: 820000,
            name: "澳门特别行政区".to_string(),
            lists: vec![],
        },
    ];

    let mut set = BTreeMap::new();

    // 直辖市
    let inner_province = vec!["11".to_string(), "12".to_string(), "31".to_string(), "50".to_string()];

    let mut fp = File::open(file)?;

    let f = BufReader::new(fp);
    for line in f.lines() {
        line.map(|line| {
            let lines: Vec<String> = line.split("	").map(|line| { line.to_string() }).collect();
            let (id, name) = (&lines[0], &lines[1]);
            let include = inner_province.iter().any(|item| id.as_str().starts_with(item));
            let (province, city, county) = (String::from(&id[0..2]), String::from(&id[2..4]), String::from(&id[4..6]));
            match include {
                true => {
                    if city + &county == "0000" {
                        set.entry(id.parse().unwrap()).or_insert(DivisonCode {
                            id: id.parse().unwrap(),
                            name: name.clone(),
                            lists: vec![],
                        });
                    } else {
                        let curr_id = (province + "0000").parse().unwrap();
                        set.get_mut(&curr_id).map(|mut key| {
                            key.lists.push(DivisonCode {
                                id: id.parse().unwrap(),
                                name: name.clone(),
                                lists: vec![],
                            })
                        });
                    }
                }
                false => {
                    if city.clone() + &county == "0000" {
                        let curr_id = (province + &"0000").parse().unwrap();
                        set.entry(curr_id).or_insert(DivisonCode {
                            id: id.parse().unwrap(),
                            name: name.clone(),
                            lists: vec![],
                        });
                    } else if &county == "00" {
                        println!("00000");
                        // 第二层
                        let curr_id = (province + &city.clone() + &"00").parse().unwrap();
                        set.entry(curr_id).or_insert(DivisonCode {
                                id: id.parse().unwrap(),
                                name: name.clone(),
                                lists: vec![], });
                    }
                }
            }
        });
    }
    Ok(set)
}


#[cfg(test)]
mod tests {
    use crate::division_code::parse_divison_code;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let set = parse_divison_code("/Users/rg/id").unwrap();
        println!("{}", set.len());
        set.iter().for_each(|(_, item)| {
            item.lists.iter().for_each(|e| {
                e.lists.iter().for_each(|e|{
                    println!("{}, {}, {}", e.id, e.name, e.lists.len());
                });
            })
        });
    }
}
