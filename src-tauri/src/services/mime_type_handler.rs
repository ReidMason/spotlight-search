const LNK_MIME_TYPE: &str = "application/x-ms-shortcut";

// 4c 00 00 00 01 14 02 00 00 00 00 00 c0 00 00 00 L............... This is lnk mime type
// 00 00 00 46 8b 00 08 00 11 00 00 00 9f f6 c0 fd ...F............
// bd cb cc 01 42 94 c9 85 92 cc cc 01 5d 6c d7 d2 ....B........l..
// c8 cc cc 01 00 00 00 00 00 00 00 00 01 00 00 00 ................

fn custom_matcher(buf: &[u8]) -> bool {
    return buf.len() >= 3 && buf[0] == 0x4c && buf[1] == 0x00 && buf[2] == 0x00;
}

pub fn get_custom_infer() -> infer::Infer {
    let mut info = infer::Infer::new();
    info.add(LNK_MIME_TYPE, "lnk", custom_matcher);
    info
}

pub fn get_matcher_type(mime_type: infer::Type) -> infer::MatcherType {
    match mime_type.mime_type() {
        LNK_MIME_TYPE => infer::MatcherType::App,
        _ => mime_type.matcher_type(),
    }
}
