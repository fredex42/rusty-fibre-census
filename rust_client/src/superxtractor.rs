use regex::{Regex, Captures};
use std::{str, collections::HashMap, vec::IntoIter};

/**
 * SuperXtractor will match a block of text for a defined set of regexes, which each contain a single capture pattern paired with a field label.
 * It will return a Vec of Matches, each match consisting of the field name paired with the extracted text
 */
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
    /**
     * Creates a new SuperXtractor based on a vector iterator of field-regex pairs.
     * Example:
     *   let the_knowledge = vec![
     *     ("my_field", Regex::new(r"))
     *   ]
     *   let xt = SuperXtractor::new()
     */
    pub fn new(refs: IntoIter<(&str, Regex)>) -> SuperXtractor {
        return SuperXtractor::new_from_hash(HashMap::from_iter(refs));
    }

    pub fn new_from_hash(refs: HashMap<&str, Regex>) -> SuperXtractor {
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

    /**
     * Runs the given SuperXtractor across a block of text, as given by the String parameter
     */
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
        let the_knowledge = vec![
            ("model", Regex::new(r"^\s+Model Name: (.*)\s*$").unwrap()),
            ("uuid", Regex::new(r"^\s+Hardware UUID: (.*)\s*$").unwrap()),
        ].into_iter();

        let xt = SuperXtractor::new(the_knowledge);

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