use crate::err::ErrMsg;

#[derive(Debug, thiserror::Error)]
pub enum ErrLlm {
    #[error("{0}\nollama is not installed and is required")]
    OllamaNotInstalled(ErrMsg),

    #[error("{0}\nerror which originated from ollama cli during execution")]
    OllamaError(ErrMsg),

    #[error("{0} http request to ollama failed")]
    HttpRequestFailure(ErrMsg),
}
