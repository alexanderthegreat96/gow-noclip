mod tools;

fn main() {
    println!(
        "+-----------------------------------+\n\
         | GOW Ragnarok No-Clip v1.3.0       |\n\
         | (c) 2026 alexanderdth             |\n\
         +-----------------------------------+"
    );

    // hook the program
    tools::handler::boot();
}
