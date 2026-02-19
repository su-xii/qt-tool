import React, {useEffect, useState} from "react";
import {getConfigList} from "../../net/api";
import {ConfigsResponse} from "../../net/reponse";
import {Button, Card, Divider, Drawer, message, Select} from "antd";
import {useNavigate} from "react-router-dom";
import {getHistoryList, HistoryItem, HistoryList} from "../../store/history-sotre";


type ShowDrawer = () => void
const HistoryItemLayout = ({item,showDrawer}:{item:HistoryItem,showDrawer:ShowDrawer})=>{

    const success = !item.error
    const date = new Date(item.time)
    const buildExtra = ()=>(<div><Button onClick={showDrawer} color="primary" variant="link" type="text">查看详情</Button>{success ? "✅":"❌"}</div>)
    return (<div className="mt-2">
    {/*用卡片布局，从上往下一次显示，名字，路径，错误原因，时间*/}
        <Card size="small" title={item.name} extra={buildExtra()} className="w-full mt-2">
            {/*<p>文件路径：{item.path}</p>*/}
            {/*{item.error && <p>错误原因：{item.error}</p>}*/}
            <p className="flex flex-row-reverse w-full">{date.toLocaleString()}</p>
        </Card>

    </div>)
}

const DrawerLayout = ({item}:{item?:HistoryItem})=>{
    if(!item) return (<></>)
    return (<>
        <p>文件路径：{item.path}</p>
        {item.error && <p className="mt-2">错误原因：{item.error}</p>}
    </>)
}

export default function HistoryPage(){

    const navigate = useNavigate()
    const toBack = ()=> navigate(-1)

    const [messageApi, contextHolder] = message.useMessage();
    const error = (text:string)=> messageApi.open({
        type: 'error',
        content: text,
    })

    const [list,setList] = useState<HistoryList|null>(null)
    const [configIndex,setConfigIndex] = useState(0)
    const [configList,setConfigList] = useState<ConfigsResponse[]>([])

    // 弹窗相关
    const [open, setOpen] = useState(false)
    const [drawerIndex,setDrawerIndex] = useState(0)
    const showDrawer = () => setOpen(true)
    const onClose = () => setOpen(false)

    const handleDrawerOpen = (index:number) =>{
        setDrawerIndex(index)
        showDrawer()
    }

    const handleSelectedChange = (value:number) =>{
        setConfigIndex(value)
        handleHistoryList(value)
    }
    const handleHistoryList = (configIndex:number)=>{
        getHistoryList(configIndex)
            .then(setList)
            .catch(e=>error(e.message))

    }
    useEffect(()=>{
        getConfigList()
            .then(setConfigList)
            .then(()=> handleHistoryList(configIndex))
            .catch(e=>error(e.message))
    },[])


    return (<>
        {contextHolder}
        <div style={{width: 240}} className="flex flex-col p-4 rounded-md">
            <Button onClick={toBack}>返回上级</Button>

            <Select
                className="mt-2 w-full"
                value={configIndex}
                onChange={handleSelectedChange}
                options={[...configList.map((item,index)=>({value:index,label:item.name}))]}
            />

            {/*列表内容，添加滚动条*/}
            <div>
                {/*{list && <Divider className="mt-2">{configList[list.configIndex].name}</Divider>}*/}
                <div
                    style={{
                        // 不显示滚动条
                        msOverflowStyle: 'none', // IE 和 Edge
                        scrollbarWidth: 'none', // Firefox
                    }}
                    className="flex flex-col mt-2 overflow-y-auto h-[300px]">
                    { list === null ?
                        (<div className="flex w-full h-full justify-center items-center">暂无历史记录</div>):
                        list.data.map((item,index)=>(<div>
                            <HistoryItemLayout
                                item={item}
                                showDrawer={()=> handleDrawerOpen(index)}
                            />
                        </div>))
                    }
                </div>
            </div>
        </div>

         {/*底部弹窗*/}
        <Drawer
            title={list?.data[drawerIndex].name}
            placement="bottom"
            closable={{ placement: 'end' }}
            // closable={false}
            onClose={onClose}
            open={open}
            size={240}
            key="bottom">
            <DrawerLayout item={list?.data[drawerIndex]} />
        </Drawer>
    </>)
}