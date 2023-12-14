use std::env::args ;
use glob::glob;
use std::vec::Vec;
use image::{self, DynamicImage};
use rayon::prelude::*;

static mut INDEX:usize = 0;

fn get_filename_list(dir_path:&String,wild_card:&String)->Vec<String>
{
    let img_extensions
        = vec!["jpg","bmp","png","tga"];


    let mut path_lst:Vec<String> = vec![];

    for entry 
        in glob(format!("{}/{}",&dir_path ,&wild_card).as_str()).unwrap() {
        let path = entry.as_ref().unwrap();
        if img_extensions.contains(&path.extension().unwrap().to_str().unwrap().to_lowercase().as_str())
        {
            path_lst.push(entry.unwrap().to_string_lossy().to_string());
        }
    }
    return path_lst;
}


fn help_print() {
    println!("
        command name:
            img_resizer

        description:
            画像ファイルのあるフォルダのfolder_pathからwild_cardで選んだ画像を
            長辺サイズ決めそのサイズでリサイズします。

        usage:
            resizer [folder_path] [wild_card] [image_long_edge_size]
            resizer [folder_path] [wild_card] [image_long_edge_size] [suffix]

        example
            resizer  C:/Users/user/Desktop/work *.jpg 1000
            resizer  C:/Users/user/Desktop/work *.jpg 1000 _aftter_etc
            ");
} 

fn file_resize(folder_path:&String,
               wild_card:&String,
               long_edge_size: u32,
               suffix:Option<&String>
            ) {

    
    let default_string:&String = &"_aft".to_string();
    let suffix = suffix.unwrap_or(default_string);

    // 画像ファイルネーム取得
    let file_names 
        = get_filename_list(folder_path,wild_card) ;

    let max_index = file_names.len();
    unsafe
    {
      INDEX  = 0;
    }
    // 画像リサイズ
    file_names.par_iter().for_each(|file_name|{
        let img_result  = image::open(file_name);

        let img:DynamicImage;
        match img_result {
          Ok(value)=>{

          img = value;
          let width_src:f64     = img.width() as f64;
          let height_src:f64    = img.height() as f64;
          let width_dst:f64;
          let height_dst:f64 ;

          if width_src > height_src {
              width_dst  = long_edge_size as f64;
              height_dst = (long_edge_size as f64 / width_src   ) * height_src;
          } else {
              height_dst = long_edge_size as f64;
              width_dst  =(long_edge_size as f64 / height_src)  * width_src;
          }
          let dst_img = img.resize(width_dst as u32, height_dst as u32,image::imageops::FilterType::Lanczos3 );

          
          let mut result  = String::new();
          let file_name_split:Vec<&str> = file_name.split(".").collect();
          let str_len = file_name_split.len();

          for (i,&itm) in file_name_split.iter().enumerate() {
              result.push_str(itm);
              if i == str_len - 2 {
                  result.push_str(format!("{}.",suffix).as_str());
              }else if i == str_len -1{
              }else {
                  result.push_str(".");
              }
          }

          dst_img.save(&result).unwrap();

          unsafe{
          // 実行チェック
          println!("{}/{}:{}  {}x{}",
          INDEX,  max_index,  &result,
          width_dst as u32, 
          height_dst as u32);

            INDEX += 1;
          }

        },

        Err(error)=>{ 
            eprintln!("{}",error);
        }
        }});

}
            
fn main() {
    // 引数処理
    let args_lst:Vec<String> = args().into_iter().collect();
    let args_len = args_lst.len();

    match args_len {
        4 => {
            let folder_path = &args_lst[1];
            let wild_card = &args_lst[2];
            let long_edge_size = args_lst[3].parse::<u32>().unwrap();
            file_resize(folder_path, wild_card, long_edge_size,None);
        },
        5 => {
            let folder_path = &args_lst[1];
            let wild_card = &args_lst[2];
            let long_edge_size = args_lst[3].parse::<u32>().unwrap();
            let suffix= Some(&args_lst[4]);
            file_resize(folder_path, wild_card, long_edge_size,suffix);
        }
        _ =>{help_print()},
    }

}
