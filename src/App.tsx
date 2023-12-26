import {
  Body1Stronger,
  Button,
  Dropdown,
  FluentProvider,
  Input,
  Label,
  Option,
  webDarkTheme,
  webLightTheme,
} from '@fluentui/react-components'
import { Eye24Regular, EyeOff24Regular } from '@fluentui/react-icons'
import { invoke } from '@tauri-apps/api'
import { useEffect, useRef, useState } from 'react'
export default function App() {
  const [theme, setTheme] = useState(webLightTheme)
  const [errMsg, setErrMsg] = useState('')
  const [account, setAccount] = useState('')
  const lastAccount = useRef('')
  const [password, setPassword] = useState('')
  const [visable, setVisable] = useState(false)
  const loginType = useRef(-1)

  const onBlur = (
    current: string,
    set: React.Dispatch<React.SetStateAction<string>>,
    last: React.MutableRefObject<string>
  ) => {
    if (/^[1-9]\d{9}$/.test(current)) {
      last.current = current
    } else {
      set(last.current)
    }
  }

  const setTask = async () => {
    if (loginType.current == -1) {
      setErrMsg('未设置运营商')
      return
    }
    if (!/^[1-9]\d{9}$/.test(account)) {
      setErrMsg('学号为10位数字')
      return
    }
    if (password.length == 0) {
      setErrMsg('密码不能为空')
      return
    }
    try {
      await invoke('create_task', {
        username: account,
        password: password,
        loginType: loginType.current,
      })
      setErrMsg('')
    } catch (error) {
      setErrMsg(error as string)
    }
  }
  const deleteTask = async () => {
    try {
      await invoke('delete_task')
      setErrMsg('')
    } catch (error) {
      setErrMsg(error as string)
    }
  }

  useEffect(() => {
    const mediaQueryList = window.matchMedia('(prefers-color-scheme: dark)')
    setTheme(mediaQueryList.matches ? webDarkTheme : webLightTheme)
    const listener = (event: MediaQueryListEvent) => {
      setTheme(event.matches ? webDarkTheme : webLightTheme)
    }
    mediaQueryList.addEventListener('change', listener)
    return () => {
      mediaQueryList.removeEventListener('change', listener)
    }
  }, [])

  return (
    <FluentProvider
      style={{
        width: '100%',
        height: '100%',
      }}
      theme={theme}>
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
        }}>
        <div
          style={{
            display: 'flex',
            padding: 15,
            flexDirection: 'column',
            justifyContent: 'center',
            alignItems: 'center',
          }}>
          {errMsg == '' ? (
            <Body1Stronger>本程序移动位置后需要重新设置</Body1Stronger>
          ) : (
            <Body1Stronger style={{ color: '#d13438' }}>{errMsg}</Body1Stronger>
          )}

          <div
            style={{
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'space-around',
              alignItems: 'flex-end',
              marginBlock: 35,
            }}>
            <div style={{ marginBottom: 10 }}>
              <Label weight="semibold">学号:</Label>
              <Input
                value={account}
                size="medium"
                style={{ marginLeft: 8, width: 160 }}
                onChange={(_event, data) => {
                  setAccount(data.value)
                }}
                onBlur={onBlur.bind(null, account, setAccount, lastAccount)}
                pattern="\d"
              />
            </div>
            <div style={{ marginBottom: 10 }}>
              <Label weight="semibold">密码:</Label>
              <Input
                value={password}
                type={visable ? 'text' : 'password'}
                size="medium"
                style={{ marginLeft: 8, width: 160 }}
                maxLength={20}
                onChange={(_event, data) => {
                  if (!/\s/.test(data.value)) {
                    setPassword(data.value)
                  }
                }}
                contentAfter={
                  <Button
                    appearance="transparent"
                    icon={visable ? <Eye24Regular /> : <EyeOff24Regular />}
                    onClick={() => {
                      setVisable((visable) => !visable)
                    }}
                  />
                }
              />
            </div>
            <div style={{ display: 'flex', alignItems: 'center' }}>
              <Label weight="semibold">运营商:</Label>
              <Dropdown
                style={{ marginLeft: 8, width: 160, minWidth: 0 }}
                onOptionSelect={(_event, data) => {
                  if (data.optionValue) {
                    loginType.current = parseInt(data.optionValue)
                  }
                }}>
                <Option value="0">电信</Option>
                <Option value="1">移动</Option>
                <Option value="2">联通</Option>
                <Option value="3">教育网</Option>
              </Dropdown>
            </div>
          </div>
          <div>
            <Button style={{ marginRight: 20 }} onClick={deleteTask}>
              删除
            </Button>
            <Button appearance="primary" onClick={setTask}>
              设置
            </Button>
          </div>
        </div>
      </div>
    </FluentProvider>
  )
}
