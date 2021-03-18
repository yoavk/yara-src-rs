use std::env;
use std::path::PathBuf;

pub fn build() {
    let basedir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yara");

    let mut cc = cc::Build::new();

    cc
        .include(basedir.join("libyara"))
        .include(basedir.join("libyara/include"))

        .file(basedir.join("libyara/ahocorasick.c"))
        .file(basedir.join("libyara/arena.c"))
        .file(basedir.join("libyara/atoms.c"))
        .file(basedir.join("libyara/bitmask.c"))
        .file(basedir.join("libyara/compiler.c"))
        .file(basedir.join("libyara/endian.c"))
        .file(basedir.join("libyara/exec.c"))
        .file(basedir.join("libyara/exefiles.c"))
        .file(basedir.join("libyara/filemap.c"))
        .file(basedir.join("libyara/grammar.c"))
        .file(basedir.join("libyara/hash.c"))
        .file(basedir.join("libyara/hex_grammar.c"))
        .file(basedir.join("libyara/hex_lexer.c"))
        .file(basedir.join("libyara/lexer.c"))
        .file(basedir.join("libyara/libyara.c"))
        .file(basedir.join("libyara/mem.c"))
        .file(basedir.join("libyara/object.c"))
        .file(basedir.join("libyara/parser.c"))
        .file(basedir.join("libyara/proc.c"))
        .file(basedir.join("libyara/re.c"))
        .file(basedir.join("libyara/re_grammar.c"))
        .file(basedir.join("libyara/re_lexer.c"))
        .file(basedir.join("libyara/rules.c"))
        .file(basedir.join("libyara/scan.c"))
        .file(basedir.join("libyara/scanner.c"))
        .file(basedir.join("libyara/sizedstr.c"))
        .file(basedir.join("libyara/stack.c"))
        .file(basedir.join("libyara/stopwatch.c"))
        .file(basedir.join("libyara/stream.c"))
        .file(basedir.join("libyara/strutils.c"))
        .file(basedir.join("libyara/threading.c"))

        .file(basedir.join("libyara/modules.c"))
        .file(basedir.join("libyara/modules/elf.c"))
        .file(basedir.join("libyara/modules/math.c"))
        .file(basedir.join("libyara/modules/pe.c"))
        .file(basedir.join("libyara/modules/pe_utils.c"))
        .file(basedir.join("libyara/modules/tests.c"))
        .file(basedir.join("libyara/modules/time.c"))

        .define("DEX_MODULE", "")
        .file(basedir.join("libyara/modules/dex.c"))

        .define("DOTNET_MODULE", "")
        .file(basedir.join("libyara/modules/dotnet.c"))

        .define("MACHO_MODULE", "")
        .file(basedir.join("libyara/modules/macho.c"));

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Use correct proc functions
    if target_os == "windows" {
        cc
            .file(basedir.join("libyara/proc/windows.c"))
            .define("USE_WINDOWS_PROC", "");
    } else if target_os == "linux" {
        cc
            .file(basedir.join("libyara/proc/linux.c"))
            .define("USE_LINUX_PROC", "");
    } else if target_os == "darwin" {
        cc
            .file(basedir.join("libyara/proc/mach.c"))
            .define("USE_MACH_PROC", "");
    } else {
        cc
            .file(basedir.join("libyara/proc/none.c"))
            .define("USE_NO_PROC", "");
    }

    if target_os == "windows" {
        cc.define("NDEBUG", "1");
    }
    else {
        cc.define("POSIX", "");
    }

    // Unfortunately, YARA compilation produces lots of warnings
    cc.warnings(false);

    cc.compile("yara");
}

fn include_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yara/libyara/include")
}

fn lib_dir() -> PathBuf {
    std::env::var("OUT_DIR").unwrap().into()
}

pub fn set_env() {
    env::set_var("LIBYARA_STATIC", "1");
    env::set_var("YARA_INCLUDE_DIR", include_dir());
    env::set_var("YARA_LIBRARY_PATH", lib_dir());
}

pub fn print_cargo_metadata() {
    println!("cargo:rustc-link-search=native={}", lib_dir().display());
    println!("cargo:rustc-link-lib=static=yara");
    println!("cargo:include={}", include_dir().display());
    println!("cargo:lib={}", lib_dir().display());
}
