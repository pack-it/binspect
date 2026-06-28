mod elf;
mod macho;
mod pe;

pub use self::elf::handle_elf;
pub use self::macho::handle_macho;
pub use self::pe::handle_pe;
