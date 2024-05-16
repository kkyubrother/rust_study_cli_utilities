use std::{env, process, fs, path};

fn cmd_echo() {
    let args: Vec<_> = env::args().collect();

    // Echo
    if args.len() > 2 && args[1].eq("echo") {
        println!("{}", args[2]);
    } else if args.len() == 2 && args[1].eq("echo")  {
        println!();
    } else {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    } else {
        println!("No argument");
        process::exit(0);
    }

    if args[1].eq("echo") {
        cmd_echo()
    }
    else if args[1].eq("cat") {
        if args[2].eq("--help") || args[2].eq("-h") {
            println!("사용 방법: cat [OPTION]... [FILE]...
FILE을 표준 출력에 연결.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help     display this help and exit
      --version  output version information and exit

Examples:
  cat f - g  Output f's contents, then standard input, then g's contents.
  cat        Copy standard input to standard output.");
            process::exit(0);
        }

        println!("command cat");

        let (_, right) = args.split_at(2);
        for item in right {
            println!("item: {}", item);
        }

        let file_path = path::Path::new(&args[2]);

        if file_path.exists() {
            let contents = fs::read_to_string(&args[2]).expect("파일을 찾을 수 없습니다!");
            println!("{}", contents);
        }

    }
}