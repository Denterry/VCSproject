use serde_derive::{Deserialize, Serialize};
use chrono::{self, Local, DateTime};

use sha1::{Sha1, Digest};
// use std::io::Read;
use std::fs;

use  std::path::PathBuf;
// use  std::path::Path;
// use clap::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Commit { // структруа нашего комит  файла   
    pub title: String,
    pub prev_commit: String/*[u8; 20]*/,
    pub files: Vec<(String/* PATH*/, [u8; 20])>,
    pub branch_where_commit: String,
    pub time_when_was_build: chrono::DateTime<Local>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Branch { // структруа нашего бренч  файла   
    pub branch_name: String, // = "main" or "branch"
    pub hash_of_last_commit: String, /*[u8; 20]*/ // хеш последнего коммита
    pub hash_of_otvetvlen_commit: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommitList { // структруа нашего листа из комитов файла   
    pub commit_name: Vec<String>, // вектор из хешов всех коммитов
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BranchList { // структруа нашего листа из бренчей файла   
    pub branch_name: Vec<String>, // вектор из именов всех бранчей 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct State { // текущие комит и бренч
    pub current_comit_hash: String,
    pub current_branch_name: String,
}

struct FillData{
    case: String,
    path: String,
}

fn try_to_work_with_dir(args: &FillData) -> std::io::Result<()> {
    // мы можем создать сразу полноценный путь до папки .vcs -- >
    let full_path_to_vcs = args.path.clone() + "/" + ".vcs";
    // println!("{}", full_path_to_vcs);
    fs::create_dir_all(full_path_to_vcs)?;
    Ok(())
}

pub fn gain_data_for_init(case: String, path: String) {
    let mut labour_item_for_init: FillData = FillData { case: (String::new()), path: (String::new()) }; // хочу показать для себя явно, что они пустые
    labour_item_for_init.case = case.clone();
    labour_item_for_init.path = path.clone();

    let result_of_making_dir = try_to_work_with_dir(&labour_item_for_init);
    match result_of_making_dir {
        Result::Ok(()) => println!("$ vcs init --path {}", path), // то он создал полноценны путь до папки точка вкс и вернул Ок(())
        Result::Err(_) => println!("Dir with folder was done before this init"), // есть разные ситуации, почему вернул еррор, но в нашем слечае из-за того, что такая папка уже  есть ну и путь до нее соответсвенно
    }

    // надо сделать тут комит:
    // так будет выглядеть мой json, который по моему предположению будет являться комитом, то есть мы хешируем json и этим хэшом называем мой комит файл(этот же json)
    // view of json file:
    // title: xxx // 
    // prev_commit: xxx // нам необходимо хранить ссылки на предыдущие комиты именно здесь для того, чтобы отследживать все изменения 
    // files: [
    // {path, hash}, // path - исходное название файла, hash - это тот же файл(имя того же файла), мы от него посчитали хэш, но он уже лежит в хранилище
    // {path, hash},
    // {path, hash},    
    //]
    // у нас получается дерево коммитов
    // у нас все объекты(файлы, за изминением которых мы следим; коммиты теги;) хранятся в директории(хранилище) objects ===>
    // let full_path_to_vcs_to_objects = path.clone() + "/" + ".vcs" + "/objects"; // - > здесь мы будем хранить наши комиты и файлы
    // fs::create_dir(full_path_to_vcs_to_objects);
    // мы должны сделать коммит основываясь на файлах которые у меня чейчас есть, я их засовываю в хранилище вместе  с хешами
    // у меня как итог в object лежат:
    //                 /.vcs/objects
    //                     (хеш коммита).json
    //                     какой - то файл data.txt
    //                     какой - то файл memory.txt
    //                     какой - то файл answer.txt
    // В каждой команде меняем стейт репозитория (служебные файлы в папке .vcs), а значит будем менять и здесь поехали:
    // В зависимости от этой команды мы поменяем или не поменяем  файлы в самом репозитории где-то в .vcs
    // мы вызвали init и передали сюда case and path
    // то есть сначала меняем стейт потом файлы в самом репе и в конце выводим отчет
    // то есть в этой команде мы обязаны поменять стейт в папке vcs (служебные файлы в этой папке)
    // C ЭТОГО МОМЕНТА Я ИСПОЛЬЗУЮ PATH А НЕ ОБЫЧНЫЙ СТРИНГ ДЛЯ УДОБСТВА РАБОТЫ С ФАЙЛАМИ

    let main_labour_dir = labour_item_for_init.path.clone() + "/" + ".vcs" + "/"; // мы уже создали эту дирректорию 

    let file_path_to_branch_list = PathBuf::from(&main_labour_dir).join("branch_list.json");
    let file_path_to_commit_list = PathBuf::from(&main_labour_dir).join("commit_list.json");
    let file_path_to_state = PathBuf::from(&main_labour_dir).join("state.json");
    
    let commits_labour_dir = main_labour_dir.clone() + "commits";
    // let path_to_commits_labour_dir = Path::new(&commits_labour_dir);
    fs::create_dir(commits_labour_dir.clone()).unwrap(); // у меня он не может быть Err, потом если что пофикшу // мы создали дир commits

    // имеем вид
    // .vcs
    // ├── branch_list.json
    // ├── commit_list.json
    // ├── /commits
    // └── state.json

    // let new_path_commit = PathBuf::from(&main_labour_dir).join("commit_1.json");
    let object_for_write_comit_1: Commit = Commit { title: (String::from("Initial commit")), prev_commit: (String::from("")), files: (vec![]), branch_where_commit: (String::from("master")), time_when_was_build: (Local::now())};

    let str_of_elemnts_for_commit_1 = serde_json::to_string(&object_for_write_comit_1).unwrap();
    
    let mut hasher_for_commit_1 = Sha1::new();
    hasher_for_commit_1.update(&str_of_elemnts_for_commit_1);
    let hash = format!("{:x}", hasher_for_commit_1.clone().finalize());
    let hash_with_json = hash.clone() + ".json";
    let new_path_commit = PathBuf::from(&main_labour_dir).join(&hash_with_json);
    std::fs::write(new_path_commit, str_of_elemnts_for_commit_1).unwrap();

    // тоже самое только для бранча
    let new_path_branch = PathBuf::from(&main_labour_dir).join("master.json");
    let object_for_write_branc_1: Branch = Branch { branch_name: (String::from("master")), hash_of_last_commit: (hash.clone()/*hasher_for_commit_1.finalize().to_vec().try_into().unwrap()*/), hash_of_otvetvlen_commit: (String::from(""))};
    
    let str_of_elemnts_for_branch_1 = serde_json::to_string(&object_for_write_branc_1).unwrap();
    std::fs::write(new_path_branch, str_of_elemnts_for_branch_1).unwrap();

    // и наконец записываем все это в конкретные json - ы и директории 

    let mut object_for_branch_list: BranchList = BranchList { branch_name: (vec![]) };
    object_for_branch_list.branch_name.push(object_for_write_branc_1.branch_name.clone());

    let str_of_elements_for_branch_list =  serde_json::to_string(&object_for_branch_list).unwrap();
    std::fs::write(file_path_to_branch_list, str_of_elements_for_branch_list).unwrap();

    let mut object_for_commit_list: CommitList = CommitList { commit_name: (vec![]) };
    object_for_commit_list.commit_name.push(hash_with_json.clone()); // добавляю таккие элементы: " хэш.json"

    let str_of_elements_for_commit_list = serde_json::to_string(&object_for_commit_list).unwrap();
    std::fs::write(file_path_to_commit_list, str_of_elements_for_commit_list).unwrap();

    // решил не тянуть время и сразу инициалиировать // не надо + ".json"   // мне всегда придется добавлять + ".json" |
    let object_for_state: State = State { current_comit_hash: (hash_with_json.clone()), current_branch_name: (object_for_write_branc_1.branch_name.clone()) }; 

    let str_of_elements_for_state = serde_json::to_string(&object_for_state).unwrap();
    std::fs::write(file_path_to_state, str_of_elements_for_state).unwrap();


    // осталось создать пустую дир в  committs
    let to_firs_comit = commits_labour_dir + "/" + hash.as_ref();
    fs::create_dir(to_firs_comit.clone()).unwrap();

    println!("Initialized VCS repository in {}", path);
    println!("Created commit:");
    println!("[ master {} ] Initial commit", hash);
}