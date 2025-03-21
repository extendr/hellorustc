/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.6.0 */

#[repr(u32)]
#[non_exhaustive]
#[doc = "#ifdef HAVE_VISIBILITY_ATTRIBUTE\n# define attribute_hidden __attribute__ ((visibility (\"hidden\")))\n#else\n# define attribute_hidden\n#endif"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AlgType {
    NREG = 1,
    OPT = 2,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " 0-based indices into v"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VPos {
    F = 9,
    F0 = 12,
    FDIF = 10,
    G = 27,
    HC = 70,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " 0-based indices into iv"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IVPos {
    AI = 90,
    AM = 94,
    ALGSAV = 50,
    COVMAT = 25,
    COVPRT = 13,
    COVREQ = 14,
    DRADPR = 100,
    DTYPE = 15,
    IERR = 74,
    INITH = 24,
    IPIVOT = 75,
    IVNEED = 2,
    LASTIV = 42,
    LASTV = 44,
    LMAT = 41,
    MXFCAL = 16,
    MXITER = 17,
    NEXTV = 46,
    NFCALL = 5,
    NFCOV = 51,
    NFGCAL = 6,
    NGCOV = 52,
    NITER = 30,
    NVDFLT = 49,
    NVSAVE = 8,
    OUTLEV = 18,
    PARPRT = 19,
    PARSAV = 48,
    PERM = 57,
    PRUNIT = 20,
    QRTYP = 79,
    RDREQ = 56,
    RMAT = 77,
    SOLPRT = 21,
    STATPR = 22,
    TOOBIG = 1,
    VNEED = 3,
    VSAVE = 59,
    X0PRT = 23,
}
impl IVPos {
    pub const INITS: IVPos = IVPos::INITH;
}
