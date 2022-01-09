#[derive(Debug, Clone)]
enum OnDelete {
    SetNull,
    Cascade,
    Restrict,
    NoAction,
    SetDefault,
}

impl OnDelete {
    fn from_str(relation_type: &str) -> OnDelete {
        match relation_type {
            "SET NULL" => OnDelete::SetNull,
            "CASCADE" => OnDelete::Cascade,
            "NO ACTION" => OnDelete::NoAction,
            "SET DEFAULT" => OnDelete::SetDefault,
            _ => OnDelete::Restrict,
        }
    }

    fn get_dot_arrowhead(&self) -> &str {
        match self {
            OnDelete::SetNull => "odot",
            OnDelete::Cascade => "dot",
            _ => "normal",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Relation {
    origin_table: String,
    refered_table: String,
    keys: Vec<(String, String)>,
    relation_type: OnDelete,
}

impl Relation {
    pub fn new(origin_table: String, refered_table: String, relation_type: String) -> Relation {
        Relation {
            origin_table,
            refered_table,
            keys: Vec::new(),
            relation_type: OnDelete::from_str(relation_type.as_str()),
        }
    }

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

    pub fn get_pairs_of_keys(&self) -> Vec<(String, String)> {
        self.keys.clone()
    }

    pub fn get_refered_table(&self) -> &str {
        self.refered_table.as_str()
    }

    pub fn push_pair_of_keys(&mut self, origin_key: String, refered_key: String) {
        self.keys.push((origin_key, refered_key));
    }

    pub fn generate_dot_output(self, dark_mode: bool) -> String {
        let color_scheme: &str = match dark_mode {
            true => "fontcolor=white, color=white",
            false => "",
        };

        let refer: &str = match cfg!(unix) {
            true => "\u{27A1}",
            _ => "refers",
        };

        let arrowhead: &str = match self.relation_type.get_dot_arrowhead() {
            "SET NULL" => "odot",
            "CASCADE" => "dot",
            _ => "normal",
        };
        let mut ret: String = String::new();
        for key in self.keys {
            ret.push_str(format!("\t{0} -> {1} [label=<<I>{2} {3} {4}</I>>, arrowhead = \"{5}\", fontsize=\"12.0\", {6}]", self.origin_table, self.refered_table, key.0, refer, key.1, arrowhead, color_scheme).as_str());
        }
        ret
    }
}
