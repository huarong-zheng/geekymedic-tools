use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::btree_map::BTreeMap;
use std::str::pattern::Pattern;

use serde_json::Result;
use serde_json::Value;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DivisonCode {
    pub id: i32,
    pub name: String,
    #[serde(rename = "child", skip_serializing_if = "Vec::is_empty")]
    pub lists: Vec<DivisonCode>,
}


fn parse_divison_code(file: &str) -> std::io::Result<BTreeMap<i32, DivisonCode>> {
    let mut special_codes = vec![
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
    special_codes.iter_mut().for_each(|divison_code|{
        set.insert(divison_code.id, divison_code.clone());
    });

    // 直辖市
    let inner_province = vec!["11".to_string(), "12".to_string(), "31".to_string(), "50".to_string()];

    let mut fp = File::open(file)?;

    let f = BufReader::new(fp);
    for line in f.lines() {
        line.map(|line| {
            let lines: Vec<String> = line.trim().split("	").map(|line| { line.to_string() }).collect();
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
//                        println!("第二ceng {}", id);
                        // 第二层
                        let parent_id: i32 = (province.clone() + &"0000").parse().unwrap();
                        let mut parent = set.get_mut(&parent_id).unwrap();
                        parent.lists.push(DivisonCode {
                            id: id.parse().unwrap(),
                            name: name.clone(),
                            lists: vec![],
                        });
                    } else {
                        let curr_id: i32 = id.parse().unwrap();
                        let grandfather_id = (province.clone() + &"0000").parse().unwrap();
                        let grandfather = set.get_mut(&grandfather_id).unwrap();
                        let parent_id = (province.clone() + &city + "00").parse().unwrap();
//                        println!("--> {}, {}", grandfather_id, parent_id);
                        match grandfather.lists.binary_search_by_key(&parent_id, |e| e.id) {
                            Ok(idx) => {
                                let parent = grandfather.lists.get_mut(idx as usize).unwrap();
                                parent.lists.push(DivisonCode {
                                    id: curr_id,
                                    name: name.clone(),
                                    lists: vec![],
                                });
                            }
                            _ => {
                                grandfather.lists.push(DivisonCode {
                                    id: curr_id,
                                    name: name.clone(),
                                    lists: vec![],
                                });
                            }
                        }
                    }
                }
            }
        });
    }
    Ok(set)
}


#[cfg(test)]
mod tests {
    use crate::division_code::{parse_divison_code, DivisonCode};
    use std::fs::File;
    use std::fs;
    use std::io::Write;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let set = parse_divison_code("id").unwrap();
        let set = set.iter().map(|(_, value)| value).collect::<Vec<&DivisonCode>>();
        let s = serde_json::to_string(&set).unwrap();
//        print!("{}", s);
        fs::write("/tmp/province", s.as_bytes()).unwrap();
//        set.iter().for_each(|(_, item)| {
//            item.lists.iter().for_each(|e| {
//                e.lists.iter().for_each(|e| {
//                    println!("{}, {}, {}", e.id, e.name, e.lists.len());
//                });
//            })
//        });
    }
}
