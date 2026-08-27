mod tools;

fn main() {
    println!(
        "+-----------------------------------+\n\
         | GOW Ragnarok No-Clip v1.3.1       |\n\
         | PS: Run the program as admin      |\n\
         | (c) 2026 alexanderdth             |\n\
         +-----------------------------------+"
    );

    // hook the program
    tools::handler::boot();
}
