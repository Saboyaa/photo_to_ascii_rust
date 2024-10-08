extern crate image;

use std::f32;
use std::path::Path;

use crate::image::GenericImageView;
use image::Pixel;

fn init(input_path: &str) {
    let img = image::open(&Path::new(input_path)).unwrap();
    let string = [" ","`",".","-","'",":","_",",","^","=",";",">","<","+","!","r","c","*","/","z","?","s","L","T","v",")","J","7","(","|","F","i","{","C","}","f","I","3","1","t","l","u","[","n","e","o","Z","5","Y","x","j","y","a","]","2","E","S","w","q","k","P","6","h","9","d","4","V","p","O","G","b","U","A","K","X","H","m","8","R","D","#","$","B","g","0","M","N","W","Q","%","&","@"];
    let scale:[f32;92] = [0.0, 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852, 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747, 0.3838, 0.3921, 0.396, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562, 0.458, 0.461, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602, 0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 0.9999];
    // let img_width = img.dimensions().0;
    // let img_height = img.dimensions().1;
let mut pixel_line:u32 = 1;
    for p in img.pixels() {

         if (p.1 != pixel_line) && (p.1%6==0) {print!("\n")};
         pixel_line = p.1;
         let mut index = 0;
         for i in 0..92 {if scale[i]*256.0< (p.2.to_luma()[0] as f32) {index = i;}}
         if (p.1%6==0) && (p.0%3==0) {print!("{}", string[(index as u8) as usize])}; 
}
}

fn main() {
    init("photos./download.png");
}