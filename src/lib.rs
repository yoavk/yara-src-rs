use std::env;
use std::path::PathBuf;

pub fn build() {
    let mut cc = cc::Build::new();

    cc
        .include("../yara/libyara")
        .include("../yara/libyara/include")

        .file("../yara/libyara/ahocorasick.c")
        .file("../yara/libyara/arena.c")
        .file("../yara/libyara/atoms.c")
        .file("../yara/libyara/bitmask.c")
        .file("../yara/libyara/compiler.c")
        .file("../yara/libyara/endian.c")
        .file("../yara/libyara/exec.c")
        .file("../yara/libyara/exefiles.c")
        .file("../yara/libyara/filemap.c")
        .file("../yara/libyara/grammar.c")
        .file("../yara/libyara/hash.c")
        .file("../yara/libyara/hex_grammar.c")
        .file("../yara/libyara/hex_lexer.c")
        .file("../yara/libyara/lexer.c")
        .file("../yara/libyara/libyara.c")
        .file("../yara/libyara/mem.c")
        .file("../yara/libyara/object.c")
        .file("../yara/libyara/parser.c")
        .file("../yara/libyara/proc.c")
        .file("../yara/libyara/re.c")
        .file("../yara/libyara/re_grammar.c")
        .file("../yara/libyara/re_lexer.c")
        .file("../yara/libyara/rules.c")
        .file("../yara/libyara/scan.c")
        .file("../yara/libyara/scanner.c")
        .file("../yara/libyara/sizedstr.c")
        .file("../yara/libyara/stack.c")
        .file("../yara/libyara/stopwatch.c")
        .file("../yara/libyara/stream.c")
        .file("../yara/libyara/strutils.c")
        .file("../yara/libyara/threading.c")

        .file("../yara/libyara/modules.c")
        .file("../yara/libyara/modules/elf.c")
        .file("../yara/libyara/modules/math.c")
        .file("../yara/libyara/modules/pe.c")
        .file("../yara/libyara/modules/pe_utils.c")
        .file("../yara/libyara/modules/tests.c")
        .file("../yara/libyara/modules/time.c")

        .define("DEX_MODULE", "")
        .file("../yara/libyara/modules/dex.c")

        .define("DOTNET_MODULE", "")
        .file("../yara/libyara/modules/dotnet.c")

        .define("MACHO_MODULE", "")
        .file("../yara/libyara/modules/macho.c");

    // Use correct proc functions
    if cfg!(windows) {
        cc
            .file("../yara/libyara/proc/windows.c")
            .define("USE_WINDOWS_PROC", "");
    } else if cfg!(linux) {
        cc
            .file("../yara/libyara/proc/linux.c")
            .define("USE_LINUX_PROC", "");
    } else if cfg!(darwin) {
        cc
            .file("../yara/libyara/proc/mach.c")
            .define("USE_MACH_PROC", "");
    } else {
        cc
            .file("../yara/libyara/proc/none.c")
            .define("USE_NO_PROC", "");
    }

    if cfg!(windows) {
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
    std::fs::canonicalize("../yara/libyara/include").unwrap()
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
