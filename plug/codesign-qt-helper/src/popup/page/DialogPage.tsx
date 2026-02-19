import React, {useEffect, useState} from "react";
import {getConfig, saveConfig} from "../../store/config-store";
import {checkFile, getConfigList, getLimits} from "../../net/api";
import {ConfigsResponse} from "../../net/reponse";
import {PopupMessage} from "../../message";
import {useParams} from "react-router-dom";
import {Button, Input, message, Select} from "antd";

export default function DialogPage(){
    // 获取路由参数
    const {downloadId} = useParams<{downloadId:string}>()
    if(!downloadId){
        return (<>
            <div className="flex flex-col justify-center items-center">未获取到下载任务ID</div>
        </>)
    }

    // 配置相关参数
    const [name,setName] = useState('')
    const [configIndex,setConfigIndex] = useState(0)
    const [configList,setConfigList] = useState<ConfigsResponse[]>([])
    const [limits,setLimits] = useState<string[]>([])

    const [messageApi, contextHolder] = message.useMessage();
    const error = (message:string) => messageApi.open({
        type: 'error',
        content: message,
    })

    const warning = (message:string) => messageApi.open({
        type: 'warning',
        content: message,
    })

    const handleLimits = (configIndex:number)=>{
        getLimits(configIndex)
            .then(setLimits)
            .catch(e=>error(e.message))
    }

    // 持久化选项
    const handleSelectedChange = (value:number) =>{
        setConfigIndex(value)
        getConfig().then((config)=>{
            config = config ?? {}
            config.configIndex = value
            const _ = saveConfig(config)
        })
        handleLimits(value)
    }

    const onSubmit = (configIndex:number,name:string,limits:string[])=>{
        if(name === ""){
            const _ = warning("请输入文件名")
            return
        }

        const checkName = ()=>{
            let res = false
            for(const item of limits){
                if(name.endsWith(item)){
                    res = true
                    break
                }
            }
            return res
        }

        if(!checkName()){
            const _ = warning("文件名不符合要求")
            return
        }

        const handler = (res:boolean)=>{
            if(!res){
                const _ = warning("文件名已存在")
                return
            }

            const message: PopupMessage = {
                downloadId: downloadId!,
                name,
                configIndex
            }
            chrome.runtime.sendMessage(message,window.close)
        }
        checkFile(name,configIndex).then(handler).catch((e)=>error(e.message))
    }

    const cancel = ()=> window.close()

    useEffect(()=>{
        getConfigList()
            .then(configList=>{
                setConfigList(configList)
                getConfig().then((config)=> {
                    let index = config?.configIndex ?? 0
                    // 处理边界情况
                    if(index >= configList.length || index < 0) index = 0
                    setConfigIndex(index)
                    handleLimits(index)
                })
            })
            .catch(e=>error(e.message))
    },[])

    return (<>
        {contextHolder}
        <div className="flex flex-col p-4 h-full">
            {/*上半部分布局*/}
            <div className="flex flex-1 flex-col">
                <div className="w-full">
                    <Select
                        className="mr-1"
                        value={configIndex}
                        onChange={handleSelectedChange}
                        options={[...configList.map((item,index)=>({value:index,label:item.name}))]}
                    />
                    {/* 添加样式匹配 Select */}
                    <span className="text-[14px] text-gray-800 leading-normal mt-1">
                        {configList[configIndex]?.description}
                    </span>
                </div>

                <div style={{height:8}}></div>
                <Input value={name} onChange={e=>setName(e.target.value)} onPressEnter={()=>onSubmit(configIndex,name,limits)} placeholder="请输入名字,按回车提交"></Input>
                文件名字，例如 my.png
                <div>文件后缀限制：{limits.join(',')}</div>
            </div>

            {/*底部布局*/}
            <div className="mt-2">
                <div className="flex flex-row w-full justify-between">
                    <Button className="mr-1" block onClick={cancel}>取消</Button>
                    <Button className="ml-1" type="primary" block onClick={()=> onSubmit(configIndex,name,limits)}>确定</Button>
                </div>
            </div>

        </div>
    </>)
}