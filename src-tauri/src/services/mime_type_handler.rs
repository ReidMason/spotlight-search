const LNK_MIME_TYPE: &str = "application/x-ms-shortcut";

// 4c 00 00 00 01 14 02 00 00 00 00 00 c0 00 00 00 L............... This is lnk mime type
// 00 00 00 46 8b 00 08 00 11 00 00 00 9f f6 c0 fd ...F............
// bd cb cc 01 42 94 c9 85 92 cc cc 01 5d 6c d7 d2 ....B........l..
// c8 cc cc 01 00 00 00 00 00 00 00 00 01 00 00 00 ................

pub struct MatcherType(pub infer::MatcherType);

impl serde::Serialize for MatcherType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            infer::MatcherType::App => serializer.serialize_str("app"),
            infer::MatcherType::Archive => serializer.serialize_str("archive"),
            infer::MatcherType::Audio => serializer.serialize_str("audio"),
            infer::MatcherType::Book => serializer.serialize_str("book"),
            infer::MatcherType::Doc => serializer.serialize_str("doc"),
            infer::MatcherType::Font => serializer.serialize_str("font"),
            infer::MatcherType::Image => serializer.serialize_str("image"),
            infer::MatcherType::Text => serializer.serialize_str("text"),
            infer::MatcherType::Video => serializer.serialize_str("video"),
            infer::MatcherType::Custom => serializer.serialize_str("other"),
        }
    }
}

fn custom_matcher(buf: &[u8]) -> bool {
    return buf.len() >= 3 && buf[0] == 0x4c && buf[1] == 0x00 && buf[2] == 0x00;
}

pub fn get_custom_infer() -> infer::Infer {
    let mut info = infer::Infer::new();
    info.add(LNK_MIME_TYPE, "lnk", custom_matcher);
    info
}

pub fn get_matcher_type(file_type: infer::Type) -> infer::MatcherType {
    match file_type.mime_type() {
        LNK_MIME_TYPE => infer::MatcherType::App,
        _ => file_type.matcher_type(),
    }
}
