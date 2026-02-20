import axios, { AxiosRequestConfig, AxiosResponse } from 'axios';
import {getConfig} from "../store/config-store";

// 响应数据结构
export interface ApiResponse<T = any> {
  code: number;
  data: T;
  message: string;
}

// 请求配置
export interface RequestConfig extends AxiosRequestConfig {
  params?: Record<string, any>;
}

// 创建 axios 实例
const instance = axios.create({
  baseURL: 'http://localhost:9999',
  timeout: 3000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器
instance.interceptors.request.use(async (config) => {
  return config;
});

// 响应拦截器统一处理
instance.interceptors.response.use(
    (response: AxiosResponse<ApiResponse>) => {
      const { code, message } = response.data;

      // 业务错误处理
      if (code !== 0) {
        return Promise.reject(new Error(message || '请求失败'));
      }

      return response;
    },
    (error) => {
      const message = error.response?.data?.message || '请求失败';
      return Promise.reject(new Error(message));
    }
);

const handleConfigServer = async ()=>{
    const config = await getConfig();
    if(config?.serverAddr){
        instance.defaults.baseURL = config.serverAddr;
    }
}

// GET 方法
export async function get<T = any>(url: string, config?: RequestConfig):Promise<T> {
  await handleConfigServer();
  const res = await instance.get<ApiResponse<T>>(url, config);
  return res.data.data;
}

// POST 方法
export async function post<T = any>(url: string, data?: any, config?: RequestConfig) :Promise<T> {
  await handleConfigServer();
  const res = await instance.post<ApiResponse<T>>(url, data, config);
  return res.data.data;
}


// 导出实例（用于扩展）
export { instance };