mod utils;

#[cfg(test)]
mod optional {
    use microparsec::{Context, ContextParserT, OptionalParser, ParserRc, StringParser};
    use rand::Rng;

    use crate::utils::{__test_get_rand_string, __test_get_seeded_rng};

    #[test]
    fn test() {
        // 100 iterations of random testing
        for i in 0..1_000 {
            let (seed, mut rng) = __test_get_seeded_rng();
            let len = rng.gen_range(40..80);

            let str = __test_get_rand_string(&mut rng, len);

            // 50 iterations of random in str
            for x in 0..50 {
                // get random substring
                let start = rng.gen_range(0..len);
                let end = rng.gen_range(start + 1..len + 1);
                let substr = &str[start..end];

                let res = OptionalParser::new(ParserRc::new(StringParser::new(substr)))
                    .parse_from_context(Context {
                        txt: str.as_str().into(),
                        pos: start,
                    });
                assert_eq!(
                    res.clone().unwrap().val.unwrap(),
                    substr,
                    "Failed i={i}, x={x}, seed={seed}"
                );
                assert_eq!(
                    res.unwrap().ctx.pos,
                    end,
                    "Failed i={i}, x={x}, seed={seed}"
                );
            }

            // 100 iterations of full random checking
            for y in 0..100 {
                // generate random string of random length and parse at random pos
                let rand_len = rng.gen_range(2..30);
                let rand_pos = rng.gen_range(0..len);
                let substr = __test_get_rand_string(&mut rng, rand_len);

                let res = OptionalParser::new(ParserRc::new(StringParser::new(&substr)))
                    .parse_from_context(Context {
                        txt: str.as_str().into(),
                        pos: rand_pos,
                    });
                if str[rand_pos..].starts_with(&substr) {
                    assert_eq!(
                        res.clone().unwrap().val.unwrap(),
                        substr,
                        "Failed i={i}, y={y}, seed={seed}"
                    );
                    assert_eq!(
                        res.unwrap().ctx.pos,
                        rand_pos + substr.len(),
                        "Failed i={i}, y={y}, seed={seed}"
                    );
                } else {
                    assert_eq!(
                        res.unwrap().val.is_none(),
                        true,
                        "Failed i={i}, y={y}, seed={seed}"
                    );
                }
            }
        }
    }
}
