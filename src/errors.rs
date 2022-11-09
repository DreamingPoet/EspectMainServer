

#[derive(Debug)]
pub enum EspectError {
    Error(String),
}

impl From<serde_json::Error> for EspectError {
    fn from(err: serde_json::Error) -> Self {
        EspectError::Error(err.to_string())
    }
}

// impl FromResidual<Result<Infallible, serde_json::Error>> for EspectError {
//     fn from_residual(_residual: Result<Infallible, serde_json::Error>) -> Self {
//         // EspectError::Error("_".to_string())
//     }
// }