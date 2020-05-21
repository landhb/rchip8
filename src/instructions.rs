

pub mod instructions {
    use std::collections::HashMap;

    pub static readable: HashMap<[u8;2],&'static str> = [
        ([0u8],"SYS"),
        /*("BAD_UUID",  "Bad UUID, try again."),
        ("BAD_INFO", "That malware isn't mine..."),
        ("NO_REG", "Did you even register bro?"),
        ("BAD_PUSH", "Hmmm, did you respond to that command correctly?"),
        ("READ_DIRECTIONS","Re-read the directions in the question."),
        ("GARBAGE", "no comprendo"), */

    ].iter().cloned().collect();
}
    