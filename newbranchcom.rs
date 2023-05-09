use crate::vcs_state_manager::{self, get_current_branch, get_current_commit};
use crate::initcom::{self, Branch};

use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use std::env;
use sha1::{Sha1, Digest};
use std::io::Read;
use std::fs;

struct FillData {
    case: String,
    br_name: String,
}

pub fn gain_data_for_new_branch(case: String, br_name: String) {
    let mut labour_item_for_init: FillData = FillData { case: (String::new()), br_name: (String::new()) }; // хочу показать для себя явно, что они пустые
    labour_item_for_init.case = case.clone();
    labour_item_for_init.br_name = br_name.clone();

    // Ответвляет новый бранч от текущего коммита в мастере. В нашей системе контроля версий ответвляться можно только от мастера. В случае, если в репозитории есть незакоммиченные изменения, также переносит их в новый бранч.
    // 1. ОБРАБОТАЕМ ОШИБКИ
    // 1.1 ошибка - если текущий бранч не мастер:
    // для этого берем кюрент бранч и сравниваем его с мастер
    let current_br = vcs_state_manager::get_current_branch();
    if current_br != "master" {
        println!("Creating a new branch is possible only when you are in the master branch.");
        println!("Aborting...");
        return;
    }

    //1.2 ошибка - В случае если бранч уже существует:
    // получается проходимся по списку бранчей и смотрим нет ли там нашего бренча
    let mut answer_of_need_path: PathBuf = PathBuf::default(); // <- проверь дефолт 
    let path_to_cut_cut_dir = env::current_dir().unwrap();
    let mut flag_on_underfolter_is_vcs = false;
    let new_try_path_all = path_to_cut_cut_dir.clone();
    new_try_path_all.join(".vcs");
    if new_try_path_all.is_dir() {
        flag_on_underfolter_is_vcs = true;
        answer_of_need_path = path_to_cut_cut_dir.clone();
    }
    while !flag_on_underfolter_is_vcs {
        let new_try_path = path_to_cut_cut_dir.parent().unwrap().to_path_buf();
        let new_tr_path_with_vcs = new_try_path.clone();
        new_tr_path_with_vcs.join(".vcs");
        if new_tr_path_with_vcs.is_dir() {
            flag_on_underfolter_is_vcs = true;
            answer_of_need_path = new_try_path.clone();
        }
    }

    let copas = answer_of_need_path.clone();
    
    let path_to_commit_list = copas.join(".vcs").join("branch_list");

    let file_with_state = File::open(path_to_commit_list).unwrap();

    let commit_json: serde_json::Value = serde_json::from_reader(file_with_state).unwrap();

    let string = serde_json::to_string(&commit_json).unwrap();

    let object_of_branches: initcom::BranchList = serde_json::from_str(&string).unwrap();

    for i in object_of_branches.branch_name {
        if br_name == i { // бранч уже сюществует
            println!("Branch branch_name already exists.");
            println!("Aborting...");
            return;
        }
    }
    // ВОЗМОЖНО ЕЩЕ НАДО ПРОВЕРИТЬ, ЧТО ЕСТЬ БРАНЧ, НО В НЕМ НЕТ КОММИТОВ -> ДЛЯ ЭТОГО НУЖНО ПРОЙТИСЬ ПО СПИСКУ КОММИТОВ И ПОНЯТЬ, ЧТО НИ ОДИН ИЗ НИХ НЕ ЛЕЖИТ В br_name
   
    
    //2. ПОСЛЕ ОБРАБОТКИ ДВУХ ОШИБОК РЕАЛИЗУЕМ ОТВЕТВЛЕНИЕ ОТ МАСТЕРА
    // НАМ НУЖНО СОЗДАТЬ НОВЫЙ БРЕНЧ С ТЕМ ИМЕНЕМ КОТОРОЕ К НАМ ПРИШЛО, ЗАКИНУТЬ ЭТОТ БРЕНЧ В ЛИСТ БРЕНЧЕЙ И ПОМЕНЯТЬ СТЕЙТ
    //2.1 создадим новый бренч и запишем в него новое имя и в последний коммит засунем тот от которого сейчас ответвляемся

    // let new_path_branch = PathBuf::from(&main_labour_dir).join("master.json");
    // let object_for_write_branc_1: Branch = Branch { branch_name: (String::from("master")), hash_of_last_commit: (hasher_for_commit_1.finalize().to_vec().try_into().unwrap()) };
    
    // let str_of_elemnts_for_branch_1 = serde_json::to_string(&object_for_write_branc_1).unwrap();
    // std::fs::write(new_path_branch, str_of_elemnts_for_branch_1).unwrap();

    let copas = answer_of_need_path.clone();
    let create_name_of_branch_file = br_name.clone() + ".json";
    let path_to_branch = copas.join(".vcs").join(create_name_of_branch_file);

    let cur_commit = vcs_state_manager::get_current_commit();
    let hash_wout_json = cur_commit[..cur_commit.len() - 5].to_string();

    let object_for_new_branch: initcom::Branch = initcom::Branch {branch_name: (br_name.clone()), hash_of_last_commit: (hash_wout_json.clone()), hash_of_otvetvlen_commit: (hash_wout_json.clone())};

    let str_of_elemnts_for_branch = serde_json::to_string(&object_for_new_branch).unwrap();
    std::fs::write(path_to_branch, str_of_elemnts_for_branch).unwrap();

    // 2.2 закинем в лист бренчей
    let copas = answer_of_need_path.clone();
    let path_to_lsit_branch = copas.join(".vcs").join("branch_list.json");
    
    let file_with_branches = File::open(path_to_lsit_branch.clone()).unwrap();
    let branch_json: serde_json::Value = serde_json::from_reader(file_with_branches).unwrap();
    let string = serde_json::to_string(&branch_json).unwrap();
    let mut object_for_vec_from_branch_list: initcom::BranchList = serde_json::from_str(&string).unwrap();

    object_for_vec_from_branch_list.branch_name.push(br_name.clone());

    let string =  serde_json::to_string(&object_for_vec_from_branch_list).unwrap();
    std::fs::write(path_to_lsit_branch, string).unwrap();

    //2.3 поменяем стейт 
    let cur_commit = vcs_state_manager::get_current_commit();
    let object_for_state_json: initcom::State = initcom::State { current_comit_hash: (cur_commit), current_branch_name: (br_name) };

    let str_of_elements_for_state = serde_json::to_string(&object_for_state_json).unwrap();

    let copas = answer_of_need_path.clone();
    let path_to_state = copas.join(".vcs").join("state.json");

    std::fs::write(path_to_state, str_of_elements_for_state).unwrap();
}

