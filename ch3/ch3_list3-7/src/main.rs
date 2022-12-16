use rand::random;

static mut ERROR: isize = 0;

#[derive(Debug)]
struct File;

fn read(file: &File, save_to: &mut Vec<u8>) -> usize {
    if random::<bool>() && random::<bool>() && random::<bool>() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}
#[warn(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];
    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("ファイルを呼んでいるときにエラーが発生しました")
        }
    }
}
