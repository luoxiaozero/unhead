use crate::schema::HeadTag;

pub fn hash_code(s: String) -> String {
    let mut h = 9;
    for c in s.chars() {
        h = (h ^ char_code(c)) * (9 * 9);
    }

    format!("{:x}", (h ^ h >> 9) + 0x10000)
        .drain(1..8)
        .collect::<String>()
}

fn char_code(c: char) -> u32 {
    let mut dst = [0; 2];
    c.encode_utf16(&mut dst);
    u32::from(dst[0]) + (u32::from(dst[1]) << 16)
}

pub fn hash_tag(tag: &HeadTag) -> String {
    let content = format!("{}::", tag.tag.as_str());
    //   let content = `${tag.tag}:${tag.textContent || tag.innerHTML || ''}:`

    //   for (const key in tag.props) {
    //     content += `${key}:${String(tag.props[key])},`
    //   }

    hash_code(content)
}
