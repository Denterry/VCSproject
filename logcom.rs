
use crate::initcom;

use std::fs::File;
use std::path::PathBuf;
use std::env;

pub fn gain_data_for_log() {
    //1. СКОРЕЕ ВСЕГО ЗДЕСЬ НАМ ПРИЕДТСЯ ПРОЙТИСЬ ПО ЛИСТУ ИЗ КОММИТОВ И АНАЛИЗИРОВАТЬ КАЖДЫЙ ХЭШ КОММИТА

    // и как всегда нужно начать с подъема с кровати
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
    }

    //1.2 считаем коммит лист и возьмем от туда вектор из коммитов и пройдемся по нему

    // let mut path_to_list_commit = path_of_dir_to_vcs.clone();
    // path_to_list_commit = path_to_list_commit.join("commit_list.json");
    // let file_with_commits = File::open(path_to_list_commit.clone()).unwrap();

    // let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();

    // let string = serde_json::to_string(&json).unwrap();

    // let mut object_for_commit_json: initcom::CommitList = serde_json::from_str(&string).unwrap();
    
    // object_for_commit_json.commit_name.push(hash_with_json.clone());

    // let str_of_elements_commit_list = serde_json::to_string(&object_for_commit_json).unwrap();

    // std::fs::write(path_to_list_commit, str_of_elements_commit_list).unwrap();

    let mut path_to_commit_list = answer_of_need_path.clone();
    path_to_commit_list = path_to_commit_list.join(".vcs").join("commit_list.json");
    let file_with_commits = File::open(path_to_commit_list.clone()).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();
    let string = serde_json::to_string(&json).unwrap();
    let object_for_commits_json: initcom::CommitList = serde_json::from_str(&string).unwrap();

    for i in (object_for_commits_json.commit_name.len() - 1)..=1 {
        // let gain_hash = i[..i.len() - 5].to_string();
        // 1.3 и здесь мы должны реализовать основной функционал
        let gain_hash = object_for_commits_json.commit_name[i][0..=19].to_string();
        println!("commit {}", gain_hash);

        let file_name = object_for_commits_json.commit_name[i].clone();
        let copas = answer_of_need_path.clone();
        let do_path_to_commit_in_dir = copas.join(".vcs").join(file_name);

        // считаем этот комит
        let file_commit = File::open(do_path_to_commit_in_dir).unwrap();
        let commit_json: serde_json::Value = serde_json::from_reader(file_commit).unwrap();
        let string = serde_json::to_string(&commit_json).unwrap();
        let object_for_commit: initcom::Commit = serde_json::from_str(&string).unwrap();

        println!("Date: {}", object_for_commit.time_when_was_build);
        println!("Message {}", object_for_commit.title);

        //теперь мы должны сравнивать файлы текущего со следующим
        //считыем следующий коммит
        let next_file_name = object_for_commits_json.commit_name[i - 1].clone();
        let copas = answer_of_need_path.clone();
        let next_do_path_to_commit_in_dir = copas.join(".vcs").join(next_file_name);


        let next_file_commit = File::open(next_do_path_to_commit_in_dir).unwrap();
        let next_commit_json: serde_json::Value = serde_json::from_reader(next_file_commit).unwrap();
        let next_string = serde_json::to_string(&next_commit_json).unwrap();
        let next_object_for_commit: initcom::Commit = serde_json::from_str(&next_string).unwrap();


        // теперь сравним наборы    current     и   next    коммитов
        let mut modifited_path: Vec<String> = vec![];
        let mut added_path: Vec<String> = vec![];
        
        for i in 0..object_for_commit.files.len() {
            for j in 0..next_object_for_commit.files.len() {
                if object_for_commit.files[i].0 == next_object_for_commit.files[j].0 { // проверь что здесь оба пути сравнивабтся либо без / в конце или оба с ним
                    for k in 0..20 {
                        if object_for_commit.files[i].1[k] != next_object_for_commit.files[j].1[k] {
                            // значит произошло модифайд
                            modifited_path.push(object_for_commit.files[i].0.clone());
                            break;
                        }
                    }
                }
            }
        }
    
        for i in 0..object_for_commit.files.len() {
            let mut counter_for_add = 0;
            for j in 0..next_object_for_commit.files.len() {
                if object_for_commit.files[j].0 == next_object_for_commit.files[i].0 {
                    counter_for_add += 1;
                    break;
                }
            }
    
            if counter_for_add == 0 {
                // значит такого файла не встретилось 
                added_path.push(object_for_commit.files[i].0.clone()) // проверитть что пушаем именно со слешом или без тоже /
            }
        }

        // подводим итоги диффров между комитами

        println!("Changes:");
        for i in modifited_path {
            println!("    modified: {}", i);
        }
        for i in added_path {
            println!("    added: {}", i)
        }
    }

    let gain_hash = object_for_commits_json.commit_name[0][0..=19].to_string();
    println!("commit {}", gain_hash);

    let file_name = object_for_commits_json.commit_name[0].clone();
    let copas = answer_of_need_path.clone();
    let do_path_to_commit_in_dir = copas.join(".vcs").join(file_name);

    let file_commit = File::open(do_path_to_commit_in_dir).unwrap();        
    let commit_json: serde_json::Value = serde_json::from_reader(file_commit).unwrap();
    let string = serde_json::to_string(&commit_json).unwrap();
    let object_for_commit: initcom::Commit = serde_json::from_str(&string).unwrap();
    println!("Date:    {}", object_for_commit.time_when_was_build);
    println!("Message  {}", object_for_commit.title);
    println!("  No changes");
}
