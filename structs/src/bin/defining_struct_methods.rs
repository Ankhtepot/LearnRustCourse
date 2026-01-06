#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(self) {
        println!("| ********* Displaying Song ********* |");
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn display_song_info_with_ref(&self) {
        println!("| ********* Displaying Song ********* |");
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn overwrite_title(&mut self, new_title: &str) {
        self.title = String::from(new_title);
    }
}

fn main() {
    let mut song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    // song.display_song_info();
    song.display_song_info_with_ref();
    song.overwrite_title("Love Story");
    song.display_song_info();
}
