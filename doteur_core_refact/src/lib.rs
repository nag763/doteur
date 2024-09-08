use sql_parser::{ClauseKeyword, DdlKeyword, SqlAttribute, SqlDataType, SqlTable};

pub mod sql_parser;

fn split_tokens(input:  &str)  {

}

pub fn parse_table(input: &str) -> Option<SqlTable> {
    let mut splitted = input.split_ascii_whitespace();
    let first = splitted.next()?;
    let Ok(ddl) = DdlKeyword::try_from(first) else {
        return None;
    };
    if ddl != DdlKeyword::Create {
        return None;
    }
    for input in splitted.by_ref() {
        let Ok(keyword) = ClauseKeyword::try_from(input) else {
            continue;
        };
        if keyword == ClauseKeyword::Table {
            break;
        }
    }
    let keywords: Vec<&str> = splitted.by_ref().take_while(|e| *e != "(").collect();
    let table_name = if keywords.len() == 3 {
        keywords.get(2)?
    } else if keywords.len() == 1 {
        keywords.first()?
    } else {
        return None;
    };

    let content: Vec<&str> = splitted.take_while(|e| *e != ")").collect();

    let table_content = content.join(" ");

    Some(SqlTable {
        name: table_name.to_string(),
        content: table_content.to_string(),
    })
}

pub fn parse_attribute(input: &str) -> Option<SqlAttribute> {
    if input.is_empty() {
        return None;
    };
    let mut splitted = input.split_whitespace();
    let attr_name = splitted.next()?;
    let next_token = splitted.next()?;
    let Ok(attr_data_type) = SqlDataType::try_from(splitted.next()?) else {
        return None;
    };
    for (index, next_token) in splitted.by_ref().enumerate() {
        if index == 0 {
            if next_token == ClauseKeyword::Default {
                let next = splitted.next();
            }
        }
    }  
        
    
    None
}