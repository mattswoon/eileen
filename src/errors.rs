
#[derive(Debug)]
pub enum Error {
    ShuffleError,
    CouldntFindPerson,
    BadChange(&'static str),
}
