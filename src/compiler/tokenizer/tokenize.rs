use crate::{
    compiler::tokenizer::{AdvancedToken, get_advanced_tokens, get_basic_tokens},
    err::ErrApp,
};

pub fn tokenize(s: String) -> Result<Vec<AdvancedToken>, ErrApp> {
    let basic_tokens = get_basic_tokens(s)?;
    let refined_tokens = get_advanced_tokens(basic_tokens)?;
    return Ok(refined_tokens);
}
