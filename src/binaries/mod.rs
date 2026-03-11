mod elf;
mod macho;

pub use self::elf::inspect_elf;
pub use self::macho::inspect_macho;
