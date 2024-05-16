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
        if args[2].eq("--help") || args.contains(&"-h".to_string()) {
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
        let show_all = args.contains(&"-A".to_string()) || args.contains(&"--show-all".to_string());
        let show_end = args.contains(&"-E".to_string()) || args.contains(&"--show-ends".to_string()) || show_all;

        let show_line_number_nonempty = args.contains(&"-b".to_string()) || args.contains(&"--number".to_string());
        let show_line_number = if show_line_number_nonempty {
            false
        } else {
            args.contains(&"-n".to_string()) || args.contains(&"--number-nonblank".to_string())
        };


        println!("command cat");

        let (_, right) = args.split_at(2);
        for item in right {
            println!("item: {}", item);
        }

        let file_path = path::Path::new(&args[2]);

        if file_path.exists() {
            let contents = fs::read_to_string(&args[2]).expect("파일을 찾을 수 없습니다!");
            for line in contents.split("\n") {
                if show_line_number {
                    print!("\t{}\t", 1);
                } else if show_line_number_nonempty && line.len() > 0 {
                    print!("\t{}\t", 1);
                }

                print!("{}", line);
                if show_end {
                    println!("$")
                } else {
                    println!()
                }
            }
        }

    }
}