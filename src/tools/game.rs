use crate::tools::memory_mappings;
use libmem::*;

pub struct Game {
    pub process: Process,
    pub base: usize,
}

impl Game {
    pub fn attach() -> Option<Self> {
        let process = find_process(memory_mappings::GOW_PROC_NAME)?;
        let module = find_module_ex(&process, memory_mappings::GOW_PROC_NAME)?;
        Some(Self {
            process,
            base: module.base,
        })
    }

    pub fn is_still_alive() -> bool {
        find_process(memory_mappings::GOW_PROC_NAME).is_some()
    }

    pub fn resolve(&self, chain: (usize, &[usize])) -> Option<usize> {
        let (root, offsets) = chain;
        let mut addr = self.base + root;
        for offset in offsets.iter() {
            let ptr = self.read::<usize>(addr)?;
            addr = ptr + offset;
        }
        Some(addr)
    }

    pub fn read<T: Copy>(&self, addr: usize) -> Option<T> {
        libmem::read_memory_ex::<T>(&self.process, addr)
    }

    pub fn write<T: Copy>(&self, addr: usize, val: &T) -> Option<()> {
        libmem::write_memory_ex(&self.process, addr, val)
    }
}
