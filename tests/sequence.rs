mod utils;

#[cfg(test)]
mod sequence {
    use std::ops::Not;

    use microparsec::{Context, ContextParserT, ParserRc, SequenceParser, StringParser};
    use rand::Rng;

    use crate::utils::{__test_get_error_message, __test_get_rand_string, __test_get_seeded_rng};

    #[test]
    fn test() {
        // 100 iterations of random testing
        for i in 0..1_000 {
            let (seed, mut rng) = __test_get_seeded_rng();
            let len = rng.gen_range(40..80);

            let str = __test_get_rand_string(&mut rng, len);

            // 50 iterations of random in str
            for x in 0..50 {
                let sequence_len = rng.gen_range(1..7);

                // get random substring
                let start = rng.gen_range(0..len - sequence_len - 1);
                let end = rng.gen_range(start + sequence_len..len + 1);
                let substr = &str[start..end];

                let piece_len = substr.len() / sequence_len;
                let mut pieces = Vec::new();

                // cut up substring into pieces
                let mut sequence: Vec<ParserRc<dyn ContextParserT<String>>> = Vec::new();
                for j in 0..sequence_len - 1 {
                    let snippet = &substr[j * piece_len..(j + 1) * piece_len];
                    sequence.push(ParserRc::new(StringParser::new(snippet)));
                    pieces.push(snippet);
                }
                sequence.push(ParserRc::new(StringParser::new(
                    &substr[(sequence_len - 1) * piece_len..],
                )));
                pieces.push(&substr[(sequence_len - 1) * piece_len..]);

                let res = SequenceParser::new(sequence).parse_from_context(Context {
                    txt: str.as_str().into(),
                    pos: start,
                });
                assert_eq!(
                    res.clone().unwrap().val,
                    pieces,
                    "Failed i={i}, x={x}, seed={seed}"
                );
                assert_eq!(
                    res.unwrap().ctx.pos,
                    end,
                    "Failed i={i}, x={x}, seed={seed}"
                );
            }

            // 100 iterations of random in str with random part
            for y in 0..100 {
                let sequence_len = rng.gen_range(1..7);

                // get random substring
                let start = rng.gen_range(0..len - sequence_len - 1);
                let end = rng.gen_range(start + sequence_len..len + 1);
                let substr = &str[start..end];

                let piece_len = substr.len() / sequence_len;
                let mut pieces = Vec::new();

                // cut up substring into pieces
                let mut sequence: Vec<ParserRc<dyn ContextParserT<String>>> = Vec::new();
                for j in 0..sequence_len - 1 {
                    let snippet = &substr[j * piece_len..(j + 1) * piece_len];
                    sequence.push(ParserRc::new(StringParser::new(snippet)));
                    pieces.push(snippet);
                }
                sequence.push(ParserRc::new(StringParser::new(
                    &substr[(sequence_len - 1) * piece_len..],
                )));
                pieces.push(&substr[(sequence_len - 1) * piece_len..]);

                // pick one index and change the string there randomly
                let change = rng.gen_range(0..sequence_len);
                let replacement = __test_get_rand_string(&mut rng, pieces[change].len());
                sequence[change] = ParserRc::new(StringParser::new(&replacement));
                pieces[change] = replacement.as_str();

                let res = SequenceParser::new(sequence).parse_from_context(Context {
                    txt: str.as_str().into(),
                    pos: start,
                });

                let mut offset = 0;
                let mut found_invalid = false;
                for j in 0..sequence_len {
                    // if the current is invalid, check that error is correct
                    if str[start + offset..].starts_with(pieces[j]).not() {
                        assert_eq!(
                            res.clone().unwrap_err().get_error_message(),
                            __test_get_error_message(pieces[j], start + offset),
                            "Failed i={i}, y={y}, j={j}, seed={seed}"
                        );
                        found_invalid = true;
                        break;
                    }

                    offset += pieces[j].len();
                }

                if found_invalid.not() {
                    assert_eq!(
                        res.clone().unwrap().val,
                        pieces,
                        "Failed i={i}, y={y}, seed={seed}"
                    );
                    assert_eq!(
                        res.unwrap().ctx.pos,
                        start + substr.len(),
                        "Failed i={i}, y={y}, seed={seed}"
                    );
                }
            }

            // 100 iterations of full random checking
            for z in 0..100 {
                // generate random string of random length and parse at random pos
                let sequence_len = rng.gen_range(1..7);
                let rand_len = rng.gen_range(sequence_len..30);
                let rand_pos = rng.gen_range(0..len);
                let substr = __test_get_rand_string(&mut rng, rand_len);

                let piece_len = substr.len() / sequence_len;
                let mut pieces = Vec::new();

                // cut up substring into pieces
                let mut sequence: Vec<ParserRc<dyn ContextParserT<String>>> = Vec::new();
                for j in 0..sequence_len - 1 {
                    let snippet = &substr[j * piece_len..(j + 1) * piece_len];
                    sequence.push(ParserRc::new(StringParser::new(snippet)));
                    pieces.push(snippet);
                }
                sequence.push(ParserRc::new(StringParser::new(
                    &substr[(sequence_len - 1) * piece_len..],
                )));
                pieces.push(&substr[(sequence_len - 1) * piece_len..]);

                let res = SequenceParser::new(sequence).parse_from_context(Context {
                    txt: str.as_str().into(),
                    pos: rand_pos,
                });

                let mut offset = 0;
                let mut found_invalid = false;
                for j in 0..sequence_len {
                    // if the current is invalid, check that error is correct
                    if str[rand_pos + offset..].starts_with(pieces[j]).not() {
                        assert_eq!(
                            res.clone().unwrap_err().get_error_message(),
                            __test_get_error_message(pieces[j], rand_pos + offset),
                            "Failed i={i}, z={z}, j={j}, seed={seed}"
                        );
                        found_invalid = true;
                        break;
                    }

                    offset += pieces[j].len();
                }

                if found_invalid.not() {
                    assert_eq!(
                        res.clone().unwrap().val,
                        pieces,
                        "Failed i={i}, z={z}, seed={seed}"
                    );
                    assert_eq!(
                        res.unwrap().ctx.pos,
                        rand_pos + substr.len(),
                        "Failed i={i}, z={z}, seed={seed}"
                    );
                }
            }
        }
    }
}
