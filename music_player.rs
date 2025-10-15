use std::io;
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
struct Song {
    title: String,
    artist: String,
    duration: u32, // в секундах
    genre: String,
    year: u16,
    path: String,
}

#[derive(Debug)]
struct Playlist {
    name: String,
    songs: Vec<Song>,
    current_index: Option<usize>,
    is_playing: bool,
    is_shuffle: bool,
}

struct MusicPlayer {
    library: Vec<Song>,
    playlists: HashMap<String, Playlist>,
    current_playlist: Option<String>,
    volume: u8,
}

impl Song {
    fn new(title: String, artist: String, duration: u32, genre: String, year: u16, path: String) -> Self {
        Song { title, artist, duration, genre, year, path }
    }

    fn format_duration(&self) -> String {
        let minutes = self.duration / 60;
        let seconds = self.duration % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn display(&self) -> String {
        format!("🎵 {} - {} [{}] ({})", 
                self.artist, self.title, self.format_duration(), self.genre)
    }
}

impl Playlist {
    fn new(name: String) -> Self {
        Playlist {
            name,
            songs: Vec::new(),
            current_index: None,
            is_playing: false,
            is_shuffle: false,
        }
    }

    fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    fn remove_song(&mut self, index: usize) -> Option<Song> {
        if index < self.songs.len() {
            Some(self.songs.remove(index))
        } else {
            None
        }
    }

    fn get_current_song(&self) -> Option<&Song> {
        if let Some(index) = self.current_index {
            self.songs.get(index)
        } else {
            None
        }
    }

    fn next_song(&mut self) -> Option<&Song> {
        if self.songs.is_empty() {
            return None;
        }

        if self.is_shuffle {
            let mut rng = rand::thread_rng();
            self.current_index = Some(rng.gen_range(0..self.songs.len()));
        } else {
            self.current_index = match self.current_index {
                Some(index) => Some((index + 1) % self.songs.len()),
                None => Some(0),
            };
        }

        self.get_current_song()
    }

    fn previous_song(&mut self) -> Option<&Song> {
        if self.songs.is_empty() {
            return None;
        }

        self.current_index = match self.current_index {
            Some(index) => {
                if index == 0 {
                    Some(self.songs.len() - 1)
                } else {
                    Some(index - 1)
                }
            }
            None => Some(0),
        };

        self.get_current_song()
    }

    fn get_total_duration(&self) -> u32 {
        self.songs.iter().map(|song| song.duration).sum()
    }

    fn display_info(&self) -> String {
        let total_duration = self.get_total_duration();
        let total_minutes = total_duration / 60;
        let total_seconds = total_duration % 60;
        
        format!("📁 {} ({} треков, {:02}:{:02})", 
                self.name, self.songs.len(), total_minutes, total_seconds)
    }
}

impl MusicPlayer {
    fn new() -> Self {
        let mut player = MusicPlayer {
            library: Vec::new(),
            playlists: HashMap::new(),
            current_playlist: None,
            volume: 50,
        };

        // Добавляем демо-композиции
        player.add_demo_songs();
        player
    }

    fn add_demo_songs(&mut self) {
        let demo_songs = vec![
            Song::new("Bohemian Rhapsody".to_string(), "Queen".to_string(), 354, "Rock".to_string(), 1975, "queen_bohemian.mp3".to_string()),
            Song::new("Stairway to Heaven".to_string(), "Led Zeppelin".to_string(), 482, "Rock".to_string(), 1971, "lz_stairway.mp3".to_string()),
            Song::new("Hotel California".to_string(), "Eagles".to_string(), 391, "Rock".to_string(), 1976, "eagles_hotel.mp3".to_string()),
            Song::new("Imagine".to_string(), "John Lennon".to_string(), 183, "Pop".to_string(), 1971, "lennon_imagine.mp3".to_string()),
            Song::new("Sweet Child O' Mine".to_string(), "Guns N' Roses".to_string(), 356, "Rock".to_string(), 1987, "gnr_sweet_child.mp3".to_string()),
            Song::new("Billie Jean".to_string(), "Michael Jackson".to_string(), 294, "Pop".to_string(), 1982, "mj_billie_jean.mp3".to_string()),
            Song::new("Smells Like Teen Spirit".to_string(), "Nirvana".to_string(), 301, "Grunge".to_string(), 1991, "nirvana_teen_spirit.mp3".to_string()),
            Song::new("Yesterday".to_string(), "The Beatles".to_string(), 125, "Pop".to_string(), 1965, "beatles_yesterday.mp3".to_string()),
        ];

        self.library = demo_songs;

        // Создаем демо-плейлисты
        let mut rock_playlist = Playlist::new("🎸 Rock Classics".to_string());
        let mut pop_playlist = Playlist::new("🎤 Pop Hits".to_string());

        for song in &self.library {
            match song.genre.as_str() {
                "Rock" | "Grunge" => rock_playlist.add_song(song.clone()),
                "Pop" => pop_playlist.add_song(song.clone()),
                _ => {}
            }
        }

        self.playlists.insert("Rock Classics".to_string(), rock_playlist);
        self.playlists.insert("Pop Hits".to_string(), pop_playlist);
    }

