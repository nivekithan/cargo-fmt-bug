fn foo() {
    match cur_tok {
        Token::Keyword(KeywordKind::If) => {
            loop {
                let cur_tok = self.get_cur_token()?;

                match cur_tok {
                    Token::Keyword(KeywordKind::Else) => {
                        match cur_tok {
                                Token::Keyword(KeywordKind::If) => {
                                    self.next(); // consumes keyword if
                                    
                                    continue;
                                },

                                tok => return Err(format!("Expected token to be either keyword if or token {{ but got token {:?}", tok)) 
                            }
                    }
                }
            }
        }
    }
}
