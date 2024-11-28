use crate::schema::HeadTag;

pub fn hash_code(s: String) -> String {
    let mut h: u32 = 9;
    for c in s.chars() {
        h = (h ^ char_code(c)).wrapping_mul(9u32.pow(9))
    }

    let mut result = format!("{:x}", (h ^ h >> 9) + 0x10000);
    match result.len() {
        0..2 => panic!(),
        2..8 => result,
        _ => result.drain(1..8).collect::<String>(),
    }
}

fn char_code(c: char) -> u32 {
    let mut dst = [0; 2];
    c.encode_utf16(&mut dst);
    u32::from(dst[0]) + (u32::from(dst[1]) << 16)
}

pub fn hash_tag(tag: &HeadTag) -> String {
    let children = if let Some(text_context) = &tag.text_content {
        text_context.clone()
    } else if let Some(inner_html) = &tag.inner_html {
        inner_html.clone()
    } else {
        String::new()
    };
    let mut content = format!("{}:{children}:", tag.tag.as_str());

    for (key, value) in tag.props.iter() {
        content += &format!("{key}:{value}");
    }

    hash_code(content)
}
