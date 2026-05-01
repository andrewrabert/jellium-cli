use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const PATH_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'%');

pub fn encode_path(s: &str) -> String {
    utf8_percent_encode(s, PATH_SET).to_string()
}
