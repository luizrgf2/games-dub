mod read_camption;
mod cache;
mod process;
mod games;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs;
use std::path::Path;
use std::process::Command;

fn remove_files(){
    let path_caption1 = Path::new("caption1.txt");
    let path_caption2 = Path::new("caption2.txt");
    let path_caption3 = Path::new("caption3.txt");

    if path_caption1.exists(){
        fs::remove_file(path_caption1).unwrap();
    }
    if path_caption2.exists(){
        fs::remove_file(path_caption2).unwrap();
    }
    if path_caption3.exists(){
        fs::remove_file(path_caption3).unwrap();
    }
}

fn wait_for_time(seconds: u64) {
    let duration = Duration::from_secs(seconds);
    thread::sleep(duration);
}

fn run_speaker() {
    let bat_file = "speaker.bat";

    let output = Command::new("cmd")
        .arg("/C")
        .arg(bat_file)
        .output()
        .expect("Falha ao executar o arquivo .bat");

    // Verifique se a execução foi bem-sucedida
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Saída do arquivo .bat:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Erro ao executar o arquivo .bat:\n{}", stderr);
    }
}

fn app() {
    thread::spawn(|| {
        run_speaker()
    });

    let cache_instance = Arc::new(Mutex::new(cache::Cache::new()));
    let cache_instance_thead = Arc::clone(&cache_instance);
    
    let mut selected_game = games::select_game::select_game();

    selected_game.start();
    
    loop {
        let mut cache = cache_instance_thead.lock().unwrap();
        let caption = selected_game.read_caption_of_conversation();
        cache.set_caption(&caption);
        let caption_already_exists = cache.check_if_caption_exists();

        if !caption_already_exists && caption.len() > 0 {
            cache.write_to_cache(&caption);
            println!("{}",caption);
        }
        wait_for_time(1);
    }

}
fn main() {
    remove_files();
    app();
}
