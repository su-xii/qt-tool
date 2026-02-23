use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use anyhow::Result;
use chrono::{Local};
use qt_tool_key::generate_key;
use zip::ZipArchive;
use crate::record::{add_record, Record};
use crate::cli::Cli;
use crate::server::config::OutputItem;

/// 抽象运行行为
pub trait Handler{
    fn run(cli :&Cli) -> impl Future<Output = Result<()>> + Send;
}


// 解压文件,name包含后缀eg:my.png
// 来到哦这个函数，说明项目已经正常启动了，config也通过了校验，可以完全信任
pub fn handle_unzip_file(zip_path:&Path, name:&str,output_config:&OutputItem) -> Result<()>{
    let handler = || -> Result<()> {
        // 校验文件名
        check_name_limit(name,&output_config).map_err(|e|anyhow::anyhow!(e))?;
        // 检验zip文件
        check_zip_file(zip_path).map_err(|e|anyhow::anyhow!(e))?;
        // 校验目录是否已存在
        check_file_exists(output_config,name).map_err(|e|anyhow::anyhow!(e))?;

        let file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(file).map_err(|_|anyhow::anyhow!("无效的压缩包格式"))?;

        // 收集文件，过滤目录
        let mut files = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            if file.is_file(){
                files.push(file.name().to_string());
            }
        }

        // 映射name -》 file
        let mut name_file_map = HashMap::new();
        let mut zip_format = output_config.zip_format.clone();
        zip_format.sort_by(|o1,o2|o2.len().cmp(&o1.len()));
        for format_path in &zip_format{
            let mut tmp = false;
            for file in &files{
                // println!("{:?}   ->   {:?}",format_path,file);
                if matches_parent_dir(format_path,file,&mut name_file_map){
                    tmp = true;
                    break;
                }
            }
            if !tmp{ anyhow::bail!(format!("ZIP 文件内容格式与配置文件({})不符合", output_config.name))}
        }

        // 将压缩包解压到指定目录
        for i in 0..output_config.format.len(){
            let output_path_str = format!("{}/{}/{}",&output_config.base_path,&output_config.format[i],name);
            let output_path = Path::new(&output_path_str);
            // 服务器启动就会验证base_path是否存在，所以只需要验证base_path后面的目录是否存在即可
            // 不存在就创建全部的父级目录
            if let Some(parent) = output_path.parent(){
                fs::create_dir_all(parent)?;
            }

            // 这里能保证肯定可以从map中拿到文件名,直接unwrap
            let file_name = &output_config.zip_format[i];
            let mut file = archive.by_name(name_file_map.get(file_name).unwrap())?;

            // 有多少读多少
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            fs::write(output_path,contents)?
        }

        Ok(())
    };

    match handler() {
        Ok(ok) => { record(true);Ok(ok) }
        Err(e) => { record(false);Err(e) }
    }
}

fn record(is_success:bool){
    // 获取时间
    let time = Local::now();
    if let Err(_e) = add_record(Record::new(generate_key(), is_success, time.timestamp_millis())){
        // todo 失败日志
    }
}

// 校验是否已经存在目标文件
pub fn check_file_exists(output_config:&OutputItem,name:&str) -> Result<(),String>{
    for format in &output_config.format{
        let output_path_str = format!("{}/{}/{}",&output_config.base_path,format,name);
        let output_path = Path::new(&output_path_str);
        if output_path.exists(){
            return Err(format!("文件已存在: {}", output_path.display()).into());
        }
    }
    Ok(())
}


// 校验文件名后缀限制
fn check_name_limit(name:&str,output_config:&OutputItem) -> Result<(),String>{
    match output_config.format_limit.iter().find(|e| name.ends_with(*e)) {
        None => Err(format!("文件名格式不正确: {}", name).into()),
        Some(_) => Ok(())
    }
}

// 校验zip文件是否存在
fn check_zip_file(zip_path:&Path) -> Result<(),String>{
    if !zip_path.exists(){
        return Err(format!("ZIP 文件不存在: {}", zip_path.display()).into());
    }
    Ok(())
}



// 辅助函数，判断一个路径是否属于另一个路径
fn matches_parent_dir(
    parent: &str,
    file: &str,
    map: &mut HashMap<String, String>,
) -> bool {

    let rule = Path::new(parent);
    let file_path = Path::new(file);

    let filename = match file_path.file_name().and_then(|s| s.to_str()) {
        Some(v) => v,
        None => return false,
    };

    // ---------- 解析规则 ----------
    let mut depth = 0usize;
    let mut suffix: Option<&str> = None;

    if parent.contains("**") {
        // wildcard 模式
        if let Some(idx) = parent.find("**") {
            suffix = Some(&parent[idx + 2..]);
            let dir = &parent[..idx];
            depth = Path::new(dir)
                .components()
                .filter(|c| matches!(c, std::path::Component::Normal(_)))
                .count();
        }
    } else {
        // 普通目录模式
        depth = rule
            .components()
            .filter(|c| matches!(c, std::path::Component::Normal(_)))
            .count();
    }

    // ---------- 后缀匹配 ----------
    if let Some(sfx) = suffix {
        if !filename.ends_with(sfx) {
            return false;
        }
    }

    // ---------- 深度匹配 ----------
    let ancestors: Vec<_> = file_path.ancestors().collect();

    if depth >= ancestors.len() {
        return false;
    }

    // depth=0 表示同级
    let target = ancestors[depth];

    if target.file_name().is_some() || depth == 0 {
        map.insert(parent.to_string(), file.to_string());
        return true;
    }

    false
}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_unzip_file(){
        // let mut list = [".png","@2x.png","@3x.png"];
        // list.sort_by(|o1,o2|o2.len().cmp(&o1.len()));
        // println!("{:?}",list);
        // return;
        let zip_path = Path::new("test/test.zip");
        let name = "my.png";
        let item = OutputItem{
            name: "测试名字".to_string(),
            description: "描述哈哈哈".to_string(),
            base_path: "test".to_string(),
            format: vec![String::from("v1"), String::from("v2"), String::from("v3")],
            zip_format: vec![String::from("hdpi"), String::from("mdpi"), String::from("xhdpi")]/*.iter().map(|e|format!("{}/h5_head_theme_text.png",e)).collect()*/,
            // format: vec![String::from("vv1"),String::from("vv2"),String::from("vv3")],
            // zip_format: vec![String::from("."),String::from("v1"),String::from("v2")],
            format_limit: vec![],
        };
        handle_unzip_file(zip_path,name,&item).unwrap();
    }
}