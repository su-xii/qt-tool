import React, {useEffect, useState} from "react";
import {getConfig, saveConfig,Config} from "../../store/config-store";
import {Button, message, InputNumber} from "antd";
import {useNavigate} from "react-router-dom";

interface PopupSize{
    height?:number,
    width?:number
}

export default function ConfigPage(){
    const [popupSize,setPopupSize] = useState<PopupSize>({})
    // const [message,setMessage] = useState("")

    const [messageApi, contextHolder] = message.useMessage();
    const navigate = useNavigate()

    const toBack = ()=> navigate(-1)
    const save = ()=>{
        getConfig().then((config)=>{
            config = config ?? {}
            config.popupWidth = popupSize.width
            config.popupHeight = popupSize.height
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
        }
        getConfig().then(handleConfig)
    },[])


    return (<>
        {contextHolder}
        <div style={{width: 240}} className="flex flex-col p-4 rounded-md">
            <div className="flex flex-row w-full mt-2 justify-center items-center">
                弹窗宽度：
                <InputNumber value={popupSize.width} className="flex flex-1" min={400} max={1000} onChange={(value) => {
                    if(value){
                        setPopupSize({...popupSize,width:value})
                    }
                }} />
            </div>
            <div className="flex flex-row w-full mt-2 justify-center items-center">
                弹窗高度：
                <InputNumber value={popupSize.height} className="flex flex-1" min={240} max={1000} onChange={(value) => {
                    if(value){
                        setPopupSize({...popupSize,height:value})
                    }
                }} />
            </div>
            <Button className="mt-2" block onClick={save}>保存配置</Button>
            <Button className="mt-2 mb-2" onClick={toBack}>返回上级</Button>
        </div>
    </>)
}