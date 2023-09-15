use regex::{Regex, Captures};
use std::{str, collections::HashMap};

pub struct SuperXtractor<'a> {
    refs: HashMap<&'a str, Regex>,
}

//derive(debug) tells the compiler to give us default behaviour for the Debug trait which allows usage of "{:#?}" in println for pretty-print
#[derive(Debug)]
pub struct Match {
    pub field: String,
    pub text: String
}


impl<'a> SuperXtractor<'_>  {
    pub fn new(refs: HashMap<&str, Regex>) -> SuperXtractor {
        return SuperXtractor {
            refs: refs.clone(),
        }
    }
    fn try_matches(&self, on: &str) -> Option<Match> {
        for (id, r) in &self.refs {
            let captures = r.captures_len();
            return match r.captures(on) {
                Some(m)=> {
                    let (text, groups):(&str, [&str; 1]) = m.extract();
                    Some(Match {
                        field: id.to_string(),
                        text: groups[0].to_string()
                    })
                },
                None=>continue
            }
        }
        return None;
    }

    pub fn execute_by_line(&self, on: String) -> Vec<Match> {
        let mut results:Vec<Match> = Vec::new();

        for line in str::split(&on, "\n") {
            match self.try_matches(line) {
                Some(m)=> {
                    results.push(m);
                }
                None=> ()
            }
        }
        return results;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::array::IntoIter;
    use super::SuperXtractor;
    use regex::Regex;

    #[test]
    fn it_works() {
        let the_knowledge = IntoIter::new([
            ("model", Regex::new(r"^\s+Model Name: (.*)\s*$").unwrap()),
            ("uuid", Regex::new(r"^\s+Hardware UUID: (.*)\s*$").unwrap()),
        ]);

        let xt = SuperXtractor::new(HashMap::from_iter(the_knowledge));

        let test_content = "Hardware:

        Hardware Overview:
    
          Model Name: MacBook Pro
          Model Identifier: MacBookPro18,2
          Model Number: SDHJSAF/A
          Chip: Apple M1 Max
          Total Number of Cores: 10 (8 performance and 2 efficiency)
          Memory: 32 GB
          System Firmware Version: 8422.121.1
          OS Loader Version: 8422.121.1
          Serial Number (system): GKJSDJHAS
          Hardware UUID: FakeUUID
          Provisioning UDID: AnotherFakeUUID
          Activation Lock Status: Secret".to_string();

        let result = xt.execute_by_line(test_content);
        // print!("{:#?}", result);
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].field, "model");
        assert_eq!(result[0].text, "MacBook Pro");
        assert_eq!(result[1].field, "uuid");
        assert_eq!(result[1].text, "FakeUUID");
    }
}