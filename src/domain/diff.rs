use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    NullValue,
    NotAllowedValue,
    NotInInterval,
    UnmatchedRegex,
    NotABool,
    NotAnInt,
    NotAFloat,
    NotAString,
    NotAnArray,
    NotAnObject,
    MissingProp,
    UnknownProp,
}

#[derive(Debug)]
pub struct Diff {
    root_key: String,
    diffs: HashMap<String, Vec<Reason>>,
}

impl Diff {
    pub fn new(root_key: String) -> Diff {
        Diff {
            root_key,
            diffs: HashMap::new(),
        }
    }

    pub fn root_key(&self) -> &str {
        &self.root_key
    }

    pub fn diffs(&self) -> &HashMap<String, Vec<Reason>> {
        &self.diffs
    }

    pub fn is_empty(&self) -> bool {
        self.diffs.is_empty()
    }

    pub fn add(&mut self, reason: Reason, key: Option<String>) {
        let key = if let Some(key) = key {
            format!("{}.{}", self.root_key, key)
        } else {
            self.root_key.to_string()
        };

        if let Some(diff) = self.diffs.get_mut(&key) {
            diff.push(reason);
        } else {
            self.diffs.insert(key, vec![reason]);
        }
    }

    pub fn merge(&mut self, diff: Diff) {
        for (key, value) in diff.diffs.into_iter() {
            self.diffs
                .insert(format!("{}.{}", self.root_key, key), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_and_sub_affixes() {
        let mut diff = Diff::new("$".to_string());
        diff.add(Reason::NotAString, Some("env".to_string()));
        diff.add(Reason::NotInInterval, Some("port".to_string()));

        let mut subdiff = Diff::new("database_urls".to_string());
        subdiff.add(Reason::UnmatchedRegex, Some("host".to_string()));

        diff.merge(subdiff);

        assert_eq!(diff.root_key(), "$");

        let diffs = diff.diffs();
        assert_eq!(diffs.len(), 3);
        assert_eq!(diffs.get("$.env").unwrap(), &vec![Reason::NotAString]);
        assert_eq!(diffs.get("$.port").unwrap(), &vec![Reason::NotInInterval]);
        assert_eq!(
            diffs.get("$.database_urls.host").unwrap(),
            &vec![Reason::UnmatchedRegex]
        );
    }
}
