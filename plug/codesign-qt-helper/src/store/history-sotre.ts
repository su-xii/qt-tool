const HISTORY_KEY = "history-list"

export interface HistoryList {
    configIndex: number
    data: HistoryItem[]
}

export interface HistoryItem{
    name:string
    path: string
    time: number
    error?: string
}

export async function saveHistoryList(history: HistoryList): Promise<void> {
    return new Promise((resolve) => {
        chrome.storage.local.set({ [`${HISTORY_KEY}/${history.configIndex}`]: JSON.stringify(history) }, () => {
            console.log('历史已保存', history);
            resolve();
        });
    });
}

export async function getHistoryList(configIndex: number): Promise<HistoryList | null> {
    return new Promise((resolve) => {
        const key = `${HISTORY_KEY}/${configIndex}`;
        chrome.storage.local.get([key], (result) => {
            const history = result[key];
            console.log('获取历史', history);
            resolve(history ? JSON.parse(history) : null);
        });
    });
}