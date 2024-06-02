// Copyright ⓒ 2021-2024 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

use std::fmt;

use super::dot_table::DotTable;
use super::relation::Relation;

/// A DotFile object is used to render the compiled schema in argument.
pub struct DotFile {
    /// The header of the dot file
    header: String,
    /// The tables to include in the file
    dot_tables: Vec<DotTable>,
    /// The relations to include in the file
    relations: Vec<Relation>,
    /// The footer of the file
    footer: String,
    /// Define if the graph has to be in dark mode
    dark_mode: bool,
}

impl fmt::Display for DotFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}\n{1}\n{2}\n\n{3}",
            self.header,
            self.dot_tables
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            self.relations
                .iter()
                .map(|r| r.clone().generate_dot_output(self.dark_mode))
                .collect::<Vec<String>>()
                .join("\n"),
            self.footer
        )
    }
}

impl DotFile {
    /// Creates a new dotfile with the given name
    pub fn new(legend: bool, dark_mode: bool) -> DotFile {
        DotFile {
            header: init_dot(legend, dark_mode),
            dot_tables: Vec::new(),
            relations: Vec::new(),
            footer: String::from("}"),
            dark_mode,
        }
    }

    /// Adds a table to the DotFile
    pub fn add_table(&mut self, table: DotTable) {
        self.dot_tables.push(table);
    }

    /// Add a relation to the DotFile
    pub fn add_relation(&mut self, relation: Relation) {
        self.relations.push(relation);
    }
}

/// Creates the dot file header
fn init_dot(legend: bool, dark_mode: bool) -> String {
    let bg_color: &str = match dark_mode {
        true => "bgcolor= black;",
        false => "",
    };

    let edge_color_scheme = match dark_mode {
        true => "white",
        false => "black",
    };

    let dot_legend: String = match legend {
        false => String::new(),
        true => format!(
            "
    {{
        labelloc=\"b\"
        labeljust=\"r\"
        rank=sink
        rankdir=LR
        d0 [style = invis];
        d1 [style = invis];
        p0 [style = invis];
        p1 [style = invis];
        s0 [style = invis];
        s1 [style = invis];
    }}
    d0 -> d1 [label=composition arrowhead=dot color={0} fontcolor={0}]
    p0 -> p1 [label=aggregation arrowhead=odot color={0} fontcolor={0}]
    s0 -> s1 [label=association color={0} fontcolor={0}]",
            edge_color_scheme
        ),
    };

    format!(
        "//This file has been generated with doteur, enjoy!
digraph doteur {{\n

    {0}

    node [\n
        shape = \"plaintext\"
    ]\n\n

    {1}",
        bg_color, dot_legend
    )
}
