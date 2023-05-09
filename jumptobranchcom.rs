use crate::vcs_state_manager::{self, get_current_branch, get_current_commit};
use crate::initcom;

use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use std::env;
use sha1::{Sha1, Digest};
use std::io::Read;
use std::fs;

struct FillData{
    case: String,
    path: String,
}

pub fn gain_data_for_jumptobranch(case: String, br_name: String) {
    let mut labour_item_for_init: FillData = FillData { case: (String::new()), path: (String::new()) }; // хочу показать для себя явно, что они пустые
    labour_item_for_init.case = case.clone();
    labour_item_for_init.path = br_name.clone();

    //1. СНОВА ОБРОБОТАЕМ ОШИБКИ
    //1.1 ПРОВЕРКА НА ОШИБКУ - В РЕПОЗИТОРИИ ЕСТЬ НЕ ЗАКОМИЧЕННЫЕ ИЗМЕНЕНИЯ
    // 1.1 поднимаемся то репозитория
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

    //1.2 заносим хеши всех файлов что сейчас есть в репозите в вектор вместе с их путями
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

    //1.3 проверяем на наличие изменений -> для этого надо убед-ся что длины век-ов mod-ed and ad-ed == 0(но сначала посчитаем их)
    // достанем данные из тек-го коммита -> для этого получим хеш теку-го коммита и откроем в корне джексона с таким хешом
    let copy_of_need_path_commit = answer_of_need_path.clone();
    
    let cur_cur_commit = get_current_commit();
    
    let path_to_cur_commit = copy_of_need_path_commit.join(".vcs").join(cur_cur_commit);
    

    let file_with_state = File::open(path_to_cur_commit).unwrap();
    let commit_json: serde_json::Value = serde_json::from_reader(file_with_state).unwrap();

    let string = serde_json::to_string(&commit_json).unwrap();

    // здесь лежит закомиченные последние кайфовые файлы
    let object_for_vec_from_commit: initcom::Commit = serde_json::from_str(&string).unwrap();


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

    if modifited_path.len() != 0 || added_path.len() != 0 {
        println!("error: Your local changes to the following files should be commited or dropped:");
        for i in modifited_path {
            println!("  {}", i);
        }
        for i in added_path {
            println!("  {}", i);
        }
        return;
    }

    //2.ЕСЛИ У НАС НЕ ПРОИЗОШЛО 1-ОЙ ОШИБКИ, ТО МЫ ЗАЙДЕМ СЮДА И ПРОВЕРИМ ВТОРУЮ - БРЕНЧА С ТАКИМ ИМЕНЕМ НЕТ, ДИБИЛЛЛ!
    //2.1 необходимо прочитать лист из бранчей и достать оттуда вектор с именами бранчей
    let path_to_dir_vcs_commit_list = answer_of_need_path.clone();
    
    let path_to_commit_list = path_to_dir_vcs_commit_list.join(".vcs").join("branch_list.json");
    
    let file_with_commits = File::open(path_to_commit_list).unwrap();
    
    let commit_json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();

    let string = serde_json::to_string(&commit_json).unwrap();

    let object_for_vec_from_branch_list: initcom::BranchList = serde_json::from_str(&string).unwrap();

    let mut counter_on_mistake = 0;
    for i in object_for_vec_from_branch_list.branch_name {
        if i == br_name {
            counter_on_mistake += 1;
        }
    }

    if counter_on_mistake == 0 {
        println!("No branch {} exists.", br_name);
        println!("Aborting...");
        return;
    }


    //3. ПОСЛЕ ТОГО КАК Я ПРОВЕРИЛ НА ОИШБКИ НУЖНО РЕАЛИЗОВАТЬ ОСНОВНОЙ ФУНКЦИОНАЛ 
    //3.1 нам нужно вычислить последний коммит в текущем бранче, который нам ЗАКИНУЛИИИ 
    //нам нужно проходится по списку всех коммитов в сommit list + обра-ть каждый коммит и смотреть в каком бранче он лежит
    // и всегла нужно обновлять какую-то одну переменную где будет хра-ся полследний коммит в бранче 

    let mut path_to_commit_list = answer_of_need_path.clone();
    path_to_commit_list = path_to_commit_list.join(".vcs").join("commit_list.json");
    let file_with_commits = File::open(path_to_commit_list.clone()).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file_with_commits).unwrap();
    let string = serde_json::to_string(&json).unwrap();
    let object_for_commits_json: initcom::CommitList = serde_json::from_str(&string).unwrap();

    let mut last_commit: String = String::from(""); // здесь будет лежать последний коммит в br_name
    for i in object_for_commits_json.commit_name {
        // let gain_hash = i[..i.len() - 5].to_string();
        //здесь мы должны считать файл итого коммита и от туда вытащить бранч нейм
        let mut path_to_commit_in_dir = answer_of_need_path.clone();
        path_to_commit_in_dir = path_to_commit_in_dir.join(".vcs").join(i.clone());

        let file_with_this_commit = File::open(path_to_commit_in_dir).unwrap();
        let commit_json: serde_json::Value = serde_json::from_reader(file_with_this_commit).unwrap();
        let string = serde_json::to_string(&commit_json).unwrap();
        let object_for_this_commit: initcom::Commit = serde_json::from_str(&string).unwrap();

        if object_for_this_commit.branch_where_commit == br_name {
            last_commit = i; // мы кладем именно имя файла самого коммита (hash.json)
        }
    }

    // 3.2 мы получили последний коммит в бренче, теперь нам нужно сделать все тоже самое, что и в джамп коммите
    // для начала удолим все файлы из репозита
    let walker = WalkDir::new(answer_of_need_path.clone()).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let path_to_each_file_or_dir_of_repo = entry.unwrap().path().to_path_buf();
        // мои пути выглядят примерно так: repo/any.txt or repo/folder
        // поэтому мне нужно проверить сейчас директория или файл, лессс гоу
        if path_to_each_file_or_dir_of_repo.is_dir() {
            fs::remove_dir(path_to_each_file_or_dir_of_repo).unwrap();
        } else {
            fs::remove_file(path_to_each_file_or_dir_of_repo).unwrap();
        }
    }
    
    // теперь нужно считать тот коммит, который нам дан
    let copy_of_answer = answer_of_need_path.clone();
    let name_of_commit_file = last_commit.clone(); // hash.json
    let path_to_known_commit = copy_of_answer.join(".vcs").join(name_of_commit_file.clone());

    
    let file_with_known_commit = File::open(path_to_known_commit).unwrap();
    let commit_json: serde_json::Value = serde_json::from_reader(file_with_known_commit).unwrap();
    let string = serde_json::to_string(&commit_json).unwrap();
    let object_for_known_commit: initcom::Commit = serde_json::from_str(&string).unwrap();

    // туперь мне нужно files из это object_for_known_commit перенести в корень репозита откуда я только что все удалил
    // 👿 не все так просто, тут должно быть полное копирование -> нам нафик не нужен сам коммит

    // нужно найти в commits данный нам коммит
    // repo/
    let copy_of_answer = answer_of_need_path.clone();
    // repo/.vcs/commits/commit_hash
    let commit_name_wout_json = name_of_commit_file[..name_of_commit_file.len() - 5].to_string();
    let path_commits_to_known_hash = copy_of_answer.join(".vcs").join("commits").join(commit_name_wout_json.clone());
    
    //теперрь мне нужно пройтись по этой директории
    
    let walker = WalkDir::new(&path_commits_to_known_hash).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let path_to_each_file_or_dir_of_repo = entry.unwrap().path().to_path_buf();
        // мои пути = repo/.vcs/commits/commit_hash/file.txt <- в такой ситуации просто кидаем файл в корень      repo/.vcs/commits/commit_hash/file.txt
        //          = repo/.vcs/commits/commit_hash/data.txt                                                      repo/.vcs/commits/commit_hash/dir_1/dir_2
        //          = repo/.vcs/commits/commit_hash/dir_1
        //          = repo/.vcs/commits/commit_hash/dir_1/kreker.txt
        // мне нужно доставать именно имена файлов и полностью дирректории с файлами и добавлять их в корень
        // let what_dir_at_this_moment: PathBuf = PathBuf::new();
        if path_to_each_file_or_dir_of_repo.is_dir() {
            // what_dir_at_this_moment = path_to_each_file_or_dir_of_repo;
            let name_of_dir = path_to_each_file_or_dir_of_repo.strip_prefix(&path_commits_to_known_hash).unwrap().to_path_buf();
            let copans = answer_of_need_path.clone();
            let path_from_dir_to_name_of_dir = copans.join(name_of_dir);
            fs::create_dir_all(path_from_dir_to_name_of_dir).unwrap(); // я создал в корне директорию которая лежит в commit_hash
        } else {
            let parent_of_file = path_to_each_file_or_dir_of_repo.parent().unwrap().to_path_buf();
            let path_without_repo_vcs_commits_commit_hash = parent_of_file.strip_prefix(&path_commits_to_known_hash).unwrap().to_path_buf();
            if path_without_repo_vcs_commits_commit_hash == PathBuf::from("") { // это значит нам просто нужно кинуть файл в корень
                let name_of_file = path_to_each_file_or_dir_of_repo.strip_prefix(&path_commits_to_known_hash).unwrap().to_path_buf();
                let copas = answer_of_need_path.clone();
                let path_from_dir_to_file = copas.join(name_of_file);
                fs::copy(path_to_each_file_or_dir_of_repo, path_from_dir_to_file).unwrap(); // скопировал по полны путям ЖЕЕЕЕЕЕЕСТЬЬЬЬЬЬЬ!!!!!
            } else { // это значит что перед нашим файлом находится как минимум одна директория
                let path_to_file_with_dirs = path_to_each_file_or_dir_of_repo.strip_prefix(&path_commits_to_known_hash).unwrap().to_path_buf();
                let copas = answer_of_need_path.clone();
                let from_dir_to_dir_with_file = copas.join(path_to_file_with_dirs);
                fs::copy(path_to_each_file_or_dir_of_repo, from_dir_to_dir_with_file).unwrap();
            }
        }
    }

    // 3.2 Обновляешь стейт
    let commit_haha_with_json = name_of_commit_file.clone();

    let object_for_state_json: initcom::State = initcom::State { current_comit_hash: (commit_haha_with_json), current_branch_name: (br_name.clone()) };

    let str_of_elements_for_state = serde_json::to_string(&object_for_state_json).unwrap();

    let copas = answer_of_need_path.clone();
    let path_to_state = copas.join(".vcs").join("state.json");

    std::fs::write(path_to_state, str_of_elements_for_state).unwrap();

    //4 ВЫВОД
    println!("Successfully jumped to branch {}. Current commit: {}.", br_name, commit_name_wout_json);
}