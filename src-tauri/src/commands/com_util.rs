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