    fn search_songs(&self, query: &str) -> Vec<&Song> {
        let query = query.to_lowercase();
        self.library.iter()
            .filter(|song| {
                song.title.to_lowercase().contains(&query) ||
                song.artist.to_lowercase().contains(&query) ||
                song.genre.to_lowercase().contains(&query)
            })
            .collect()
    }

    fn create_playlist(&mut self, name: String) -> bool {
        if self.playlists.contains_key(&name) {
            false
        } else {
            self.playlists.insert(name.clone(), Playlist::new(name));
            true
        }
    }

    fn add_song_to_playlist(&mut self, playlist_name: &str, song_index: usize) -> bool {
        if let Some(song) = self.library.get(song_index) {
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                playlist.add_song(song.clone());
                return true;
            }
        }
        false
    }

    fn play_playlist(&mut self, playlist_name: &str) -> bool {
        if self.playlists.contains_key(playlist_name) {
            self.current_playlist = Some(playlist_name.to_string());
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                playlist.is_playing = true;
                if playlist.current_index.is_none() && !playlist.songs.is_empty() {
                    playlist.current_index = Some(0);
                }
            }
            true
        } else {
            false
        }
    }

    fn get_current_song(&self) -> Option<&Song> {
        if let Some(playlist_name) = &self.current_playlist {
            if let Some(playlist) = self.playlists.get(playlist_name) {
                return playlist.get_current_song();
            }
        }
        None
    }

    fn next_song(&mut self) -> Option<String> {
        if let Some(playlist_name) = &self.current_playlist.clone() {
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                if let Some(song) = playlist.next_song() {
                    return Some(format!("▶️ Играет: {}", song.display()));
                }
            }
        }
        None
    }

    fn previous_song(&mut self) -> Option<String> {
        if let Some(playlist_name) = &self.current_playlist.clone() {
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                if let Some(song) = playlist.previous_song() {
                    return Some(format!("▶️ Играет: {}", song.display()));
                }
            }
        }
        None
    }

    fn toggle_shuffle(&mut self) -> bool {
        if let Some(playlist_name) = &self.current_playlist.clone() {
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                playlist.is_shuffle = !playlist.is_shuffle;
                return playlist.is_shuffle;
            }
        }
        false
    }

    fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    fn get_recommendations(&self) -> Vec<&Song> {
        if let Some(current_song) = self.get_current_song() {
            // Рекомендуем песни того же жанра или артиста
            self.library.iter()
                .filter(|song| {
                    song.genre == current_song.genre || 
                    song.artist == current_song.artist
                })
                .take(5)
                .collect()
        } else {
            // Случайные рекомендации
            let mut rng = rand::thread_rng();
            let mut songs: Vec<&Song> = self.library.iter().collect();
            songs.sort_by_key(|_| rng.gen::<u32>());
            songs.into_iter().take(5).collect()
        }
    }
}

fn main() {
    println!("🎵 МУЗЫКАЛЬНЫЙ ПЛЕЕР");
    println!("==================");
    println!();

    let mut player = MusicPlayer::new();
    let mut input = String::new();

    loop {
        print_menu();
        input.clear();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let choice = input.trim();
                match choice {
                    "1" => show_library(&player),
                    "2" => show_playlists(&player),
                    "3" => search_music(&player),
                    "4" => create_new_playlist(&mut player),
                    "5" => play_playlist_menu(&mut player),
                    "6" => control_playback(&mut player),
                    "7" => manage_volume(&mut player),
                    "8" => show_recommendations(&player),
                    "9" => show_current_status(&player),
                    "0" => {
                        println!("👋 До свидания!");
                        break;
                    }
                    _ => println!("❌ Неверный выбор!"),
                }
            }
            Err(_) => println!("❌ Ошибка ввода!"),
        }

        println!("\nНажмите Enter для продолжения...");
        input.clear();
        let _ = io::stdin().read_line(&mut input);
    }
}

fn print_menu() {
    println!("\n🎵 ГЛАВНОЕ МЕНЮ:");
    println!("1. 📚 Библиотека");
    println!("2. 📁 Плейлисты");
    println!("3. 🔍 Поиск");
    println!("4. ➕ Создать плейлист");
    println!("5. ▶️ Воспроизвести плейлист");
    println!("6. 🎮 Управление воспроизведением");
    println!("7. 🔊 Громкость");
    println!("8. 💡 Рекомендации");
    println!("9. 📊 Текущий статус");
    println!("0. 🚪 Выход");
    print!("\nВыберите действие: ");
}

fn show_library(player: &MusicPlayer) {
    println!("\n📚 БИБЛИОТЕКА ({} треков):", player.library.len());
    println!("{}", "=".repeat(50));
    
    for (i, song) in player.library.iter().enumerate() {
        println!("{}. {}", i + 1, song.display());
    }
}

