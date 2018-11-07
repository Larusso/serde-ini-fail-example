#[macro_use]
extern crate serde_derive;
extern crate serde_ini;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
enum Keys {
    Section1,
    Section2,
    Section3,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize,Debug)]
pub struct FieldData {
    title: String,
    description: String,
    url: String,
    size: u64,
    md5: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    section_1: Option<Box<FieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    section_2: Option<Box<FieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    section_3: Option<Box<FieldData>>,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Data2(HashMap<Keys,FieldData>);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Data3 {
    section_1: FieldData,

    #[serde(flatten)]
    components: HashMap<Keys, FieldData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = "
[Section1]
title=Just a Title
description=The first section of this document
url=http://custom.com/url
md5=1de0b7d9f705dbd0eab65cbf2cc693ee
size=886532131
someKey=2365775000
otherKey=2017.1.5f1
[Section2]
title=Just a Title
description=The second section of this document
url=http://custom.com/url
md5=1de0b7d9f705dbd0eab65cbf2cc693ee
size=886532131
someKey=2365775000
otherKey=2017.1.5f1
[Section3]
title=Just a Title
description=The third section of this document
url=http://custom.com/url
md5=1de0b7d9f705dbd0eab65cbf2cc693ee
size=886532131
someKey=2365775000
otherKey=2017.1.5f1
[Section4]
title=Just a Title
description=The fourth section of this document
url=http://custom.com/url
md5=1de0b7d9f705dbd0eab65cbf2cc693ee
size=886532131
someKey=2365775000
otherKey=2017.1.5f1";

    #[test]
    fn data() {
        let manifest:Data = serde_ini::from_str(TEST_INPUT).unwrap();
        println!("{:?}", manifest);
    }

    #[test]
    fn data_2() {
        let manifest:Data2 = serde_ini::from_str(TEST_INPUT).unwrap();
        println!("{:?}", manifest);
    }

    #[test]
    fn data_3() {
        let manifest:Data3 = serde_ini::from_str(TEST_INPUT).unwrap();
        println!("{:?}", manifest);
    }
}
