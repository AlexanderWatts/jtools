pub struct TokenCollection {
    tokens: Vec<i32>,
}

impl TokenCollection {
    pub fn new() -> Self {
        Self {
            tokens: vec![234, 78, 235],
        }
    }

    pub fn iter(&self) -> TokenIterator {
        TokenIterator {
            index: 0,
            token_collection: self,
        }
    }
}

pub struct TokenIterator<'a> {
    index: usize,
    token_collection: &'a TokenCollection,
}

impl Iterator for TokenIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.token_collection.tokens.len() {
            let token = self.token_collection.tokens.get(self.index);
            self.index += 1;
            return token.copied();
        }

        None
    }
}

#[test]
fn test() {
    let tc = TokenCollection::new();
    let mut iter = tc.iter().peekable();
    println!("{:#?}", &iter.peek());
    println!("{:#?}", &iter.next());
    println!("{:#?}", &iter.peek());
}
