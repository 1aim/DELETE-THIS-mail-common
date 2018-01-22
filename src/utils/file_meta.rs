
use chrono::DateTime;
use chrono::Utc;


#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct FileMeta {
    pub file_name: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub read_date: Option<DateTime<Utc>>,
    pub size: Option<usize>
}