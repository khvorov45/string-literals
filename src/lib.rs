struct _Literal {}

impl _Literal {
    async fn _literal_async(&self) {
        _takes_literal_async(
            "String literal \
            over multiple \
            lines",
        )
        .await;
        _takes_literal(
            "String literal \
            over multiple \
            lines",
        );
        let _var = "String literal \
        over multiple \
        lines";
    }
    fn _literal(&self) {
        _takes_literal(
            "String literal \
            over multiple \
            lines",
        );
        let _var = "String literal \
        over multiple \
        lines";
    }
}

async fn _literal_async() {
    _takes_literal_async(
        "String literal \
        over multiple \
        lines",
    )
    .await;
    _takes_literal(
        "String literal \
        over multiple \
        lines",
    );
    let _var = "String literal \
    over multiple \
    lines";
}

async fn _takes_literal_async(_s: &str) {}

fn _takes_literal(_s: &str) {}

fn _literal() {
    _takes_literal(
        "String literal \
        over multiple \
        lines",
    );
    let _var = "String literal \
    over multiple \
    lines";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works_async() {
        let _lit_struct = _Literal {};
        _lit_struct._literal_async().await;
        _lit_struct._literal();
        _literal_async().await;
        _literal();
        assert_eq!(2 + 2, 4);
    }
}
