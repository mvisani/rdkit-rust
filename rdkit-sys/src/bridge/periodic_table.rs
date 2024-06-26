#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/periodic_table.h");
        pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32>;
        pub fn get_most_common_isotope_mass(atom: &CxxString) -> f64;
    }
}
