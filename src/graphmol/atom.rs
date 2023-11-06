use std::fmt::Formatter;

use cxx::SharedPtr;
use rdkit_sys::ro_mol_ffi;

use crate::ROMol;

pub struct Atom {
    ptr: SharedPtr<rdkit_sys::ro_mol_ffi::Atom>,
}
pub use rdkit_sys::ro_mol_ffi::HybridizationType;

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        f.write_str(&symbol)
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        f.write_str(&symbol)
    }
}

impl Atom {
    pub fn from_ptr(ptr: SharedPtr<ro_mol_ffi::Atom>) -> Self {
        Self { ptr }
    }

    pub fn symbol(&self) -> String {
        ro_mol_ffi::get_symbol(&self.ptr)
    }

    pub fn get_is_aromatic(&self) -> bool {
        ro_mol_ffi::get_is_aromatic(&self.ptr)
    }

    pub fn get_atomic_num(&self) -> i32 {
        ro_mol_ffi::get_atomic_num(&self.ptr)
    }

    pub fn get_formal_charge(&self) -> i32 {
        ro_mol_ffi::get_formal_charge(&self.ptr)
    }

    pub fn get_total_num_hs(&self) -> u32 {
        ro_mol_ffi::get_total_num_hs(&self.ptr)
    }

    pub fn get_total_valence(&self) -> u32 {
        ro_mol_ffi::get_total_valence(&self.ptr)
    }

    pub fn set_formal_charge(&mut self, what: i32) {
        ro_mol_ffi::set_formal_charge(&mut self.ptr, what)
    }

    pub fn set_num_explicit_hs(&mut self, what: i32) {
        ro_mol_ffi::set_num_explicit_hs(&mut self.ptr, what)
    }

    pub fn update_property_cache(&mut self, strict: bool) {
        ro_mol_ffi::atom_update_property_cache(&mut self.ptr, strict)
    }

    pub fn set_hybridization_type(&mut self, what: HybridizationType) {
        ro_mol_ffi::atom_set_hybridization(&mut self.ptr, what);
    }

    pub fn get_hybridization_type(&self) -> HybridizationType {
        ro_mol_ffi::atom_get_hybridization(&self.ptr)
    }
}

pub struct AtomIter<'a> {
    ro_mol: &'a ROMol,
    num_atoms: u32,
    idx: u32,
}

impl<'a> AtomIter<'a> {
    pub fn new(ro_mol: &'a ROMol, only_explicit: bool) -> Self {
        let num_atoms = ro_mol.num_atoms(only_explicit);
        Self {
            ro_mol,
            num_atoms,
            idx: 0,
        }
    }
}

impl<'a> Iterator for AtomIter<'a> {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.num_atoms {
            return None;
        }

        let atom = self.ro_mol.atom_with_idx(self.idx);
        self.idx += 1;

        Some(atom)
    }
}
