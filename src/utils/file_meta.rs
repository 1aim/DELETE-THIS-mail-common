
use chrono::DateTime;
use chrono::Utc;

//TODO potentially move this to `mail-headers`

/// A struct representing common file metadata.
///
/// This is used by e.g. attachments, when attaching
/// a file (or embedding an image). Through it's usage
/// is optional.
///
/// # Stability Note
///
/// This is likely to move to an different place at
/// some point, potentially in a different `mail-*`
/// crate.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct FileMeta {
    /// The file name.
    ///
    /// Note that this utility is limited to utf-8 file names.
    /// This is normally used when downloading a attachment to
    /// choose the default file name.
    pub file_name: Option<String>,

    /// The creation date of the file (in utc).
    pub creation_date: Option<DateTime<Utc>>,

    /// The last modification date of the file (in utc).
    pub modification_date: Option<DateTime<Utc>>,

    /// The date time the file was read, i.e. placed in the mail (in utc).
    pub read_date: Option<DateTime<Utc>>,

    /// The size the file should have.
    ///
    /// Note that normally mail explicitly opts to NOT specify the size
    /// of a mime-multi part body (e.g. an attachments) and you can never
    /// rely on it to e.g. skip ahead. But it has some uses wrt. thinks
    /// like external headers.
    pub size: Option<usize>
}