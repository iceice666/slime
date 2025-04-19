use chumsky::prelude::*;

fn parser<'src>() -> impl Parser<'src, &'src str, ()> {
    end()
}