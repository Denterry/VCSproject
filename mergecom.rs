use crate::vcs_state_manager::{self, get_current_branch, get_current_commit};
use crate::initcom;
use chrono::{self, Local, DateTime};

use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use std::env;
use sha1::{Sha1, Digest};
use std::io::Read;
use std::fs;
use std::path;

struct FillData {
    case: String,
    branch_name: String,
}

pub fn gain_data_for_new_branch(case: String, branch_name: String) {
    let mut labour_item_for_init: FillData = FillData { case: (String::new()), branch_name: (String::new()) }; // хочу показать для себя явно, что они пустые
    labour_item_for_init.case = case.clone();
    labour_item_for_init.branch_name = branch_name.clone();


    //1. ОБРАБАТЫВАЕМ ОШИБКИ ДЛЯ МЕРДЖ-КОНФЛИКТА
    //1.1 В случае, если текущий коммит — не последний коммит мастера, выводится сообщение об ошибке:
    
    let cur_com = vcs_state_manager::get_current_commit();

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with(".vcs"))
             .unwrap_or(false)
    }

    // для того, чтобы найти последний коммит для мастера нужно достать из комит листа вектор с комиатми и там для каждого проанализить бранч
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

    let mut path_to_comits = answer_of_need_path.clone();
    path_to_comits = path_to_comits.join(".vcs").join("commit_list.json");
    let file_with_commits = File::open(path_to_comits.clone()).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();
    let string = serde_json::to_string(&json).unwrap();
    let object_for_commits_json: initcom::CommitList = serde_json::from_str(&string).unwrap();

    // нашли ласт коммит в мастере
    let mut last_commit_in_master: String = String::from("");
    for i in 0..object_for_commits_json.commit_name.len() {
        let mut path_commit = answer_of_need_path.clone();
        path_commit = path_commit.join(".vcs").join(&object_for_commits_json.commit_name[i]);
        let file_with_state = File::open(path_commit).unwrap();
        let commit_json: serde_json::Value = serde_json::from_reader(file_with_state).unwrap();
        let string = serde_json::to_string(&commit_json).unwrap();
        let object_for_vec_from_commit: initcom::Commit = serde_json::from_str(&string).unwrap();
        if object_for_vec_from_commit.branch_where_commit == "master" {
            last_commit_in_master = object_for_commits_json.commit_name[i].clone();
        }
    }

    if cur_com != last_commit_in_master {
        println!("The merge is possible only when you are in the last commit in master.");
        println!("Aborting...");
        return;
    }

    //2. пытаемся добыть три множества нужных нам файлов а именно три коммита
    //2.1 найдем последний коммит в бренче, который нам приходит в запросе
    //нашли ласт коммит в бранче
    let mut last_commit_in_branch: String = String::from("");
    for i in 0..object_for_commits_json.commit_name.len() {
        let mut path_commit = answer_of_need_path.clone();
        path_commit = path_commit.join(".vcs").join(&object_for_commits_json.commit_name[i]);
        let file_with_state = File::open(path_commit).unwrap();
        let commit_json: serde_json::Value = serde_json::from_reader(file_with_state).unwrap();
        let string = serde_json::to_string(&commit_json).unwrap();
        let object_for_vec_from_commit: initcom::Commit = serde_json::from_str(&string).unwrap();
        if object_for_vec_from_commit.branch_where_commit == branch_name.clone() {
            last_commit_in_branch = object_for_commits_json.commit_name[i].clone();
        }
    }
    //2.2 последний коммит в мастере мы уже нашли ^^^^^^^^^^^^^

    //2.3 найдем комит ответвления от бранча, он собственно и лежит в самом бранче
    let mut path_to_branch = answer_of_need_path.clone();
    let name_of_branch = branch_name.clone() + ".json";
    path_to_branch = path_to_branch.join(".vcs").join(name_of_branch);

    let file_brancha = File::open(path_to_branch).unwrap();

    let branch_json: serde_json::Value = serde_json::from_reader(file_brancha).unwrap();
    
    let string = serde_json::to_string(&branch_json).unwrap();

    let object_for_branch: initcom::Branch = serde_json::from_str(&string).unwrap();
    let otv_commit = object_for_branch.hash_of_otvetvlen_commit + ".json";
    // ЗАЧЕМ Я В БРЕНЧЕ ХРАНЮ КОММИТЫ БЕЗ ДЖЕКСОНА В ЧЕМ СМЫСЛ

    //2.4 нашли все три коммита, теперь соберем их в кучку
    let commit_master = last_commit_in_master.clone();
    let commit_branch = last_commit_in_branch.clone();
    let commit_otvetv = otv_commit.clone();

    //3 ТЕПЕРЬ ОТНОСИТЕЛЬНО ФАЙЛОВ КОТОРЫЕ ЕСТЬ В КОММИТЕ ОТВЕТВЛЕНИЯ Я БУДУ СМОТРЕТЬ НА ФАЙЛЫ В ДРУГИХ ДВУХ КОММИТАХ
    //3.1 считаем каждый коммит

    let copy_of_need_path_commit = answer_of_need_path.clone();
    let path_to_commit_master = copy_of_need_path_commit.join(".vcs").join(commit_master);
    let file_with_commit_master = File::open(path_to_commit_master).unwrap();
    let commit_master_json: serde_json::Value = serde_json::from_reader(file_with_commit_master).unwrap();
    let string = serde_json::to_string(&commit_master_json).unwrap();
    let object_for_commit_master: initcom::Commit = serde_json::from_str(&string).unwrap();

    let copy_of_need_path_commit = answer_of_need_path.clone();
    let path_to_commit_branch = copy_of_need_path_commit.join(".vcs").join(commit_branch);
    let file_with_commit_branch = File::open(path_to_commit_branch).unwrap();
    let commit_branch_json: serde_json::Value = serde_json::from_reader(file_with_commit_branch).unwrap();
    let string = serde_json::to_string(&commit_branch_json).unwrap();
    let object_for_commit_branch: initcom::Commit = serde_json::from_str(&string).unwrap();

    let copy_of_need_path_commit = answer_of_need_path.clone();
    let path_to_commit_otvetv = copy_of_need_path_commit.join(".vcs").join(commit_otvetv);
    let file_with_commit_otvetv = File::open(path_to_commit_otvetv).unwrap();
    let commit_otvetv_json: serde_json::Value = serde_json::from_reader(file_with_commit_otvetv).unwrap();
    let string = serde_json::to_string(&commit_otvetv_json).unwrap();
    let object_for_commit_otvetv: initcom::Commit = serde_json::from_str(&string).unwrap();

    //3.2 сравним файлы в ответвлен коммит и мастер коммит 

    let mut modifited_added_path_fist: Vec<(String, [u8; 20])> = vec![];
    // let mut added_path: Vec<(String, [u8; 20])> = vec![];

    // commit_master. files =  {path.txt, 12
    //                          data.txt, 13
    //                          london.txt,14
    //                          }
    // commit_otvetvleniya.files = {path.txt, 12
    //                              data.txt, 15
    //                              capital.txt, 16
    //                              }
    // list_izmeneniy = {data.txt, capital.txt, london.txt}

    for i in 0..object_for_commit_master.files.len() {
        for j in 0..object_for_commit_otvetv.files.len() {
            if object_for_commit_master.files[i].0 == object_for_commit_otvetv.files[j].0 { // проверь что здесь оба пути сравнивабтся либо без / в конце или оба с ним
                for k in 0..20 {
                    if object_for_commit_master.files[i].1[k] != object_for_commit_otvetv.files[j].1[k] {
                        // значит произошло модифайд
                        modifited_added_path_fist.push((object_for_commit_master.files[i].0.clone(), object_for_commit_master.files[i].1.clone()));
                        break;
                    }
                }
            }
        }
    }

    for i in 0..object_for_commit_master.files.len() {
        let mut counter_for_add = 0;
        for j in 0..object_for_commit_otvetv.files.len() {
            if object_for_commit_otvetv.files[j].0 == object_for_commit_master.files[i].0 {
                counter_for_add += 1;
                break;
            }
        }
        if counter_for_add == 0 {
            // значит такого файла не встретилось 
            modifited_added_path_fist.push((object_for_commit_master.files[i].0.clone(), object_for_commit_master.files[i].1.clone())) // проверитть что пушаем именно со слешом или без тоже /
        }
    }
    for i in 0..object_for_commit_otvetv.files.len() {
        let mut counter_for_add = 0;
        for j in 0..object_for_commit_master.files.len() {
            if object_for_commit_master.files[j].0 == object_for_commit_otvetv.files[i].0 {
                counter_for_add += 1;
                break;
            }
        }
        if counter_for_add == 0 {
            // значит такого файла не встретилось 
            modifited_added_path_fist.push((object_for_commit_otvetv.files[i].0.clone(), object_for_commit_otvetv.files[i].1.clone())) // проверитть что пушаем именно со слешом или без тоже /
        }
    }

    //3.3 теперь тоже самое только сравним ответлен коммит и бранч коммит

    let mut modifited_added_path_second: Vec<(String, [u8; 20])> = vec![];

    for i in 0..object_for_commit_branch.files.len() {
        for j in 0..object_for_commit_otvetv.files.len() {
            if object_for_commit_branch.files[i].0 == object_for_commit_otvetv.files[j].0 { // проверь что здесь оба пути сравнивабтся либо без / в конце или оба с ним
                for k in 0..20 {
                    if object_for_commit_branch.files[i].1[k] != object_for_commit_otvetv.files[j].1[k] {
                        // значит произошло модифайд
                        modifited_added_path_second.push((object_for_commit_branch.files[i].0.clone(), object_for_commit_branch.files[i].1.clone()));
                        break;
                    }
                }
            }
        }
    }

    for i in 0..object_for_commit_branch.files.len() {
        let mut counter_for_add = 0;
        for j in 0..object_for_commit_otvetv.files.len() {
            if object_for_commit_otvetv.files[j].0 == object_for_commit_branch.files[i].0 {
                counter_for_add += 1;
                break;
            }
        }
        if counter_for_add == 0 {
            // значит такого файла не встретилось 
            modifited_added_path_second.push((object_for_commit_branch.files[i].0.clone(), object_for_commit_branch.files[i].1.clone())) // проверитть что пушаем именно со слешом или без тоже /
        }
    }
    for i in 0..object_for_commit_otvetv.files.len() {
        let mut counter_for_add = 0;
        for j in 0..object_for_commit_branch.files.len() {
            if object_for_commit_branch.files[j].0 == object_for_commit_otvetv.files[i].0 {
                counter_for_add += 1;
                break;
            }
        }
        if counter_for_add == 0 {
            // значит такого файла не встретилось 
            modifited_added_path_second.push((object_for_commit_otvetv.files[i].0.clone(), object_for_commit_otvetv.files[i].1.clone())) // проверитть что пушаем именно со слешом или без тоже /
        }
    }

    // получили два списка изменений 
    // let mut modifited_added_path_second: Vec<(String, [u8; 20])> = vec![];
    // let mut modifited_added_path_fist: Vec<(String, [u8; 20])> = vec![];

    let mut all_mistake_pathes: Vec<String> = vec![];
    for it_1 in 0..modifited_added_path_fist.len() {
        for it_2 in 0..modifited_added_path_second.len() {
            if (modifited_added_path_fist[it_1].0 == modifited_added_path_second[it_2].0) 
                && (modifited_added_path_fist[it_1].1 != modifited_added_path_second[it_2].1) {
                    all_mistake_pathes.push(modifited_added_path_fist[it_1].0.clone());
                    break;
                }
            // if (k.0. == t.0) && k.1 != t.1 {
            //     all_mistake_pathes.push(k.0);
            //     // произошло пересечение
            //     // println!("Merge confilict: file has been changed both in master and branch");
            //     // println!("   {}", i.0);
            //     // println!("   {}", j.0);
            //     // println!("Aborting...");
            //     // return;
            // }
        }
    }
    if all_mistake_pathes.len() != 0 {
        println!("Merge confilict: file has been changed both in master and branch");
        for i in all_mistake_pathes {
            println!("   {}", i);
        }
        println!("Aborting...");
        return;
    }
    
    // СПРОСИ ПРО ТО НУЖНО ЛИ ПРОВЕРЯТЬ ТУТ НА ОШИБКУ КАК В КОММИТЕ ИЛИ НЕТ 
    // СДЕЛАЕМ ПРОВЕРКУ незакомиченности - как в джфампах

    // теперь если мы не вышли из функции сформируем коммит
    let message = "Merged branch  ".to_string() + branch_name.as_ref();
    let merge_commit: initcom::Commit = initcom::Commit { title: (String::from(message)), prev_commit: (last_commit_in_master), files: (modifited_added_path_second.clone()), branch_where_commit: (String::from("master")), time_when_was_build: (Local::now())};
    let str_merge_commit = serde_json::to_string(&merge_commit).unwrap();
    
    let mut hasher_for_merge_commit = Sha1::new();

    hasher_for_merge_commit.update(&str_merge_commit);

    let hash = format!("{:x}", hasher_for_merge_commit.clone().finalize());

    let hash_with_json = hash.clone() + ".json";

    let mut path_merge_commit = answer_of_need_path.clone();

    path_merge_commit = path_merge_commit.join(".vcs").join(hash_with_json.clone());

    std::fs::write(path_merge_commit, str_merge_commit).unwrap();

    // сейчас добавлю в лист из коммитов = считать объект типа лист коммит + добавить в поле этого объектановый коммит  + записать в файл
    let mut path_to_list_commit = answer_of_need_path.clone();
    path_to_list_commit = path_to_list_commit.join(".vcs").join("commit_list.json");
    let file_with_commits = File::open(path_to_list_commit.clone()).unwrap();

    let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();

    let string = serde_json::to_string(&json).unwrap();

    let mut object_for_commit_json: initcom::CommitList = serde_json::from_str(&string).unwrap();
    
    object_for_commit_json.commit_name.push(hash_with_json.clone());

    let str_of_elements_commit_list = serde_json::to_string(&object_for_commit_json).unwrap();

    std::fs::write(path_to_list_commit, str_of_elements_commit_list).unwrap();

     // сейчас я обновлю текущий коммит в стейте = считать текущие коммит и бренч (-) + создать объект стейта + записать туда новые данные
     let _curre_commit = vcs_state_manager::get_current_commit();

     let object_for_state_json: initcom::State = initcom::State { current_comit_hash: (hash_with_json), current_branch_name: (String::from("master")) };
 
     let str_of_elements_for_state_ = serde_json::to_string(&object_for_state_json).unwrap();
 
     let mut path_to_state_ = answer_of_need_path.clone();

     path_to_state_ = path_to_state_.join(".vcs").join("state.json");
 
     std::fs::write(path_to_state_, str_of_elements_for_state_).unwrap();


     // сейчас я должен каким-то невероятным образом закинуть этот коммит в commits
     //БОЖЕ КАКОЙ ЖЕ Я ОСЕЛ, У МЕНЯ УЖЕ В ФАЙЛС В КОММИТЕ ПОЛНЫЙ ПУТЬ ДО ФАЙЛА ОТ РЕПОЗИТОРИЯ ПРОСТО ОТКУСИ ПРЕФИКС answer_need_path ОТ ПУТИ

    let mut path_to_commits = answer_of_need_path.clone();
    path_to_comits = path_to_comits.join(".vcs").join("commits");

    let path_to_commits_hash = path_to_comits.join(hash);

     for i in modifited_added_path_second {
        let path = i.0.clone();
        let mut path_wout_dir = PathBuf::from(path);
        path_wout_dir = path_wout_dir.strip_prefix(&answer_of_need_path).unwrap().to_path_buf();
        // /repo/file.txt -> file.txt;
        // /repo/dir_1/file.txt -> dir_1/file.txt;
        let mut path_to_each_file = path_to_commits_hash.clone();
        path_to_each_file = path_to_each_file.join(path_wout_dir);
        if path_to_each_file.is_dir() {
            fs::create_dir_all(path_to_each_file).unwrap();
        } else {
            fs::copy(i.0.clone(), path_to_each_file).unwrap();
        }
     }

    // let walker = WalkDir::new(answer_of_need_path.clone()).into_iter();
    // for entry in walker.filter_entry(|e| !is_hidden(e)) {
    //     let path_to_each_file_of_repo = entry.unwrap().path().display().to_string();
        // let path_to_each_file_of_repo = entry.unwrap().path().display().to_string();
        // let path_to_file = path_to_each_file_of_repo.clone();
        // let real_path_to_each_file_of_repo: PathBuf = PathBuf::from(path_to_file);
       
        // // в этом блоке я создаю директорию в которой лежит очередной файл -----------
        // let dir_path = real_path_to_each_file_of_repo.parent().unwrap();
        // if dir_path != answer_of_need_path {
        //     let create_dir_in_commits = path_of_dir_to_vcs_commits_to_cimmit.join(dir_path);
        //     fs::create_dir(create_dir_in_commits.clone()).unwrap();

        //     let create_current_file_in_dir = create_dir_in_commits.join(path_to_each_file_of_repo.clone()); // попробую поменять на path_to_each_file_of_repo 
        //     fs::copy(real_path_to_each_file_of_repo.clone(), create_current_file_in_dir.clone()).unwrap();
        // }
        
        // let create_just_file = path_of_dir_to_vcs_commits_to_cimmit.join(path_to_each_file_of_repo.clone());
        // fs::copy(real_path_to_each_file_of_repo.clone(), create_just_file.clone()).unwrap();
    // }


     // ТЕПЕРЬ ПРИСТУПИМ К УДАЛЕНИЮ КОММИТОВ КОТОРЫЕ ЛЕЖАЛИ В branch_name и сам бранч
     // мы должны пройтись по листу из коммитов открывать каждый коммит и если 
     // он лежит в branch_name, то я = удаляю этот комит из .vcs + удаляю папку коммита в commits (и файлы ему принадлежащие)*

     let mut path_to_list_commit = answer_of_need_path.clone();
     path_to_list_commit = path_to_list_commit.join(".vcs").join("commit_list.json");
     let file_with_commits = File::open(path_to_list_commit.clone()).unwrap();
 
     let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();
 
     let string = serde_json::to_string(&json).unwrap();
 
     let mut object_for_commit_json: initcom::CommitList = serde_json::from_str(&string).unwrap();

     let mut new_commit_name: Vec<String> = vec![]; 
     for i in object_for_commit_json.commit_name {
        let mut path_to_each_commit = answer_of_need_path.clone();
        path_to_each_commit  = path_to_each_commit.join(".vcs").join(i.clone());

        let file_with_commit = File::open(path_to_each_commit.clone()).unwrap();
        let commit_js: serde_json::Value = serde_json::from_reader(file_with_commit).unwrap();
        let string = serde_json::to_string(&commit_js).unwrap();
        let object_for_commit: initcom::Commit = serde_json::from_str(&string).unwrap();

        if object_for_commit.branch_where_commit == branch_name {// тот самый момент
            //1. удалим файлы из commits
            let hash_clear = i[0..=19].to_string();
            let mut path_commit_in_commits = answer_of_need_path.clone();
            path_commit_in_commits = path_commit_in_commits.join(".vcs").join("commits").join(hash_clear);
            fs::remove_dir(path_commit_in_commits).unwrap();

            //2. удалим коммит из .vcs
            fs::remove_file(path_to_each_commit).unwrap();
        } else {
            new_commit_name.push(i.clone());
        }
     }
     object_for_commit_json.commit_name = new_commit_name;

     let str_for_new_commit_list = serde_json::to_string(&object_for_commit_json).unwrap();

     let copas = answer_of_need_path.clone();
     let path_to_commit_list = copas.join(".vcs").join("commit_list.json");
 
     std::fs::write(path_to_commit_list, str_for_new_commit_list).unwrap();




     // осталось удалить бранч
     let mut path_to_list_branch = answer_of_need_path.clone();
     path_to_list_branch = path_to_list_branch.join(".vcs").join("branch_list.json");
     let file_with_commits = File::open(path_to_list_commit.clone()).unwrap();
 
     let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();
 
     let string = serde_json::to_string(&json).unwrap();
 
     let mut object_for_branch_json: initcom::BranchList = serde_json::from_str(&string).unwrap();

     let mut new_branch_name: Vec<String> = vec![];
     for i in object_for_branch_json.branch_name {
        if i == branch_name {
            let file_name = i + ".json";
            let mut path_to_each_branch = answer_of_need_path.clone();
            path_to_each_branch  = path_to_each_branch.join(".vcs").join(file_name);

            fs::remove_file(path_to_each_branch).unwrap();
        } else {
            new_branch_name.push(i);
        }
     }
     object_for_branch_json.branch_name = new_branch_name;

     let str_for_new_branch_list = serde_json::to_string(&object_for_branch_json).unwrap();

     let copas = answer_of_need_path.clone();
     let path_to_branch_list = copas.join(".vcs").join("branch_list.json");
 
     std::fs::write(path_to_branch_list, str_for_new_branch_list).unwrap();
}