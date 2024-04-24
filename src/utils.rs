use arch::pagetable::PageTable;

/// Translate virtual address into physical address in the current virtual address space
///
#[inline]
pub fn virt_to_phys(vaddr: usize) -> Option<usize> {
    PageTable::current()
        .translate(vaddr.into())
        .map(|x| x.0.addr())
}
