#[derive(Debug, Clone)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    // immutable struct value (self parameter takes ownership)
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn print_self(&self) {
        println!("{:#?}", self);
    }

    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    fn double_length(mut self) {
        self.duration_secs = self.duration_secs * 2;
        self.print_self();
    }

    // Immutable reference to the struct instance (no ownership moved)

    fn double_length_ref(&self) {
        let mut clone: TaylorSwiftSong = self.clone();
        clone.duration_secs *= 2;
        clone.print_self();
    }

    // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
    fn double_length_ref_mut(&mut self) {
        self.duration_secs *= 2;
        self.print_self();
    }
}

fn main() {
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.print_self();
    // song.double_length();
    song.double_length_ref();

    let mut mut_song = song;
    mut_song.double_length_ref_mut();
}
