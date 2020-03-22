
#[derive(Debug, Clone, Copy)]
pub enum Error {
    ShuffleError,
    CouldntFindPerson,
    BadChange(&'static str),
}
