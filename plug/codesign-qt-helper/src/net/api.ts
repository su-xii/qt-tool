import {get, post, upload} from "./request"
import {ConfigsResponse} from "./reponse"


export function test():Promise<void>{
    return get<void>("/process/test")
}

export function getConfigList():Promise<ConfigsResponse[]>{
    return get<Array<ConfigsResponse>>("/process/configs")
}

export function process(name:string,path:string,config_index:number):Promise<void>{
    return post<void>("/process",{
        name,
        path,
        config_index
    })
}

export function uploadFile(name:string,fileBlob:Blob,config_index:number):Promise<void>{
    const formData = new FormData();
    formData.append('file', fileBlob);
    formData.append('name', name);
    formData.append('config_index', config_index.toString());
    return upload<void>("/process/upload", formData);
}

export function checkFile(name:string,config_index:number):Promise<boolean>{
    return post<boolean>("/process/check",{
        name,
        config_index
    })
}

export function getLimits(config_index:number):Promise<string[]>{
    return get<string[]>("/process/limits",{
        params:{
            config_index
        }
    })
}
