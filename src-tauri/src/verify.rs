use std::time::Duration;

use reqwest::Client;

use crate::commands::LoginUser;

pub async fn verify(login_user: &mut LoginUser) -> Result<(), &'static str> {
    let client_builder = reqwest::ClientBuilder::new()
        .timeout(Duration::from_millis(500))
        .http1_title_case_headers()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36");
    let client = client_builder.build().unwrap();

    let mut flag = false; //连接是否成功
    if login_user.login_type == 0 {
        flag = telecom(&client, login_user).await?;
    }
    if !flag {
        flag = normal(&client, login_user).await?;
    }
    if !flag {
        login_user.login_type = 3; //尝试校园网
        flag = normal(&client, login_user).await?;
    }
    if flag {
        Ok(())
    } else {
        Err("验证校园网失败")
    }
}

async fn telecom(client: &Client, login_user: &LoginUser) -> Result<bool, &'static str> {
    let mut body = vec![
        ("loginType", ""),
        ("auth_type", "0"),
        ("isBindMac1", "0"),
        ("pageid", "1"),
        ("templatetype", "1"),
        ("listbindmac", "0"),
        ("recordmac", "0"),
        ("isRemind", "1"),
        ("loginTimes", ""),
        ("groupId", ""),
        ("distoken", ""),
        ("echostr", ""),
        ("url", ""),
        ("isautoauth", ""),
        ("notice_pic_loop2", "/portal/uploads/pc/demo2/images/bj.png"),
        (
            "notice_pic_loop1",
            "/portal/uploads/pc/demo2/images/logo.png",
        ),
        ("remInfo", "on"),
    ];
    body.push(("userId", &login_user.username));
    body.push(("passwd", &login_user.password));
    let req = client
        .post("http://10.254.241.3/webauth.do?&wlanacname=SC-CD-XXGCDX-SR8810-X")
        .form(&body)
        .send()
        .await;
    match req {
        Ok(rep) => {
            let result = rep.text().await.or(Err("转为字符串出错"))?;
            if result.contains("登录失败") {
                Err("电信的网络账号或密码错误")
            } else {
                Ok(true)
            }
        }
        Err(_) => Ok(false),
    }
}

async fn normal(client: &Client, login_user: &LoginUser) -> Result<bool, &'static str> {
    //需要先动态地获取校园网重定向中的参数
    let req = client
        .get("http://1.1.1.1/")
        .send()
        .await
        .or(Err("网络异常"))?;
    let rep = req.text().await.or(Err("转为字符串出错"))?;
    let mut iter = rep.split("'");
    let _ = iter.next().ok_or("分割字符串出错")?;
    let tmp = iter.next().ok_or("分割字符串出错")?;
    let mut iter = tmp.split("index.jsp?");
    let _ = iter.next().ok_or("分割字符串出错")?;
    let query_string = iter.next().ok_or("分割字符串出错")?;

    let mut body = vec![
        ("operatorPwd", ""),
        ("operatorUserId", ""),
        ("validcode", ""),
        ("passwordEncrypt", "false"),
    ];
    body.push(("queryString", query_string));
    body.push(("userId", &login_user.username));
    body.push(("password", &login_user.password));
    match login_user.login_type {
        // SB学校urlencode编码编两次也是没谁了
        0 => body.push(("service", "%E7%94%B5%E4%BF%A1")), //电信
        1 => body.push(("service", "%E7%A7%BB%E5%8A%A8")), //移动
        2 => body.push(("service", "%E8%81%94%E9%80%9A")), //联通
        3 => body.push(("service", "%E6%A0%A1%E5%9B%AD%E7%BD%91")), //校园网
        _ => (),
    }
    let req = client
        .post("http://10.254.241.19/eportal/InterFace.do?method=login")
        .form(&body)
        .send()
        .await;
    match req {
        Ok(rep) => {
            let result = rep.text().await.or(Err("转为字符串出错"))?;
            println!("{}", result);
            Ok(result.contains("success"))
        }
        Err(_) => Ok(false),
    }
}
