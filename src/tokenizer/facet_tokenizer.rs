use super::{Token, TokenStream, Tokenizer};

/// BYTE used as a level separation in the binary
/// representation of facets.
pub const FACET_SEP_BYTE: u8 = 0u8;

/// The `FacetTokenizer` process a `Facet` binary representation
/// and emits a token for all of its parent.
///
/// For instance,  `/america/north_america/canada`
/// will emit the three following tokens
///     - `/america/north_america/canada`
///     - `/america/north_america`
///     - `/america`
#[derive(Clone, Default)]
pub struct FacetTokenizer {
    token: Token,
}

#[derive(Debug)]
enum State {
    RootFacetNotEmitted,
    UpToPosition(usize), //< we already emitted facet prefix up to &text[..cursor]
    Terminated,
}

pub struct FacetTokenStream<'a> {
    text: &'a str,
    state: State,
    token: &'a mut Token,
}

impl Tokenizer for FacetTokenizer {
    type TokenStream<'a> = FacetTokenStream<'a>;
    fn token_stream<'a>(&'a mut self, text: &'a str) -> FacetTokenStream<'a> {
        self.token.reset();
        self.token.position = 0;
        FacetTokenStream {
            text,
            state: State::RootFacetNotEmitted, //< pos is the first char that has not been processed yet.
            token: &mut self.token,
        }
    }
}

impl TokenStream for FacetTokenStream<'_> {
    fn advance(&mut self) -> bool {
        match self.state {
            State::RootFacetNotEmitted => {
                self.state = if self.text.is_empty() {
                    State::Terminated
                } else {
                    State::UpToPosition(0)
                };
                true
            }
            State::UpToPosition(cursor) => {
                let bytes: &[u8] = self.text.as_bytes();
                if let Some(next_sep_pos) = bytes[cursor + 1..]
                    .iter()
                    .cloned()
                    .position(|b| b == FACET_SEP_BYTE)
                    .map(|pos| cursor + 1 + pos)
                {
                    let facet_part = &self.text[cursor..next_sep_pos];
                    self.token.text.push_str(facet_part);
                    self.state = State::UpToPosition(next_sep_pos);
                } else {
                    let facet_part = &self.text[cursor..];
                    self.token.text.push_str(facet_part);
                    self.state = State::Terminated;
                }
                true
            }
            State::Terminated => false,
        }
    }

    fn token(&self) -> &Token {
        self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}
