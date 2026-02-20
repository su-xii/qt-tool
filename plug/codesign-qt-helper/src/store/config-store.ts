const CONFIG_KEY = 'config'

export interface Config{
    configIndex?:number,
    serverAddr?:string,
    popupHeight?:number,
    popupWidth?:number
}


// 存储配置到本地
export async function saveConfig(config: Config): Promise<void> {
    return new Promise((resolve) => {
        chrome.storage.local.set({ [CONFIG_KEY]: JSON.stringify(config) }, () => {
            console.log('配置已保存', config);
            resolve();
        });
    });
}

// 获取配置
export async function getConfig(): Promise<Config | null> {
    return new Promise((resolve) => {
        chrome.storage.local.get([CONFIG_KEY], (result) => {
            const config = result[CONFIG_KEY];
            console.log('获取配置', config);
            resolve(config ? JSON.parse(config) : null);
        });
    });
}