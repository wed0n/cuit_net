use windows::core::BSTR;

pub fn escape_xml(str: &str) -> Result<String, &'static str> {
    let bytes = str.as_bytes();
    let mut result: Vec<u8> = vec![];
    for item in bytes {
        match item {
            b'<' => result.extend_from_slice(b"&lt;"),
            b'&' => result.extend_from_slice(b"&amp;"),
            // 下列三个字符在XML的Text部分不需要转义
            // b'>'=>{result.extend_from_slice(b"&gt;")},
            // b'"'=>{result.extend_from_slice(b"&quot;")},
            // b'\''=>{result.extend_from_slice(b"&apos;")},
            other => result.push(*other),
        }
    }
    String::from_utf8(result).or(Err("转义XML失败"))
}

pub fn str_to_bstr(str: &str) -> Result<BSTR, &'static str> {
    let tmp: Vec<u16> = String::from(str).encode_utf16().collect();
    BSTR::from_wide(tmp.as_slice()).or(Err("创建BSTR失败"))
}
