use windows::Win32::System::Com::{
    CoInitializeEx, CoInitializeSecurity, CoUninitialize, COINIT_MULTITHREADED,
    EOLE_AUTHENTICATION_CAPABILITIES, RPC_C_AUTHN_LEVEL_PKT_PRIVACY, RPC_C_IMP_LEVEL_IMPERSONATE,
};

#[allow(dead_code)]
pub struct ComUtil {
    private: (), //禁止不经过new生成实例
}

impl ComUtil {
    pub unsafe fn new() -> Result<Self, &'static str> {
        CoInitializeEx(None, COINIT_MULTITHREADED).or(Err("初始化COM失败"))?;
        CoInitializeSecurity(
            None,
            -1,
            None,
            None,
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOLE_AUTHENTICATION_CAPABILITIES(0),
            None,
        )
        .map_err(|_| {
            CoUninitialize();
            "初始化COM安全策略失败"
        })?;
        Ok(ComUtil { private: () })
    }
}

impl Drop for ComUtil {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

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