fn show_playlists(player: &MusicPlayer) {
    println!("\n📁 ПЛЕЙЛИСТЫ:");
    println!("{}", "=".repeat(50));
    
    if player.playlists.is_empty() {
        println!("Плейлисты отсутствуют");
        return;
    }

    for playlist in player.playlists.values() {
        println!("{}", playlist.display_info());
        
        if let Some(current_playlist) = &player.current_playlist {
            if playlist.name == *current_playlist {
                println!("  ▶️ Сейчас играет");
                if let Some(song) = playlist.get_current_song() {
                    println!("  🎵 {}", song.display());
                }
            }
        }
        
        if playlist.is_shuffle {
            println!("  🔀 Случайный порядок");
        }
        println!();
    }
}

fn search_music(player: &MusicPlayer) {
    println!("🔍 Введите поисковый запрос:");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let query = input.trim();
        let results = player.search_songs(query);
        
        if results.is_empty() {
            println!("❌ Ничего не найдено для '{}'", query);
        } else {
            println!("\n🎯 Результаты поиска ({}):", results.len());
            println!("{}", "=".repeat(50));
            for (i, song) in results.iter().enumerate() {
                println!("{}. {}", i + 1, song.display());
            }
        }
    }
}

fn create_new_playlist(player: &mut MusicPlayer) {
    println!("➕ Введите название нового плейлиста:");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let name = input.trim().to_string();
        if player.create_playlist(name.clone()) {
            println!("✅ Плейлист '{}' создан!", name);
        } else {
            println!("❌ Плейлист с таким названием уже существует!");
        }
    }
}

fn play_playlist_menu(player: &mut MusicPlayer) {
    if player.playlists.is_empty() {
        println!("❌ Нет доступных плейлистов!");
        return;
    }

    println!("▶️ Выберите плейлист для воспроизведения:");
    let playlist_names: Vec<_> = player.playlists.keys().collect();
    
    for (i, name) in playlist_names.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= playlist_names.len() {
                let playlist_name = playlist_names[choice - 1];
                if player.play_playlist(playlist_name) {
                    println!("🎵 Воспроизводится: {}", playlist_name);
                    if let Some(message) = player.next_song() {
                        println!("{}", message);
                    }
                }
            } else {
                println!("❌ Неверный выбор!");
            }
        }
    }
}

fn control_playback(player: &mut MusicPlayer) {
    if player.current_playlist.is_none() {
        println!("❌ Не выбран плейлист для воспроизведения!");
        return;
    }

    println!("\n🎮 УПРАВЛЕНИЕ ВОСПРОИЗВЕДЕНИЕМ:");
    println!("1. ⏭️ Следующий трек");
    println!("2. ⏮️ Предыдущий трек");
    println!("3. 🔀 Переключить перемешивание");
    println!("4. 🔙 Назад");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        match input.trim() {
            "1" => {
                if let Some(message) = player.next_song() {
                    println!("{}", message);
                }
            }
            "2" => {
                if let Some(message) = player.previous_song() {
                    println!("{}", message);
                }
            }
            "3" => {
                let shuffle_status = player.toggle_shuffle();
                println!("🔀 Перемешивание: {}", if shuffle_status { "включено" } else { "выключено" });
            }
            "4" => return,
            _ => println!("❌ Неверный выбор!"),
        }
    }
}

fn manage_volume(player: &mut MusicPlayer) {
    println!("🔊 Текущая громкость: {}%", player.volume);
    println!("Введите новое значение (0-100):");
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(volume) = input.trim().parse::<u8>() {
            player.set_volume(volume);
            println!("🔊 Громкость установлена: {}%", player.volume);
        } else {
            println!("❌ Неверное значение!");
        }
    }
}

fn show_recommendations(player: &MusicPlayer) {
    println!("\n💡 РЕКОМЕНДАЦИИ:");
    println!("{}", "=".repeat(50));
    
    let recommendations = player.get_recommendations();
    for (i, song) in recommendations.iter().enumerate() {
        println!("{}. {}", i + 1, song.display());
    }
}

fn show_current_status(player: &MusicPlayer) {
    println!("\n📊 ТЕКУЩИЙ СТАТУС:");
    println!("{}", "=".repeat(50));
    println!("🔊 Громкость: {}%", player.volume);
    
    if let Some(playlist_name) = &player.current_playlist {
        println!("📁 Активный плейлист: {}", playlist_name);
        
        if let Some(playlist) = player.playlists.get(playlist_name) {
            if let Some(song) = playlist.get_current_song() {
                println!("🎵 Сейчас играет: {}", song.display());
            }
            
            println!("🔀 Перемешивание: {}", if playlist.is_shuffle { "включено" } else { "выключено" });
            println!("📊 Прогресс: {} / {}", 
                     playlist.current_index.map_or(0, |i| i + 1), 
                     playlist.songs.len());
        }
    } else {
        println!("❌ Плейлист не выбран");
    }
    
    println!("📚 Всего треков в библиотеке: {}", player.library.len());
    println!("📁 Всего плейлистов: {}", player.playlists.len());
}
