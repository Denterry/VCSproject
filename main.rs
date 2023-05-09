mod initcom;
mod statuscom;
mod commitcom;
mod jumptobranchcom;
mod jumptocomitcom;
mod newbranchcom;
mod mergecom;
mod logcom;

mod repo_file_manager;
mod vcs_state_manager;

use clap::{Parser /*, Subcommand */};
// use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    typecomand: Option<String>,
    case: Option<String>,
    path: Option<String>,

    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

// #[derive(Subcommand)]
// enum Commands {
//     init, // Создает в директории <directory path> папку .vcs, которая будет содержать метаинформацию по репозиторию, включающему все поддерево папки directory. Создает коммит с сообщением Initial commit
//     status, // Выводит в терминал текущий статус 
//     commit, // Формирует новый коммит с сообщением message из текущих изменений.
//     jump, // to comit - Переносит репозиторий в коммит с хешом <commit_hash>.
//     jump, // Переносит репозиторий в последний коммит бранча <branch_name>.
//     new_brnch, // Ответвляет новый бранч от текущего коммита в мастере. В нашей системе контроля версий ответвляться можно только от мастера. В случае, если в репозитории есть незакоммиченные изменения, также переносит их в новый бранч.
//     merge, // Вливает изменения из бранча в мастер. В нашей системе контроля версий мердж вливать можно только в последний коммит мастера, побочный бранч удаляется после мерджа.
//     log, // Выводит в терминал список от инициализации репозитория
// }

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let mut args: Args = Args::new();
    // args.subcom = &args[1];
    // args.path = &args[2];

    // cargo run --bin vcs -- typecomand case path
    
    let args = Args::parse();
    
    // println!("{:?}, {:?}, {:?}", args.typecomand.as_deref(), args.case.as_deref(), args.path.as_deref());

    // ОБРАБОТАТЬ ПОТОМ ОШИБКИ НА НЕККОРЕКТНОСТЬ ФАЙЛА, КЭША И ТД И НА ПОДАЧУ ПУСТОТЫ 
    let mut need_command: String = String::new();
    if args.typecomand != None {
        need_command = args.typecomand.unwrap();
    }
    // println!("{}", need_command);

    let mut need_case: String = String::new();
    if args.case != None {
        need_case = args.case.unwrap();
    }
    // println!("{}", need_case);

    let mut need_path: String = String::new();
    if args.path != None {
        need_path = args.path.unwrap();
    }
    // println!("{}", need_path);
    


    // ОПРЕДЕЛЯЕМ КАКГО_ТИПА ПЕРЕМЕННАЯ И ВЫЗЫВАЕМ СООТВЕТСВЮЩИЕ ФУНКЦИИ ИЗ НУЖНЫХ МОДУЛЕЙ
    // 1) Do all command
    // let init = String::from("init");
    // let status = String::from("status");
    // let commit = String::from("commit");
    // let jump = String::from("jump");
    // let new_branch = String::from("new_branch");
    // let mergei = String::from("merge");

    match need_command.as_ref() {
        "init" => {
            initcom::gain_data_for_init(need_case, need_path);
            // println!("{}", "init");
        },

        "status" => {
            statuscom::gain_data_for_status();
            // println!("{}", "status")
        },

        "commit" => {
            commitcom::gain_data_for_commit(need_case, need_path);
            // println!("{}", "commit");
        },

        "jump" => {
            if need_case == "commit" {
                jumptocomitcom::gain_data_for_jumptocommit(need_case, need_path);
            } else {
                jumptobranchcom::gain_data_for_jumptobranch(need_case, need_path);
            }
            // println!("{}", "jump");
        },

        "new_branch" => {
            newbranchcom::gain_data_for_new_branch(need_case, need_path);
            // println!("{}", "new_branch");
        },

        "merge" => {
            mergecom::gain_data_for_new_branch(need_case, need_path);
            // println!("{}", "merge");
        },

        "log" => {
            logcom::gain_data_for_log();
            // println!("{}", "log");
        },

        &_ => println!("{}", "You need to enter the existing command"),
    }
}