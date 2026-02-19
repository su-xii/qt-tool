import {process} from "./net/api"
import {PopupMessage} from "./message";
import {getConfig} from "./store/config-store";
import {getHistoryList, HistoryItem, HistoryList, saveHistoryList} from "./store/history-sotre";



// 定义下载任务类型
type fn = () => void;

// 定义下载任务接口
interface DownloadTask {
    downloadId: number;
    status: 'pending' | 'completed' | 'failed';
    filePath?: string;
    userAction?: fn;
    timestamp: number;
}

// 下载队列
interface DownloadQueue {
    [downloadId: number]: DownloadTask;
}

// 初始化全局变量队列
const downloadQueue: DownloadQueue = {};


// 插件加载成功触发
chrome.runtime.onInstalled.addListener(() => {
    console.log('插件加载成功');
});

chrome.downloads.onCreated.addListener((downloadItem) => {
    console.log("download url",downloadItem.url)
    if(!downloadItem.url.includes("codesign.qq.com")) return

    console.log('下载任务被创建:', downloadItem.id);
    // 加入队列中
    downloadQueue[downloadItem.id] = {
        downloadId: downloadItem.id,
        status: 'pending',
        timestamp: Date.now(),
    };
    // 弹窗展示
    const _ = showDialog(downloadItem.id)
});

async function showDialog(downloadId: number){
    const config = await getConfig()
    chrome.windows.create({
        url: chrome.runtime.getURL(`popup.html#/dialog/${downloadId}`),
        type: "popup",
        width: config?.popupWidth ?? 400,
        height: config?.popupHeight ?? 240,
    },(x)=>{
        console.log("用户点击的操作",x)
    })
}

chrome.downloads.onChanged.addListener((downloadDelta) => {
    const downloadId = downloadDelta.id;
    if(!downloadQueue[downloadId]) return
    // 安全地获取文件名
    if (downloadDelta.filename && downloadDelta.filename.current) downloadQueue[downloadId].filePath = downloadDelta.filename.current
    if (!(downloadDelta.state && downloadDelta.state.current)) return;
    console.log('下载任务状态改变:', downloadId, downloadDelta.state.current);
    if (!downloadQueue[downloadId]) return;
    downloadQueue[downloadId].status = downloadDelta.state.current === 'complete' ? 'completed' : 'failed';
    if (downloadDelta.state.current === 'complete') processCompletedDownload(downloadId);
});

function processCompletedDownload(downloadId: number) {
    const task = downloadQueue[downloadId];
    if (!task) return;

    if (task.userAction) {
        executeUserAction(task);
        delete downloadQueue[downloadId];
    }
}

function executeUserAction(task: DownloadTask) {
    console.log('执行用户的下载行为:', task.downloadId);
    if (task.userAction) task.userAction()
}

// 处理popup交互请求
function processPopupRequest(message:PopupMessage){
    const downloadId = parseInt(message.downloadId)
    const task = downloadQueue[downloadId]
    // 绑定用户操作，这里是为了防止下载过快，用户行为还没有绑定到下载任务
    task.userAction = ()=>{
        process(message.name,task.filePath!,message.configIndex)
            .then(()=>handleMessage(message,task))
            .catch((e)=>handleMessage(message,task,e))
        // console.log("提交的参数:",message.name,task.filePath,message.configIndex)
    }

    if(task.status === 'completed'){
        // 执行用户操作
        executeUserAction(task)
        delete downloadQueue[downloadId];
    }
}

function handleMessage(message: PopupMessage, task: DownloadTask, e?: Error) {
    const success = !e;
    const notificationId = `cut-image-${Date.now()}-${Math.random()}`

    chrome.notifications.create(notificationId, {
        type: 'basic',
        iconUrl: chrome.runtime.getURL('icons/icon48.png'),
        title: success ? '✅ 切图成功' : `❌ ${message.name} 切图失败`,
        message: success ? message.name : (e?.message ?? '未知错误'),
        priority: 2,
        requireInteraction: false, // 自动关闭
    }, (createdId) => {
        console.log('通知已创建:', createdId);
    })
    handleHistory(message, task, e);
}


function handleHistory(message:PopupMessage,task:DownloadTask,e?:Error){
    const handleHistoryList = (historyList:HistoryList|null)=>{
        const historyItem:HistoryItem = {
            name:message.name,
            path:task.filePath!,
            time:task.timestamp,
            error: e?.message
        }

        if(!historyList){
            const _ = saveHistoryList({
                configIndex:message.configIndex,
                data:[historyItem]
            })
            return
        }

        // 限制最多50条，删除旧数据
        if(historyList.data.length > 50){
            historyList.data.pop() // 移除末尾元素
        }
        const _ = saveHistoryList({...historyList,data:[historyItem,...historyList.data]})
    }

    getHistoryList(message.configIndex)
        .then(handleHistoryList)
        .catch((e)=>console.log("获取历史失败",e))
}


chrome.runtime.onMessage.addListener((message:PopupMessage, sender, sendResponse) => {
    console.log("收到了弹窗消息：",message)
    processPopupRequest(message)
    // sendResponse({})
})


// 只对指定网站可用
chrome.tabs.onActivated.addListener(async ({tabId})=>{
    const tab = await chrome.tabs.get(tabId);
    console.log("onActivated tab:",tab)
    const ok = tab.url?.includes("codesign.qq.com");
    ok ? await chrome.action.enable(tabId)
        : await chrome.action.disable(tabId);
})

