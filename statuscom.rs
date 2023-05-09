use crate::vcs_state_manager::{self, get_current_branch, get_current_commit};
//use crate::repo_file_manager::{self, get_list_of_files_from_repo, get_changing_files_and_new_from_comit};
use crate::initcom;

use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use std::env;
use sha1::{Sha1, Digest};
use std::io::Read;

pub fn gain_data_for_status() {
    let cur_cur_branch = get_current_branch();
    let cur_cur_commit = get_current_commit();
    println!("On branch {}", cur_cur_branch);
    println!("Changes to be commited: ");

    // необхоимо пройтись по всем не закомиченным файлам на текущий момент 
    // убери все в objects иначе ты считаешь лищгие джейсоны
    // считаем данный из текущего комммита 

    let file_with_state = File::open(cur_cur_commit).unwrap();
    let commit_json: serde_json::Value = serde_json::from_reader(file_with_state).unwrap();

    let _bytes = serde_json::to_vec(&commit_json).unwrap();
    let string = serde_json::to_string(&commit_json).unwrap();

    // здесь лежит закомиченные последние кайфовые файлы
    let object_for_vec_from_commit: initcom::Commit = serde_json::from_str(&string).unwrap();

    // теперь считаем все файлы из репозитория
    let mut vec_of_not_commited_files: Vec<(String, [u8; 20])> = Vec::new();

    let mut answer_of_need_path: PathBuf = PathBuf::default();
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

        // let mut part_of_pathing = path_to_cut_cut_dir.parent().unwrap();
        // path_to_cut_cut_dir = part_of_pathing.to_path_buf();
        // // path_to_cut_cut_dir = path_to_cut_cut_dir.to_parent().unwrap().to_path_buf();
        // let mut parh_try_to_create_dir_vcs = path_to_cut_cut_dir;
        // parh_try_to_create_dir_vcs.join(".vcs");
        // if parh_try_to_create_dir_vcs.is_dir() {
        //     flag_on_underfolter_is_vcs = true;
        // }
    }

    // теперь нам нужно пройтись по этой дирректории

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with(".vcs"))
             .unwrap_or(false)
    }
    
    let walker = WalkDir::new(answer_of_need_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let path_to_each_file_of_repo = entry.unwrap().path().display().to_string();
        
        let mut file = File::open(path_to_each_file_of_repo.clone()).unwrap();
        let mut content: Vec<u8> = Vec::new();
        file.read_to_end(&mut content).unwrap();

        let mut hasher_for_each_file = Sha1::new();
        hasher_for_each_file.update(content);
        let result: [u8; 20] = hasher_for_each_file.finalize().to_vec().try_into().unwrap();
        // let result = format!("{:x}", hasher_for_each_file.finalize());
        
        vec_of_not_commited_files.push((path_to_each_file_of_repo, result));
    }

    let mut modifited_path: Vec<String> = vec![];
    let mut added_path: Vec<String> = vec![];
    for i in 0..object_for_vec_from_commit.files.len() {
        for j in 0..vec_of_not_commited_files.len() {
            if object_for_vec_from_commit.files[i].0 == vec_of_not_commited_files[j].0 { // проверь что здесь оба пути сравнивабтся либо без / в конце или оба с ним
                for k in 0..20 {
                    if object_for_vec_from_commit.files[i].1[k] != vec_of_not_commited_files[j].1[k] {
                        // значит произошло модифайд
                        modifited_path.push(object_for_vec_from_commit.files[i].0.clone());
                        break;
                    }
                }
            }
        }
    }

    for i in 0..vec_of_not_commited_files.len() {
        let mut counter_for_add = 0;
        for j in 0..object_for_vec_from_commit.files.len() {
            if object_for_vec_from_commit.files[j].0 == vec_of_not_commited_files[i].0 {
                counter_for_add += 1;
                break;
            }
        }

        if counter_for_add == 0 {
            // значит такого файла не встретилось 
            added_path.push(vec_of_not_commited_files[i].0.clone()) // проверитть что пушаем именно со слешом или без тоже /
        }
    }

    for i in modifited_path {
        println!("    modified: {}", i);
    }
    for i in added_path {
        println!("    added: {}", i)
    }
}