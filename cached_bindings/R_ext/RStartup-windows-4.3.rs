/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.3.3 */

pub type Rstart = * mut structRstart ; # [repr (C)] # [derive (Copy , Clone , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct __BindgenBitfieldUnit < Storage > { storage : Storage , } # [repr (C)] pub struct structRstart { pub R_Quiet : Rboolean , pub R_NoEcho : Rboolean , pub R_Interactive : Rboolean , pub R_Verbose : Rboolean , pub LoadSiteFile : Rboolean , pub LoadInitFile : Rboolean , pub DebugInitFile : Rboolean , pub RestoreAction : SA_TYPE , pub SaveAction : SA_TYPE , pub vsize : usize , pub nsize : usize , pub max_vsize : usize , pub max_nsize : usize , pub ppsize : usize , pub _bitfield_align_1 : [u16 ; 0] , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 4usize] > , # [doc = " R_HOME"] pub rhome : * mut :: std :: os :: raw :: c_char , # [doc = " HOME"] pub home : * mut :: std :: os :: raw :: c_char , pub ReadConsole : :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_char , arg2 : * mut :: std :: os :: raw :: c_uchar , arg3 : :: std :: os :: raw :: c_int , arg4 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int > , pub WriteConsole : :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_char , arg2 : :: std :: os :: raw :: c_int) > , # [doc = " ProcessEvents under Unix"] pub CallBack : :: std :: option :: Option < unsafe extern "C" fn () > , pub ShowMessage : :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_char) > , pub YesNoCancel : :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int > , # [doc = " Return value here is expected to be 1 for Yes, -1 for No and\n0 for Cancel: symbolic constants in graphapp.h"] pub Busy : :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int) > , pub CharacterMode : UImode , # [doc = " The following field has been added in R 2.5.0"] pub WriteConsoleEx : :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_char , arg2 : :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) > , # [doc = " The following field has been added in R 4.0.0."] pub EmitEmbeddedUTF8 : Rboolean , # [doc = " The following fields have been added in R 4.2.0 and are only\navailable with RstarVersion 1."] pub CleanUp : :: std :: option :: Option < unsafe extern "C" fn (arg1 : SA_TYPE , arg2 : :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) > , pub ClearerrConsole : :: std :: option :: Option < unsafe extern "C" fn () > , pub FlushConsole : :: std :: option :: Option < unsafe extern "C" fn () > , pub ResetConsole : :: std :: option :: Option < unsafe extern "C" fn () > , pub Suicide : :: std :: option :: Option < unsafe extern "C" fn (s : * const :: std :: os :: raw :: c_char) > , } pub const RSTART_VERSION : u32 = 1 ; # [repr (i32)] # [non_exhaustive] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum UImode { RGui = 0 , RTerm = 1 , LinkDLL = 2 , } # [repr (i32)] # [non_exhaustive] # [doc = " Startup Actions"] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum SA_TYPE { # [doc = " = 0"] SA_NORESTORE = 0 , SA_RESTORE = 1 , # [doc = " was === SA_RESTORE"] SA_DEFAULT = 2 , SA_NOSAVE = 3 , SA_SAVE = 4 , SA_SAVEASK = 5 , SA_SUICIDE = 6 , } impl < Storage > __BindgenBitfieldUnit < Storage > { # [inline] pub const fn new (storage : Storage) -> Self { Self { storage } } } impl < Storage > __BindgenBitfieldUnit < Storage > where Storage : AsRef < [u8] > + AsMut < [u8] > , { # [inline] fn extract_bit (byte : u8 , index : usize) -> bool { let bit_index = if cfg ! (target_endian = "big") { 7 - (index % 8) } else { index % 8 } ; let mask = 1 << bit_index ; byte & mask == mask } # [inline] pub fn get_bit (& self , index : usize) -> bool { debug_assert ! (index / 8 < self . storage . as_ref () . len ()) ; let byte_index = index / 8 ; let byte = self . storage . as_ref () [byte_index] ; Self :: extract_bit (byte , index) } # [inline] pub unsafe fn raw_get_bit (this : * const Self , index : usize) -> bool { debug_assert ! (index / 8 < core :: mem :: size_of ::< Storage > ()) ; let byte_index = index / 8 ; let byte = unsafe { * (core :: ptr :: addr_of ! ((* this) . storage) as * const u8) . offset (byte_index as isize) } ; Self :: extract_bit (byte , index) } # [inline] fn change_bit (byte : u8 , index : usize , val : bool) -> u8 { let bit_index = if cfg ! (target_endian = "big") { 7 - (index % 8) } else { index % 8 } ; let mask = 1 << bit_index ; if val { byte | mask } else { byte & ! mask } } # [inline] pub fn set_bit (& mut self , index : usize , val : bool) { debug_assert ! (index / 8 < self . storage . as_ref () . len ()) ; let byte_index = index / 8 ; let byte = & mut self . storage . as_mut () [byte_index] ; * byte = Self :: change_bit (* byte , index , val) ; } # [inline] pub unsafe fn raw_set_bit (this : * mut Self , index : usize , val : bool) { debug_assert ! (index / 8 < core :: mem :: size_of ::< Storage > ()) ; let byte_index = index / 8 ; let byte = unsafe { (core :: ptr :: addr_of_mut ! ((* this) . storage) as * mut u8) . offset (byte_index as isize) } ; unsafe { * byte = Self :: change_bit (* byte , index , val) } ; } # [inline] pub fn get (& self , bit_offset : usize , bit_width : u8) -> u64 { debug_assert ! (bit_width <= 64) ; debug_assert ! (bit_offset / 8 < self . storage . as_ref () . len ()) ; debug_assert ! ((bit_offset + (bit_width as usize)) / 8 <= self . storage . as_ref () . len ()) ; let mut val = 0 ; for i in 0 .. (bit_width as usize) { if self . get_bit (i + bit_offset) { let index = if cfg ! (target_endian = "big") { bit_width as usize - 1 - i } else { i } ; val |= 1 << index ; } } val } # [inline] pub unsafe fn raw_get (this : * const Self , bit_offset : usize , bit_width : u8 ,) -> u64 { debug_assert ! (bit_width <= 64) ; debug_assert ! (bit_offset / 8 < core :: mem :: size_of ::< Storage > ()) ; debug_assert ! ((bit_offset + (bit_width as usize)) / 8 <= core :: mem :: size_of ::< Storage > ()) ; let mut val = 0 ; for i in 0 .. (bit_width as usize) { if unsafe { Self :: raw_get_bit (this , i + bit_offset) } { let index = if cfg ! (target_endian = "big") { bit_width as usize - 1 - i } else { i } ; val |= 1 << index ; } } val } # [inline] pub fn set (& mut self , bit_offset : usize , bit_width : u8 , val : u64) { debug_assert ! (bit_width <= 64) ; debug_assert ! (bit_offset / 8 < self . storage . as_ref () . len ()) ; debug_assert ! ((bit_offset + (bit_width as usize)) / 8 <= self . storage . as_ref () . len ()) ; for i in 0 .. (bit_width as usize) { let mask = 1 << i ; let val_bit_is_set = val & mask == mask ; let index = if cfg ! (target_endian = "big") { bit_width as usize - 1 - i } else { i } ; self . set_bit (index + bit_offset , val_bit_is_set) ; } } # [inline] pub unsafe fn raw_set (this : * mut Self , bit_offset : usize , bit_width : u8 , val : u64 ,) { debug_assert ! (bit_width <= 64) ; debug_assert ! (bit_offset / 8 < core :: mem :: size_of ::< Storage > ()) ; debug_assert ! ((bit_offset + (bit_width as usize)) / 8 <= core :: mem :: size_of ::< Storage > ()) ; for i in 0 .. (bit_width as usize) { let mask = 1 << i ; let val_bit_is_set = val & mask == mask ; let index = if cfg ! (target_endian = "big") { bit_width as usize - 1 - i } else { i } ; unsafe { Self :: raw_set_bit (this , index + bit_offset , val_bit_is_set) } ; } } } impl structRstart { # [inline] pub fn NoRenviron (& self) -> Rboolean { unsafe { :: std :: mem :: transmute (self . _bitfield_1 . get (0usize , 16u8) as u32) } } # [inline] pub fn set_NoRenviron (& mut self , val : Rboolean) { unsafe { let val : u32 = :: std :: mem :: transmute (val) ; self . _bitfield_1 . set (0usize , 16u8 , val as u64) } } # [inline] pub unsafe fn NoRenviron_raw (this : * const Self) -> Rboolean { unsafe { :: std :: mem :: transmute (< __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_get (:: std :: ptr :: addr_of ! ((* this) . _bitfield_1) , 0usize , 16u8 ,) as u32) } } # [inline] pub unsafe fn set_NoRenviron_raw (this : * mut Self , val : Rboolean) { unsafe { let val : u32 = :: std :: mem :: transmute (val) ; < __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_set (:: std :: ptr :: addr_of_mut ! ((* this) . _bitfield_1) , 0usize , 16u8 , val as u64 ,) } } # [inline] pub fn RstartVersion (& self) -> :: std :: os :: raw :: c_int { unsafe { :: std :: mem :: transmute (self . _bitfield_1 . get (16usize , 16u8) as u32) } } # [inline] pub fn set_RstartVersion (& mut self , val : :: std :: os :: raw :: c_int) { unsafe { let val : u32 = :: std :: mem :: transmute (val) ; self . _bitfield_1 . set (16usize , 16u8 , val as u64) } } # [inline] pub unsafe fn RstartVersion_raw (this : * const Self) -> :: std :: os :: raw :: c_int { unsafe { :: std :: mem :: transmute (< __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_get (:: std :: ptr :: addr_of ! ((* this) . _bitfield_1) , 16usize , 16u8 ,) as u32) } } # [inline] pub unsafe fn set_RstartVersion_raw (this : * mut Self , val : :: std :: os :: raw :: c_int) { unsafe { let val : u32 = :: std :: mem :: transmute (val) ; < __BindgenBitfieldUnit < [u8 ; 4usize] > > :: raw_set (:: std :: ptr :: addr_of_mut ! ((* this) . _bitfield_1) , 16usize , 16u8 , val as u64 ,) } } # [inline] pub fn new_bitfield_1 (NoRenviron : Rboolean , RstartVersion : :: std :: os :: raw :: c_int) -> __BindgenBitfieldUnit < [u8 ; 4usize] > { let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 4usize] > = Default :: default () ; __bindgen_bitfield_unit . set (0usize , 16u8 , { let NoRenviron : u32 = unsafe { :: std :: mem :: transmute (NoRenviron) } ; NoRenviron as u64 }) ; __bindgen_bitfield_unit . set (16usize , 16u8 , { let RstartVersion : u32 = unsafe { :: std :: mem :: transmute (RstartVersion) } ; RstartVersion as u64 }) ; __bindgen_bitfield_unit } } unsafe extern "C" { pub fn R_DefParams (arg1 : Rstart) ; pub fn R_DefParamsEx (arg1 : Rstart , arg2 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ; # [doc = " New code should always use R_DefParamsEx(Rstart, RSTART_VERSION) to\ninform R about the version of the structure used. R_DefParams(Rstart)\nonly supports version 0 of the structure."] pub fn R_SetParams (arg1 : Rstart) ; pub fn R_DefCallbacks (arg1 : Rstart , arg2 : :: std :: os :: raw :: c_int) ; pub fn R_SetWin32 (arg1 : Rstart) ; pub fn R_SizeFromEnv (arg1 : Rstart) ; pub fn R_common_command_line (arg1 : * mut :: std :: os :: raw :: c_int , arg2 : * mut * mut :: std :: os :: raw :: c_char , arg3 : Rstart) ; pub fn R_set_command_line_arguments (argc : :: std :: os :: raw :: c_int , argv : * mut * mut :: std :: os :: raw :: c_char) ; }