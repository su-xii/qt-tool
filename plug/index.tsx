/*
import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import './styles.css';

interface DownloadTask {
    downloadId: number;
    status: 'pending' | 'completed' | 'failed';
    filePath?: string;
    userAction?: any;
    timestamp: number;
}

const Popup: React.FC = () => {
    const [downloadQueue, setDownloadQueue] = useState<{ [key: number]: DownloadTask }>({});
    const [selectedDownloadId, setSelectedDownloadId] = useState<string>('');
    const [actionType, setActionType] = useState<string>('extract');
    const [data, setData] = useState<string>('');
    const [input, setInput] = useState<string>('');

    useEffect(() => {
        loadDownloadQueue();
        loadStoredData();
    }, []);

    const loadDownloadQueue = () => {
        chrome.runtime.sendMessage({ action: 'getDownloadQueue' }, (response) => {
            if (response.queue) {
                setDownloadQueue(response.queue);
            }
        });
    };

    const loadStoredData = () => {
        chrome.runtime.sendMessage({ action: 'getData' }, (response) => {
            setData(response);
        });
    };

    const handleBindAction = () => {
        if (!selectedDownloadId) {
            alert('请选择下载任务');
            return;
        }

        const downloadId = parseInt(selectedDownloadId);
        const userAction = {
            type: actionType,
            // TODO: 在这里添加更多用户操作参数
            // 例如：解压路径、上传目标等
            params: {},
        };

        chrome.runtime.sendMessage({ action: 'bindDownload', downloadId, userAction }, (response) => {
            if (response.success) {
                alert('绑定成功！');
                loadDownloadQueue();
            } else {
                alert('绑定失败：' + response.message);
            }
        });
    };

    const handleCancelDownload = (downloadId: number) => {
        if (confirm('确定要取消这个下载任务吗？')) {
            chrome.runtime.sendMessage({ action: 'cancelDownload', downloadId }, (response) => {
                if (response.success) {
                    alert('已取消');
                    loadDownloadQueue();
                }
            });
        }
    };

    const handleSave = () => {
        chrome.runtime.sendMessage({ action: 'setData', data: input }, (response) => {
            if (response.success) {
                setData(input);
                setInput('');
            }
        });
    };

    const getStatusText = (status: string) => {
        switch (status) {
            case 'pending': return '下载中';
            case 'completed': return '已完成';
            case 'failed': return '失败';
            default: return status;
        }
    };

    const getStatusColor = (status: string) => {
        switch (status) {
            case 'pending': return '#ffd700';
            case 'completed': return '#4caf50';
            case 'failed': return '#f44336';
            default: return '#fff';
        }
    };

    return (
        <div className="popup-container">
            <h1>Chrome Extension</h1>

            <div className="section">
                <h2>下载任务队列</h2>
                {Object.keys(downloadQueue).length === 0 ? (
                    <p className="empty-message">暂无下载任务</p>
                ) : (
                    <div className="download-list">
                        {Object.entries(downloadQueue).map(([id, task]) => (
                            <div key={id} className="download-item">
                                <div className="download-info">
                                    <span className="download-id">#{task.downloadId}</span>
                                    <span
                                        className="download-status"
                                        style={{ backgroundColor: getStatusColor(task.status) }}
                                    >
                    {getStatusText(task.status)}
                  </span>
                                    {task.userAction && <span className="action-bound">✓ 已绑定操作</span>}
                                </div>
                                <button
                                    className="cancel-btn"
                                    onClick={() => handleCancelDownload(task.downloadId)}
                                >
                                    取消
                                </button>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            <div className="section">
                <h2>绑定下载操作</h2>
                <div className="bind-section">
                    <select
                        value={selectedDownloadId}
                        onChange={(e) => setSelectedDownloadId(e.target.value)}
                        className="select-input"
                    >
                        <option value="">选择下载任务</option>
                        {Object.entries(downloadQueue)
                            .filter(([_, task]) => !task.userAction)
                            .map(([id, task]) => (
                                <option key={id} value={id}>
                                    #{task.downloadId} - {getStatusText(task.status)}
                                </option>
                            ))}
                    </select>

                    <select
                        value={actionType}
                        onChange={(e) => setActionType(e.target.value)}
                        className="select-input"
                    >
                        <option value="extract">解压文件</option>
                        <option value="notify">发送通知</option>
                        <option value="upload">上传文件</option>
                        {/!* TODO: 在这里添加更多操作类型 *!/}
                    </select>

                    <button onClick={handleBindAction} className="bind-btn">
                        绑定操作
                    </button>
                </div>
            </div>

            <div className="section">
                <h2>存储的数据</h2>
                <div className="data-display">
                    <p>{data || '暂无数据'}</p>
                </div>
                <div className="input-section">
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="输入要保存的数据"
                    />
                    <button onClick={handleSave}>保存</button>
                </div>
            </div>
        </div>
    );
};

const root = ReactDOM.createRoot(document.getElementById('root')!);
root.render(<Popup />);*/
