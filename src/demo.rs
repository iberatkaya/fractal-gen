use std::io::Write;

pub fn write_image(path: &str){
    let mut data = std::fs::File::create(path).unwrap();
    println!("{:?}", data);
    let mut bdata: Vec<u8> = vec!();
    //File Type Data
    //File Type
    bdata.push('B' as u8);
    bdata.push('M' as u8);  
    //File Size
    bdata.push(0); //Initially 0  
    bdata.push(0);  
    bdata.push(0);  
    bdata.push(0);
    //Res  
    bdata.push(0);  
    bdata.push(0);
    //Res  
    bdata.push(0);  
    bdata.push(0);
    //PixelDataOffset  
    bdata.push(54);  
    bdata.push(0);  
    bdata.push(0);  
    bdata.push(0);  
    //Header Size 
    bdata.push(40);  
    bdata.push(0);  
    bdata.push(0);  
    bdata.push(0);  
    //Img Width
    bdata.push(2);  
    bdata.push(0);  
    bdata.push(0);  
    bdata.push(0);  
    //Img Height
    bdata.push(1); 
    bdata.push(0);  
    bdata.push(0);  
    bdata.push(0);  
    //Planes 
    bdata.push(1);  
    bdata.push(0);  
    //BitsPerPixel
    bdata.push(24);  
    bdata.push(0);
    //Compression
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    //ImageSize
    bdata.push(1);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    //xPerMeter
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    //yPerMeter
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    //colorMapEntriesUsed
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    //sigColors
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    //Pixels
    bdata.push(0);
    bdata.push(0);
    bdata.push(255);
    bdata.push(0);
    bdata.push(255);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata.push(0);
    bdata[2] = bdata.len() as u8;
    data.write(&bdata).unwrap();
}