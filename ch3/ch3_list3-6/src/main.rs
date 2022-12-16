static mut ERROR: i32 = 0;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let mut f = File::new("something.text");
    f.read(buffer);
    unsafe {
        if ERROR != 0 {
            panic!("ファイルを呼んでいるときにエラーが発生しました")
        }
    }
    close(&mut f);
    unsafe {
        if ERROR != 0 {
            panic!("ファイルを閉じているときにエラーが発生しました")
        }
    }
}
