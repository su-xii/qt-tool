use std::fs::File;
use std::io::{Read, Write};
use zip::ZipArchive;
use crate::cli::Cli;
use crate::handler::Handler;

pub struct CliHandler;

impl Handler for CliHandler{
    async fn run(cli:&Cli) -> anyhow::Result<()> {
        // 命令行模式需要参数 压缩包地址 输出文件名（my.png) -p path输出路径  -o [输出的文件目录（需要对应压缩包的文件，按照从小到大顺序排列）]
        // eg: qt test.zip ok.png -p ./test -o [v1 v2 v3]
        let file = File::open(cli.zip_path().expect("请指定压缩包地址"))?;
        let base_path = cli.output_dir().expect("请指定输出文件目录");
        let mut archive = ZipArchive::new(file).map_err(|_|anyhow::anyhow!("无效的压缩包格式"))?;
        let mut file_entitys = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|_|anyhow::anyhow!("无效的压缩包格式"))?;
            if file.is_file() {
                let name = file.name().to_string();
                let size = file.size();
                file_entitys.push((name, size));
            }
        }
        if file_entitys.len() != cli.output_dirs().len(){ return Err(anyhow::anyhow!("输入的输出文件目录数量与压缩包文件数量不一致")) }
        file_entitys.sort_by(|(_,a), (_,b)|a.cmp(b));

        // 解压到指定目录
        for (index,dir_name) in cli.output_dirs().iter().enumerate() {
            let dir_path = base_path.join(dir_name);
            if !dir_path.exists() {
                std::fs::create_dir_all(&dir_path)?;
            }
            let (file_name, _) = &file_entitys[index];
            let mut file = archive.by_name(file_name).map_err(|_|anyhow::anyhow!("无效的压缩包格式"))?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            let out_file_path = format!("{}/{}", dir_path.to_str().unwrap(),cli.output_name().unwrap());
            let mut file = File::create(out_file_path)?;
            file.write_all(&buf)?
        }

        Ok(())
    }
}