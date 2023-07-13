/*
 * @Author: ralph zzltianhong@gmail.com
 * @Date: 2023-07-13 20:54:43
 * @LastEditors: ralph zzltianhong@gmail.com
 * @LastEditTime: 2023-07-13 22:01:42
 * @FilePath: \rust_learning\hello-rust\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
    let stdout = stdout();
    let message = String::from("hello World!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    println!("Hello World!\n");
}
