// эта темка управляет пользовательскими файлами в репозитории

// методы которые нам пригодятся ______>
// - Получить список файлов в репозитории
// - Получить список файлов, измененных и созданных после последнего коммита
// - Сформировать коммит из файлов в репозитории
// - “Загрузить” состояние файлов из коммита

use crate::vcs_state_manager;
use crate::initcom;
use chrono;

use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use std::env;
use sha1::{Sha1, Digest};
use std::io::Read;
use std::fs;

// для обхода дерева файловой системы удобно воспользоваться крейтом walkdir - вот эта темка нужна нам туд для вывода комита)))

// pub fn get_list_of_files_from_repo() {
//     unimplemented!()
// }

// pub fn get_changing_files_and_new_from_comit() {
//     unimplemented!() // я это вручную реализовал в стутусе, если хочешь можешь перенести сюда
// }

pub fn make_file_commit_with_repo_files(message: &String) {
    // достать current commit из state
    // сформировать новый джейсон который будет иметь в title сообщение
    // в prev_commit = ссылку на сurrent commit (его хэш)
    // в files файлы которые в данный момент находятся в репозите и еще не закомичены

    // посчитать хэш(параллельно)
    // заснуть хэш этого коммита в лист коммитов, поменять current commit, и добавить дирректорию commit

    // let cur_cur_commit = vcs_state_manager::get_current_commit();


    // И все, что имплементирует трейт serde::Serialize, можно также превращать в строку/в вектор байт:

    // СНАЧАЛА Я СФОРМИРУЮ НОВЫЙ КОМИТ И ДОБАВЛЮ ЕГО ВО ВСЕ ДЖЕКСОНЫ ТОЛЬКО ПОТОМ В ДИР КОММИТС

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
    
    // ----------------------------------------------------

    let mut vec_of_not_commited_files: Vec<(String, [u8; 20])> = Vec::new();

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with(".vcs"))
             .unwrap_or(false)
    }
    
    let walker = WalkDir::new(answer_of_need_path.clone()).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let path_to_each_file_of_repo = entry.unwrap().path().display().to_string();
        
        let mut file = File::open(path_to_each_file_of_repo.clone()).unwrap();
        let mut content: Vec<u8> = Vec::new();
        file.read_to_end(&mut content).unwrap();

        let mut hasher_for_each_file = Sha1::new();
        hasher_for_each_file.update(content);
        let result: [u8; 20] = hasher_for_each_file.finalize().to_vec().try_into().unwrap();
        
        vec_of_not_commited_files.push((path_to_each_file_of_repo, result));
    }
    // благодаря этим действиям я всплыл до репозита, прошелся по всем его файлам взял от них хэш и записал в вектор, что потом будет полем моего коммита 

    let again_current_branch = vcs_state_manager::get_current_branch();
    let again_current_commit = vcs_state_manager::get_current_commit(); // вот эту штуку еще надо внизу использовать для сравнения этих файлов с этим уже старым комммитом

    let object_for_write_comit: initcom::Commit = initcom::Commit { title: (String::from(message.clone())), prev_commit: (again_current_commit.clone()), files: (vec_of_not_commited_files.clone()), branch_where_commit: (again_current_branch.clone()), time_when_was_build: (chrono::Local::now())};
    
    let str_of_elemnts_for_commit = serde_json::to_string(&object_for_write_comit).unwrap();

    let mut hasher_for_commit = Sha1::new();

    hasher_for_commit.update(&str_of_elemnts_for_commit);

    let hash = format!("{:x}", hasher_for_commit.clone().finalize());

    let hash_with_json = hash.clone() + ".json";

    let path_of_dir_to_vcs = answer_of_need_path.join(".vcs");
    let mut new_path_commit = path_of_dir_to_vcs.clone();
    new_path_commit = PathBuf::from(&new_path_commit).join(&hash_with_json); // просто в vcs создал новый коммит джексон
    std::fs::write(new_path_commit, str_of_elemnts_for_commit).unwrap();

    let mut path_of_dir_to_vcs_commits_to_cimmit = path_of_dir_to_vcs.clone();
    path_of_dir_to_vcs_commits_to_cimmit = path_of_dir_to_vcs_commits_to_cimmit.join("commits").join(hash);
    fs::create_dir(path_of_dir_to_vcs_commits_to_cimmit.clone()).unwrap();

    // добавь блин все в джексоны 

    // сейчас добавлю в лист из коммитов = считать объект типа лист коммит + добавить в поле этого объектановый коммит  + записать в файл
    let mut path_to_list_commit = path_of_dir_to_vcs.clone();
    path_to_list_commit = path_to_list_commit.join("commit_list.json");
    let file_with_commits = File::open(path_to_list_commit.clone()).unwrap();

    let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();

    let string = serde_json::to_string(&json).unwrap();

    let mut object_for_commit_json: initcom::CommitList = serde_json::from_str(&string).unwrap();
    
    object_for_commit_json.commit_name.push(hash_with_json.clone());

    let str_of_elements_commit_list = serde_json::to_string(&object_for_commit_json).unwrap();

    std::fs::write(path_to_list_commit, str_of_elements_commit_list).unwrap();


    // сейчас я обновлю текущий коммит в стейте = считать текущие коммит и бренч (-) + создать объект стейта + записать туда новые данные
    let _curre_commit = vcs_state_manager::get_current_commit();
    let curre_branch = vcs_state_manager::get_current_branch();

    // бренч остался таким же, а коммит поменяли на то, что мы только что обработали
    let object_for_state_json: initcom::State = initcom::State { current_comit_hash: (hash_with_json), current_branch_name: (curre_branch) };

    let str_of_elements_for_state_ = serde_json::to_string(&object_for_state_json).unwrap();

    let mut path_to_state_ = path_of_dir_to_vcs.clone();
    path_to_state_ = path_to_state_.join("state.json");

    std::fs::write(path_to_state_, str_of_elements_for_state_).unwrap();

    // ----------------------------------------------------
    
    let walker = WalkDir::new(answer_of_need_path.clone()).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let path_to_each_file_of_repo = entry.unwrap().path().display().to_string();
        let path_to_file = path_to_each_file_of_repo.clone();
        let real_path_to_each_file_of_repo: PathBuf = PathBuf::from(path_to_file);
       
        // в этом блоке я создаю директорию в которой лежит очередной файл -----------
        let dir_path = real_path_to_each_file_of_repo.parent().unwrap();
        if dir_path != answer_of_need_path {
            let create_dir_in_commits = path_of_dir_to_vcs_commits_to_cimmit.join(dir_path);
            fs::create_dir(create_dir_in_commits.clone()).unwrap();

            let create_current_file_in_dir = create_dir_in_commits.join(path_to_each_file_of_repo.clone()); // попробую поменять на path_to_each_file_of_repo 
            fs::copy(real_path_to_each_file_of_repo.clone(), create_current_file_in_dir.clone()).unwrap();
        }
        
        let create_just_file = path_of_dir_to_vcs_commits_to_cimmit.join(path_to_each_file_of_repo.clone());
        fs::copy(real_path_to_each_file_of_repo.clone(), create_just_file.clone()).unwrap();
        // --------------------------------------------------------------------------
    }
    // здесь я выполнил все, хотя мог часть делигировать на вкс 😵‍💫😵‍💫😵‍💫

    // ЗДЕСЬ МНЕ НУЖНО ВЫПОЛНИТЬ ПОЛНЫЙ ФУНКЦИОНАЛ СТАТУСА, ТАК КАК НУЖНА ИНФОРМАЦИЯ О ФАЙЛАХ (И ВЫНЕСИ НАКОНЕЦ ПОДЪЕМ ДО ДИРЕКТОРИИ В ФУНКЦИЮ)
    
    // НО Я УЖЕ НАШЕЛ РЕПО ДИРЕКТОРИЮ И ЗАПОЛНИЛ ВЕКТОР СО СВЕЖИМИ ФАЙЛАМИ :)

    let mut modifited_path: Vec<String> = vec![];
    let mut added_path: Vec<String> = vec![];

    let file_with_state = File::open(again_current_commit.clone()).unwrap();
    let commit_json: serde_json::Value = serde_json::from_reader(file_with_state).unwrap();

    let string = serde_json::to_string(&commit_json).unwrap();

    // здесь лежит закомиченные последние кайфовые файлы
    let object_for_vec_from_commit: initcom::Commit = serde_json::from_str(&string).unwrap();

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

    // ПРИХОДИТСЯ ДЕЛАТЬ ТУТ КОММЕНТ
    // если нет изменений
    if modifited_path.len() == 0 && added_path.len() == 0 {
        println!("No changes to be committed");
        return;
    }

    // есид есть изменения
    println!("[{}  {}] Work in progress", again_current_branch, again_current_commit);
    println!("{} files changed, {} added", modifited_path.len(), added_path.len());
    for i in modifited_path {
        println!("    modified: {}", i);
    }
    for i in added_path {
        println!("    added: {}", i)
    }
}



// pub fn downl_state_of_files_from_commit() {

// }

