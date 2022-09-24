/*
 * Archive:
 *      Tar: Tarball, primary default option
 *      Zip: Zip compression method, secondary default option
 *      Gzip: Gzip compression method, last default option 
 *      when other options are inactivated
 *      Bzip: Bzip selected when 
 */
enum Archive {
    Tar,
    Zip,
    Gzip,
    Bzip
}

/*
 * Binary:
 *      ExeX86: x86 Windows PE
 *      ExeX86_64: x86_64 Windows PE
 *      ElfX86: x86 Linux ELF
 *      ElfX86_64: x86_64 Linux ELF
 */
enum Binary {
    ExeX86,
    ExeX86_64,
    ElfX86,
    ElfX86_64
}

enum Errno {
    ENOADDR = 0,
    ENONAME = 1,
    ENOVER = 2
}

/*
 * name: package name, mandatory
 * addr: package addr (where install from), mandatory (not mandatory if just installing, no download from internet)
 * z: zip, tar, gzip etc. not mandatory
 * version: version number, mandatory
 * binary: binary type for compability of packages
 * author: author, may be anonymous; not mandatory
 */
pub struct Package {
    name: Result<String, Errno>,
    addr: Result<String, Errno>,
    version: Result<String, Errno>,
    binary: Option<Binary>,
    z: Option<Archive>,
    author: Option<String>
}
