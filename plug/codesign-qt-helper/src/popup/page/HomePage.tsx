import React from "react";
import {Button} from "antd";
import {useNavigate} from "react-router-dom";


export default function HomePage(){
    const navigate = useNavigate()
    const toHistoryPage = ()=> navigate("/history")
    const toConfigPage = ()=> navigate("/config")
    return (<>
        <div style={{width: 240}} className="flex flex-col p-4 rounded-md">
            <Button className="mt-2" block onClick={toHistoryPage}>历史记录</Button>
            <Button className="mt-2 mb-2" onClick={toConfigPage}>通用设置</Button>
        </div>
    </>)
}