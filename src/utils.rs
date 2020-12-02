// Q&D way of catching all errors without worrying about their types at compile time
// @see https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
pub(crate) type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// type alias to make it reaaaaalllly concise
pub(crate) type NullResult = GenericResult<()>;

// for functions, which should return a result, but can never fail:
// type OkResult<T> = Result<T, core::convert::Infallible>;

#[derive(Debug, Clone)]
pub(crate) struct CustomError(pub(crate) String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "custom error raised: {}", self.0)
    }
}

impl std::error::Error for CustomError {}

pub(crate) type CustomResult<T> = std::result::Result<T, CustomError>;
