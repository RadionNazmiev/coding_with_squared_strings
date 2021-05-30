
fn code(s: &str) -> String {
    let length = (s.len() as f32).sqrt().ceil() as usize;
    (0..length).map(|i| {
        (0..length)
            .map(|j| s.chars().nth(length * j + i).unwrap_or_else(|| '\u{b}').to_string())
            .rev()
            .collect::<String>()
    })
        .collect::<Vec<_>>()
        .join("\n")
}

fn decode(s: &str) -> String {
    let words = s.split("\n").collect::<Vec<_>>();

    (0..words.len())
        .map(|index| {
            words.iter()
                .map(|word| word.chars().nth(words.len() - index - 1).unwrap())
                .collect::<String>()
        })
        .collect::<String>()
        .replace("\u{b}", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_code(s: &str, exp: String) -> () {
        let ans = code(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }
    fn testing_decode(s: &str, exp: String) -> () {
        let ans = decode(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }

    #[test]
    fn basic_tests_code() {
        let data1 =  "What do you remember? When I looked at his streaky glasses, I wanted \
         to leave him. And before that? He stole those cherries for me at midnight. We were walking \
         in the rain and I loved him. And before that? I saw him coming \
         toward me that time at the picnic, edgy, foreign.";
        let data1_sol = "\x0bctg?.nadr d gdbW\n\x0b,i    lnis tl eh\n\x0b mtIAakietboaara\n\x0beeo nnigsoe st?t\n\
         \x0bd wsddnh lfls   \n\x0bgaaa  gtfeoeehWd\n\
         \x0bytrwbI .o rasiho\n\x0b, d e i rtev,se \n\x0b t hflnW h e  ny\n\x0bfhmioo emot Is o\n\x0boeemrvt eshh tIu\n\x0br   eehw eaiwr  \n\
         \x0beptc deea tmaelr\n\x0biihot  rtc?.naoe\n\x0bgcamhhre h  tkom\n\x0bnntiaia meHAeyke\n\x0b.i ntmiwirend em".to_string();
        testing_code(data1, data1_sol);

    }
    #[test]
    fn basic_tests_decode() {
        let data1 =  "And on black ground a bear-skin rug of snow. The sparks made no attempt to be the moon. ".to_string();
        let data1_sol = "\x0bet pnnacA\n\x0b  nao  kn\n\x0bmtorwrb d\n\x0boo k.ueg \n\x0bo as garo\n\x0bnbt T ron\n\x0b.etmho-u \n\x0b  eaefsnb\n\x0b\x0btmd  kdl\n\x0b\x0bhpessi a";
        testing_decode(data1_sol, data1);

    }
}
