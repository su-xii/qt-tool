import React, {useEffect, useState} from "react";
import {getConfig, saveConfig, Config, ModeType} from "../../store/config-store";
import {Button, message, InputNumber, Input, Radio} from "antd";
import {useNavigate} from "react-router-dom";
import {test} from "../../net/api";

interface PopupSize{
    height?:number,
    width?:number
}

export default function ConfigPage(){
    const [popupSize,setPopupSize] = useState<PopupSize>({})
    const [addr,setAddr] = useState("")
    const [mode,setMode] = useState<ModeType>("local")

    const [messageApi, contextHolder] = message.useMessage();
    const navigate = useNavigate()

    const toBack = ()=> navigate(-1)
    const testConnect = (addr:string)=>{
        getConfig().then((config)=>{
            config = config ?? {}
            config.serverAddr = addr
            saveConfig(config).then(()=>{
                test()
                    .then(()=>messageApi.success("连接成功"))
                    .catch((_)=>messageApi.error("连接失败"))
            })
        })
    }
    const save = (popupSize:PopupSize,addr:string, mode:ModeType)=>{
        getConfig().then((config)=>{
            config = config ?? {}
            config.popupWidth = popupSize.width
            config.popupHeight = popupSize.height
            config.serverAddr = addr
            config.mode = mode
            saveConfig(config)
                .then(() => messageApi.open({
                    type: 'success',
                    content: '保存配置成功',
                }))
        })
    }

    useEffect(()=>{
        const handleConfig = (config:Config|null) => {
            config = config ?? {}
            setPopupSize({
                height:config.popupHeight,
                width:config.popupWidth
            })
            setAddr(config.serverAddr ?? "http://localhost:9999")
            setMode(config.mode ?? "local")
        }
        getConfig().then(handleConfig)
    },[])


    return (<>
        {contextHolder}
        <div style={{width: 240}} className="flex flex-col p-4 rounded-md">
            <div className="flex flex-row w-full mt-2 justify-center items-center">
                弹窗宽度：
                <InputNumber value={popupSize.width} className="flex flex-1" min={400} max={1000} onChange={(value) => {
                    if (value) {
                        setPopupSize({...popupSize, width: value})
                    }
                }}/>
            </div>
            <div className="flex flex-row w-full mt-2 justify-center items-center">
                弹窗高度：
                <InputNumber value={popupSize.height} className="flex flex-1" min={240} max={1000}
                             onChange={(value) => {
                                 if (value) {
                                     setPopupSize({...popupSize, height: value})
                                 }
                             }}/>
            </div>
            <div className="flex flex-row w-full mt-2 justify-center items-center">
                服务地址：
                <Input value={addr} className="flex flex-1" onChange={(value)=> setAddr(value.target.value)}/>
            </div>
            <div className="flex flex-row w-full mt-2 items-center">
                连接模式：
                <Radio.Group
                    value={mode}
                    onChange={(e) => setMode(e.target.value)}
                    className="ml-2"
                >
                    <Radio.Button value="local">本地</Radio.Button>
                    <Radio.Button value="remote">远程</Radio.Button>
                </Radio.Group>
            </div>
            {mode === 'remote' && !addr && (
                <div className="text-red-500 text-xs mt-1">远程模式需要配置服务地址</div>
            )}
            <Button className="mt-2" block onClick={()=>testConnect(addr)}>测试连接</Button>
            <Button className="mt-2" block onClick={()=>save(popupSize,addr,mode)}>保存配置</Button>
            <Button className="mt-2 mb-2" onClick={toBack}>返回上级</Button>
        </div>
    </>)
}