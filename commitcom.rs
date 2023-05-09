use crate::repo_file_manager;
use crate::vcs_state_manager;
use crate::initcom;



use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use sha1::{Sha1, Digest};


struct FillData {
    case: String,
    message: String,
}

pub fn gain_data_for_commit(case: String, message: String) {
    let mut labour_item_for_init: FillData = FillData { case: (String::new()), message: (String::new()) };
    labour_item_for_init.case = case.clone();
    labour_item_for_init.message = message.clone();

    //ПРОВЕРЯЮ НА ОШИБКУ

    let cur_cur_commit = vcs_state_manager::get_current_commit();
    let mut cur_cur_branch = vcs_state_manager::get_current_branch();
    cur_cur_branch += ".json";

    let file_with_import_branch = File::open(cur_cur_branch.clone()).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file_with_import_branch).unwrap();

    let string = serde_json::to_string(&json).unwrap();

    let object_for_branch_json: initcom::Branch = serde_json::from_str(&string).unwrap();

    let mut nu_perevedem_v_string_v_ruschnuy = object_for_branch_json.hash_of_last_commit.clone();
    // let mut nu_perevedem_v_string_v_ruschnuy = String::from("");
    // for i in 0..20 {
    //     nu_perevedem_v_string_v_ruschnuy += object_for_branch_json.hash_of_last_commit[i].to_string().as_ref();
    // }

    let hash_in_branch_to_string = nu_perevedem_v_string_v_ruschnuy.clone() + ".json";

    if hash_in_branch_to_string != cur_cur_commit {
        println!("You can create a new commit only from last one.");
        println!("Aborting...");
        return;
    }

    // НАДО ЕЩЕ ПРОВЕРИТЬ НА НЕИЗМЕННОСТЬ 


    // // СНАЧАЛА Я СФОРМИРУЮ НОВЫЙ КОМИТ И ДОБАВЛЮ ЕГО ВО ВСЕ ДЖЕЙСОНЫ ТОЛЬКО ПОТОМ В ДИР КОММИТС

    // ВЫЗЫВАЮ ЗДЕСЬ МЕТОД ИЗ РЕПО МЕНЕДЖЕРА
    repo_file_manager::make_file_commit_with_repo_files(&message);

    // ОСТАЛОСЬ ВЫВЕСТИ КОММЕНТ К КОММИТУ
    //     [<branch_name> <commit_hash>] Work in progress                                               
    //   3 files changed, 1 added
    //   modified: path/to/modified/file.rs
    //   modified: path/to/modified/file2.rs
    //   added: path/to/new/file.rs

    // И ЭТО МНЕ КАКИМ-ТО МАКАРОМ ПРИХОДИТСЯ ДЕЛАТЬ В РЕПО МЕНЕДЖЕРЕ
    // И КАКОЙ ВООБЩЕ ХЭШ БУДЕТ ИМЕТЬ ТАКУЩИЙ КОММИТ, ОТ ЧЕГО СЧИАТЬ ХЕШ 
    
}