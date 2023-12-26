mod util;
use windows::{
    core::GUID,
    Win32::System::{
        Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        TaskScheduler::{ITaskFolder, ITaskService, TASK_CREATE_OR_UPDATE, TASK_LOGON_NONE},
        Variant::VARIANT,
    },
};

use util::str_to_bstr;

use self::util::escape_xml;
pub struct LoginUser {
    pub username: String,
    pub password: String,
    pub login_type: i32,
}

fn generate_task_xml(path: &str, login_user: &LoginUser) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.4" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
    <RegistrationInfo>
    <Date>2023-12-22T13:35:23.8585696</Date>
    <Author>Wed0n</Author>
    <URI>\cuit_net</URI>
    </RegistrationInfo>
    <Triggers>
    <EventTrigger>
        <Enabled>true</Enabled>
        <Subscription>&lt;QueryList&gt;&lt;Query Id="0" Path="Microsoft-Windows-NetworkProfile/Operational"&gt;&lt;Select Path="Microsoft-Windows-NetworkProfile/Operational"&gt;*[System[Provider[@Name='Microsoft-Windows-NetworkProfile'] and EventID=10000]]&lt;/Select&gt;&lt;/Query&gt;&lt;/QueryList&gt;</Subscription>
    </EventTrigger>
    <LogonTrigger>
      <Enabled>true</Enabled>
    </LogonTrigger>
    </Triggers>
    <Principals>
    <Principal id="Author">
        <UserId>S-1-5-18</UserId>
        <RunLevel>HighestAvailable</RunLevel>
    </Principal>
    </Principals>
    <Settings>
    <MultipleInstancesPolicy>StopExisting</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>true</StopIfGoingOnBatteries>
    <AllowHardTerminate>false</AllowHardTerminate>
    <StartWhenAvailable>false</StartWhenAvailable>
    <RunOnlyIfNetworkAvailable>false</RunOnlyIfNetworkAvailable>
    <IdleSettings>
        <StopOnIdleEnd>true</StopOnIdleEnd>
        <RestartOnIdle>false</RestartOnIdle>
    </IdleSettings>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <Hidden>false</Hidden>
    <RunOnlyIfIdle>false</RunOnlyIfIdle>
    <DisallowStartOnRemoteAppSession>false</DisallowStartOnRemoteAppSession>
    <UseUnifiedSchedulingEngine>true</UseUnifiedSchedulingEngine>
    <WakeToRun>false</WakeToRun>
    <ExecutionTimeLimit>PT1H</ExecutionTimeLimit>
    <Priority>7</Priority>
    </Settings>
    <Actions Context="Author">
    <Exec>
        <Command>{}</Command>
        <Arguments>{} {} {}</Arguments>
    </Exec>
    </Actions>
</Task>"#,
        path, login_user.username, login_user.password, login_user.login_type
    )
}

unsafe fn get_root_folder() -> Result<ITaskFolder, &'static str> {
    //参考 https://github.com/microsoft/windows-rs/issues/1946#issuecomment-1436749818
    const CLSID_TASK_SERVICE: GUID = GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd);
    let service: ITaskService = CoCreateInstance(&CLSID_TASK_SERVICE, None, CLSCTX_INPROC_SERVER)
        .or(Err("创建ITaskService实例失败"))?;
    service
        .Connect(
            VARIANT::default(),
            VARIANT::default(),
            VARIANT::default(),
            VARIANT::default(),
        )
        .or(Err("连接ITaskService失败"))?;

    let root_folder = service
        .GetFolder(&str_to_bstr("\\")?)
        .or(Err("获取Folder失败"))?;
    Ok(root_folder)
}

#[tauri::command]
pub async fn create_task(
    username: String,
    password: String,
    login_type: i32,
) -> Result<(), &'static str> {
    let login_user = LoginUser {
        username: escape_xml(&username)?,
        password: escape_xml(&password)?,
        login_type,
    };
    unsafe {
        let root_folder = get_root_folder()?;
        root_folder
            .RegisterTask(
                None,
                &str_to_bstr(&generate_task_xml(
                    std::env::current_exe().unwrap().to_str().unwrap(),
                    &login_user,
                ))?,
                TASK_CREATE_OR_UPDATE.0,
                VARIANT::default(),
                VARIANT::default(),
                TASK_LOGON_NONE,
                VARIANT::default(),
            )
            .or(Err("注册计划任务失败"))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_task() -> Result<(), &'static str> {
    unsafe {
        let root_folder = get_root_folder()?;
        root_folder
            .DeleteTask(&str_to_bstr("cuit_net")?, 0)
            .or(Err("删除计划任务失败"))?;
        Ok(())
    }
}
