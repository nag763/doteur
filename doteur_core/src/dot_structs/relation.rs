// Copyright ⓒ 2021-2024 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

const RELATE_TO_EMOJI: &str = "➡";

#[derive(Debug, Clone)]
enum OnDelete {
    SetNull,
    Cascade,
    Restrict,
    NoAction,
    SetDefault,
}

impl OnDelete {
    /// Returns the on delete type from a str
    fn from_str(relation_type: &str) -> OnDelete {
        match relation_type {
            "SET NULL" => OnDelete::SetNull,
            "CASCADE" => OnDelete::Cascade,
            "NO ACTION" => OnDelete::NoAction,
            "SET DEFAULT" => OnDelete::SetDefault,
            _ => OnDelete::Restrict,
        }
    }
    /// Return the arrow head
    fn get_dot_arrowhead(&self) -> &str {
        match self {
            OnDelete::SetNull => "odot",
            OnDelete::Cascade => "dot",
            _ => "normal",
        }
    }
}

#[derive(Debug, Clone)]
/// A relation is a link between a list of pair of keys
/// between two tables
pub struct Relation {
    /// Table that got the refering key
    origin_table: String,
    /// Table that got the refered key
    refered_table: String,
    /// List of keys refering each others
    keys: Vec<(String, String)>,
    /// Relations type
    relation_type: OnDelete,
}

impl Relation {
    /// A new relation
    pub fn new(origin_table: String, refered_table: String, relation_type: String) -> Relation {
        Relation {
            origin_table,
            refered_table,
            keys: Vec::new(),
            relation_type: OnDelete::from_str(relation_type.as_str()),
        }
    }

    /// A new relation with a single pair
    pub fn new_with_single_pair(
        origin_table: String,
        refered_table: String,
        origin_key: String,
        refered_key: String,
        relation_type: String,
    ) -> Relation {
        Relation {
            origin_table,
            refered_table,
            keys: vec![(origin_key, refered_key)],
            relation_type: OnDelete::from_str(relation_type.as_str()),
        }
    }

    /// Returns a copy of the pairs of key
    pub fn get_pairs_of_keys(&self) -> Vec<(String, String)> {
        self.keys.clone()
    }

    /// Returns the number of pairs of keys
    pub fn get_number_of_pairs_of_keys(&self) -> usize {
        self.keys.len()
    }

    /// Returns the refered table
    pub fn get_refered_table(&self) -> &str {
        self.refered_table.as_str()
    }

    /// Add a new pair of key to the relation
    pub fn push_pair_of_keys(&mut self, origin_key: String, refered_key: String) {
        self.keys.push((origin_key, refered_key));
    }

    /// Returns the relations as a dot output
    pub fn generate_dot_output(self, dark_mode: bool) -> String {
        let color_scheme: &str = match dark_mode {
            true => "fontcolor=white, color=white",
            false => "",
        };

        let mut ret: String = String::new();
        for key in self.keys {
            ret.push_str(format!("\t{0} -> {1} [label=<<I>{2} {3} {4}</I>>, arrowhead = \"{5}\", fontsize=\"12.0\", {6}]", self.origin_table, self.refered_table, key.0, RELATE_TO_EMOJI, key.1, self.relation_type.get_dot_arrowhead(), color_scheme).as_str());
        }
        ret
    }
}
