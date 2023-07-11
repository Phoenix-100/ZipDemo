use std::fs::File;
use std::io::{self, BufReader, Write};
use zip::read::ZipArchive;

//获取压缩包文件列表
fn _list_files(zip_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut zip = ZipArchive::new(BufReader::new(file))?;

    for i in 0..zip.len() {
        let file: zip::read::ZipFile<'_> = zip.by_index(i)?;
        println!("文件名: {}", file.name());
    }

    Ok(())
}

//提取文件
fn _extracting_files(
    zip_path: &str,
    file_name: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut zip = ZipArchive::new(BufReader::new(file))?;

    let mut found = false;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name() == file_name {
            let mut output_file = File::create(output_path)?;
            std::io::copy(&mut file, &mut output_file)?;
            found = true;
            break;
        }
    }

    if found {
        Ok(())
    } else {
        Err("在压缩包中未找到此文件".into())
    }
}

fn main() {
    loop {
        print!("1.获取列表\r\n2.提取文件\r\n");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let a = input.trim();
        println!("你输入的内容是：{}", input.trim());

        // 在此处可以根据输入内容执行其他操作或退出循环

        match a {
            "1" => {
                print!("请输入zip压缩包路径：\r\n");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let zip_path = input.trim();
                println!("你输入zip压缩包路径：{}\r\n", zip_path.trim());

                let zip_path = zip_path.trim();

                if let Err(e) = _list_files(zip_path) {
                    eprintln!("读取失败: {}", e);
                    return;
                }
            }
            "2" => {
                print!("请输入zip压缩包路径：\r\n");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let zip_path = input.trim();
                println!("你输入zip压缩包路径：{}\r\n", zip_path.trim());

                let zip_path = zip_path.trim();

                print!("需要提取的文件名：\r\n");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let file_name = input.trim();
                println!("你输入的内容是：{}\r\n", zip_path.trim());

                let output_path = file_name;

                if let Err(e) = _extracting_files(zip_path, file_name, output_path) {
                    eprintln!("提取文件失败: {}", e);
                } else {
                    println!("提取成功");
                }
            }
            _ => {
                println!("输入有误");
                // 这里是其他情况下执行的代码块
            }
        }
    }
}
