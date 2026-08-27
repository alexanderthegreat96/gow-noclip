use crate::tools::memory_mappings;
use libmem::*;

pub struct Game {
    pub process: Process,
    pub base: usize,
    pub size: usize,
    pub camera_base: usize,
    pub entity_slot: usize,
}

impl Game {
    pub fn attach() -> Option<Self> {
        let process = find_process(memory_mappings::GOW_PROC_NAME)?;
        let module = find_module_ex(&process, memory_mappings::GOW_PROC_NAME)?;

        let mut game = Self {
            process,
            base: module.base,
            size: module.size,
            camera_base: module.base + memory_mappings::camera::BASE,
            entity_slot: module.base + memory_mappings::entity::BASE,
        };

        if let Some(m) = game.sig_scan(memory_mappings::camera::SIG) {
            if let Some(addr) = game.resolve_rip_rel(m, 3, 7) {
                game.camera_base = addr;
                println!(
                    "camera sig hit 0x{:X} → data ptr RVA 0x{:X}",
                    m,
                    addr - game.base
                );
            }
        } else {
            println!(
                "camera sig not found — falling back to hardcoded RVA 0x{:X}",
                memory_mappings::camera::BASE
            );
        }

        if let Some(m) = game.sig_scan(memory_mappings::entity::SIG) {
            game.entity_slot = m;
            println!("player entity sig hit → slot RVA 0x{:X}", m - game.base);
        } else {
            println!(
                "player entity sig not found — falling back to hardcoded RVA 0x{:X}",
                memory_mappings::entity::BASE
            );
        }

        Some(game)
    }

    pub fn is_still_alive() -> bool {
        find_process(memory_mappings::GOW_PROC_NAME).is_some()
    }

    pub fn camera_ptr(&self) -> Option<usize> {
        self.read::<usize>(self.camera_base)
    }

    pub fn entity_ptr(&self) -> Option<usize> {
        self.read::<usize>(self.entity_slot)
    }

    pub fn sig_scan(&self, pattern: &str) -> Option<usize> {
        const SCAN_LIMIT: usize = 0x4000000; // 64MB 
        let size = self.size.min(SCAN_LIMIT);
        libmem::sig_scan_ex(&self.process, pattern, self.base, size)
    }

    pub fn resolve_rip_rel(
        &self,
        addr: usize,
        disp_offset: usize,
        instr_len: usize,
    ) -> Option<usize> {
        let disp = self.read::<i32>(addr + disp_offset)? as isize;
        let next_ip = (addr + instr_len) as isize;
        Some((next_ip + disp) as usize)
    }

    pub fn read<T: Copy>(&self, addr: usize) -> Option<T> {
        libmem::read_memory_ex::<T>(&self.process, addr)
    }

    pub fn write<T: Copy>(&self, addr: usize, val: &T) -> Option<()> {
        libmem::write_memory_ex(&self.process, addr, val)
    }
}
