mod elf;
mod macho;
mod pe;

pub use self::elf::inspect_elf;
pub use self::macho::inspect_macho;
pub use self::pe::inspect_pe;
