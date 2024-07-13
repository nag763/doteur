use keyword_error::KeywordError;

pub mod keyword_error;

pub struct SqlTable {
    pub name: String,
    pub content: String,
}

#[derive(Eq, PartialEq)]
pub enum DdlKeyword {
    Add,
    Alter,
    Create,
    Drop,
    Truncate,
}

pub enum SqlDataType {
    Bigint,
    Binary,
    Bit,
    Blob,
    Boolean,
    Char,
    Date,
    Datetime,
    Decimal,
    Double,
    Float,
    Int,
    Integer,
    Numeric,
    Real,
    Smallint,
    Time,
    Timestamp,
    Varchar,
}

#[derive(Eq, PartialEq)]
pub enum ClauseKeyword {
    And,
    As,
    Asc,
    Between,
    By,
    Desc,
    Distinct,
    Exists,
    Foreign,
    From,
    Group,
    Having,
    In,
    Index,
    Into,
    Join,
    Key,
    Like,
    Not,
    Null,
    Or,
    Order,
    Primary,
    References,
    Set,
    Table,
    Union,
    Unique,
    Values,
    Where,
}

impl TryFrom<String> for DdlKeyword {
    type Error = KeywordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        DdlKeyword::try_from(value.as_str())
    }
}

impl TryFrom<&str> for DdlKeyword {
    type Error = KeywordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "add" => Ok(DdlKeyword::Add),
            "alter" => Ok(DdlKeyword::Alter),
            "create" => Ok(DdlKeyword::Create),
            "drop" => Ok(DdlKeyword::Drop),
            "truncate" => Ok(DdlKeyword::Truncate),
            _ => Err(KeywordError(value.to_string())),
        }
    }
}

impl TryFrom<String> for SqlDataType {
    type Error = KeywordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SqlDataType::try_from(value.as_str())
    }
}

impl TryFrom<&str> for SqlDataType {
    type Error = KeywordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "bigint" => Ok(SqlDataType::Bigint),
            "binary" => Ok(SqlDataType::Binary),
            "bit" => Ok(SqlDataType::Bit),
            "blob" => Ok(SqlDataType::Blob),
            "boolean" => Ok(SqlDataType::Boolean),
            "char" => Ok(SqlDataType::Char),
            "date" => Ok(SqlDataType::Date),
            "datetime" => Ok(SqlDataType::Datetime),
            "decimal" => Ok(SqlDataType::Decimal),
            "double" => Ok(SqlDataType::Double),
            "float" => Ok(SqlDataType::Float),
            "int" => Ok(SqlDataType::Int),
            "integer" => Ok(SqlDataType::Integer),
            "numeric" => Ok(SqlDataType::Numeric),
            "real" => Ok(SqlDataType::Real),
            "smallint" => Ok(SqlDataType::Smallint),
            "time" => Ok(SqlDataType::Time),
            "timestamp" => Ok(SqlDataType::Timestamp),
            "varchar" => Ok(SqlDataType::Varchar),
            _ => Err(KeywordError(value.to_string())),
        }
    }
}

impl TryFrom<String> for ClauseKeyword {
    type Error = KeywordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ClauseKeyword::try_from(value.as_str())
    }
}

impl TryFrom<&str> for ClauseKeyword {
    type Error = KeywordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "and" => Ok(ClauseKeyword::And),
            "as" => Ok(ClauseKeyword::As),
            "asc" => Ok(ClauseKeyword::Asc),
            "between" => Ok(ClauseKeyword::Between),
            "by" => Ok(ClauseKeyword::By),
            "desc" => Ok(ClauseKeyword::Desc),
            "distinct" => Ok(ClauseKeyword::Distinct),
            "exists" => Ok(ClauseKeyword::Exists),
            "foreign" => Ok(ClauseKeyword::Foreign),
            "from" => Ok(ClauseKeyword::From),
            "group" => Ok(ClauseKeyword::Group),
            "having" => Ok(ClauseKeyword::Having),
            "in" => Ok(ClauseKeyword::In),
            "index" => Ok(ClauseKeyword::Index),
            "into" => Ok(ClauseKeyword::Into),
            "join" => Ok(ClauseKeyword::Join),
            "key" => Ok(ClauseKeyword::Key),
            "like" => Ok(ClauseKeyword::Like),
            "not" => Ok(ClauseKeyword::Not),
            "null" => Ok(ClauseKeyword::Null),
            "or" => Ok(ClauseKeyword::Or),
            "order" => Ok(ClauseKeyword::Order),
            "primary" => Ok(ClauseKeyword::Primary),
            "references" => Ok(ClauseKeyword::References),
            "set" => Ok(ClauseKeyword::Set),
            "table" => Ok(ClauseKeyword::Table),
            "union" => Ok(ClauseKeyword::Union),
            "unique" => Ok(ClauseKeyword::Unique),
            "values" => Ok(ClauseKeyword::Values),
            "where" => Ok(ClauseKeyword::Where),
            _ => Err(KeywordError(value.to_string())),
        }
    }
}
