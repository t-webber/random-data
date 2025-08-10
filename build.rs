fn main() {
    if !(cfg!(feature = "address")
        || cfg!(feature = "animals")
        || cfg!(feature = "art")
        || cfg!(feature = "colours")
        || cfg!(feature = "computer")
        || cfg!(feature = "currencies")
        || cfg!(feature = "datetime")
        || cfg!(feature = "education")
        || cfg!(feature = "finance")
        || cfg!(feature = "france")
        || cfg!(feature = "history")
        || cfg!(feature = "internet")
        || cfg!(feature = "minimal")
        || cfg!(feature = "names")
        || cfg!(feature = "people")
        || cfg!(feature = "personal")
        || cfg!(feature = "science")
        || cfg!(feature = "sky_space")
        || cfg!(feature = "text")
        || cfg!(feature = "uk")
        || cfg!(feature = "us")
        || cfg!(feature = "work")
        || cfg!(feature = "world"))
    {
        panic!("\x1b[31mYou must enable at least one feature!\x1b[0m");
    }
}
