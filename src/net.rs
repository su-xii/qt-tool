use crate::record::{load_records, save_records, Record};
use anyhow::Result;
use crate::util::result_util::ResultUtil;

// 上传记录,返回值为失败的记录
pub async fn upload_record(records:Vec<Record>) -> Result<Vec<Record>>{
    let client = reqwest::Client::new();
    async fn upload(client:&reqwest::Client,record:Record) -> Result<()>{
        let _ = client
            .post("http://qt.suxii.cn/api/record/add")
            .json(&record)
            .send()
            .await?
            .json::<ResultUtil>()
            .await?;
        tracing::info!("上传成功: {:?}", record);
        Ok(())
    }
    let mut fail_records = vec![];
    for record in records {
        if let Err(e) = upload(&client,record.clone()).await{
            fail_records.push(record);
            tracing::error!("上传失败: {:?}", e);
        }
    }

    Ok(fail_records)
}

pub async fn upload_record_auto() -> Result<()>{
    let records = load_records()?;
    let fail_records = upload_record(records).await?;
    save_records(&fail_records)?;
    Ok(())
}

#[cfg(test)]
mod tests{
    use chrono::Local;
    use qt_tool_key::generate_key;
    use super::*;

    #[test]
    fn test_upload_record(){
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let time = Local::now();
        let record = Record::new(generate_key(), true, time.timestamp_millis());
        runtime.block_on(async move {
            upload_record(vec![record]).await.unwrap();
        });
    }
}