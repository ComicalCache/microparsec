use microparsec::{Context, ContextParserT, StringParser};

use rand::{self, Rng};

mod utils;
use utils::{__test_get_error_message, __test_get_rand_string, __test_get_seeded_rng};

#[test]
fn string_test() {
    // 100 iterations of random testing
    for i in 0..1_000 {
        let (seed, mut rng) = __test_get_seeded_rng();
        let len = rng.gen_range(40..80);

        let str = __test_get_rand_string(&mut rng, len);

        // 50 iterations of random in str
        for x in 0..50 {
            let start = rng.gen_range(0..len);
            let end = rng.gen_range(start..len + 1);
            let substr = &str[start..end];
            let res = StringParser::new(substr).parse_from_context(Context {
                txt: str.as_str().into(),
                pos: start,
            });
            assert_eq!(
                res.clone().unwrap().val,
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
            let rand_len = rng.gen_range(2..30);
            let rand_pos = rng.gen_range(0..len);
            let substr = __test_get_rand_string(&mut rng, rand_len);
            let res = StringParser::new(&substr).parse_from_context(Context {
                txt: str.as_str().into(),
                pos: rand_pos,
            });
            if str[rand_pos..].starts_with(&substr) {
                assert_eq!(
                    res.clone().unwrap().val,
                    substr,
                    "Failed i={i}, x={y}, seed={seed}"
                );
                assert_eq!(
                    res.unwrap().ctx.pos,
                    rand_pos + substr.len(),
                    "Failed i={i}, x={y}, seed={seed}"
                );
            } else {
                assert_eq!(
                    res.unwrap_err().get_error_message(),
                    __test_get_error_message(&substr, rand_pos)
                );
            }
        }
    }
}
