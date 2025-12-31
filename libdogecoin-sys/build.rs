use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let vendor_dir = PathBuf::from(&manifest_dir).join("vendor/libdogecoin");

    // --- Build secp256k1 ---
    let secp_src = vendor_dir.join("src/secp256k1");
    
    let mut secp_build = cc::Build::new();
    secp_build
        .include(secp_src.join("include"))
        .include(secp_src.join("src"))
        .file(secp_src.join("src/secp256k1.c"))
        .file(secp_src.join("src/precomputed_ecmult.c"))
        .file(secp_src.join("src/precomputed_ecmult_gen.c"))

        .define("SECP256K1_BUILD", None)
        .define("ENABLE_MODULE_RECOVERY", None)
        .define("USE_NUM_NONE", "1")
        .define("USE_FIELD_INV_BUILTIN", "1")
        .define("USE_SCALAR_INV_BUILTIN", "1")
        .define("ECMULT_GEN_PREC_BITS", "4")
        .define("ECMULT_WINDOW_SIZE", "15")
        // .warnings(false)
        ;

    // Compile secp256k1
    secp_build.compile("secp256k1");

    // --- Build libdogecoin ---
    let mut build = cc::Build::new();
    
    let doge_src = vendor_dir.join("src");
    let doge_include = vendor_dir.join("include");

    build
        .include(&doge_include)
        .include(secp_src.join("include")) // libdogecoin needs secp256k1.h
        // Add all source files manually from Makefile.am
        .file(doge_src.join("address.c"))
        .file(doge_src.join("aes.c"))
        .file(doge_src.join("arith_uint256.c"))
        .file(doge_src.join("auxpow.c"))
        .file(doge_src.join("base58.c"))
        .file(doge_src.join("bip32.c"))
        .file(doge_src.join("bip39.c"))
        .file(doge_src.join("bip44.c"))
        .file(doge_src.join("block.c"))
        .file(doge_src.join("buffer.c"))
        .file(doge_src.join("chacha20.c"))
        .file(doge_src.join("chainparams.c"))
        .file(doge_src.join("cstr.c"))
        .file(doge_src.join("ctaes.c"))
        .file(doge_src.join("ecc.c"))
        .file(doge_src.join("eckey.c"))
        .file(doge_src.join("key.c"))
        .file(doge_src.join("koinu.c"))
        .file(doge_src.join("map.c"))
        .file(doge_src.join("mem.c"))
        .file(doge_src.join("moon.c"))
        .file(doge_src.join("pow.c"))
        .file(doge_src.join("png.c"))
        .file(doge_src.join("jpeg.c"))
        .file(doge_src.join("qrengine.c"))
        .file(doge_src.join("qr.c"))
        .file(doge_src.join("random.c"))
        .file(doge_src.join("rmd160.c"))
        .file(doge_src.join("script.c"))
        .file(doge_src.join("scrypt.c"))
        .file(doge_src.join("seal.c"))
        .file(doge_src.join("sign.c"))
        .file(doge_src.join("serialize.c"))
        .file(doge_src.join("sha2.c"))
        .file(doge_src.join("cli/tool.c"))
        .file(doge_src.join("transaction.c"))
        .file(doge_src.join("tx.c"))
        .file(doge_src.join("utf8proc.c"))
        .file(doge_src.join("utils.c"))
        .file(doge_src.join("validation.c"))
        .file(doge_src.join("vector.c"))
        // Flags
        .define("HAVE_STDLIB_H", None) // minimal config
        .define("HAVE_STRING_H", None)
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        ;

    build.compile("dogecoin");

    // --- Generate Bindings ---
    println!("cargo:rerun-if-changed=vendor/libdogecoin/include/dogecoin/libdogecoin.h");

    let bindings = bindgen::Builder::default()
        .header(vendor_dir.join("include/dogecoin/libdogecoin.h").to_str().unwrap())
        .clang_arg(format!("-I{}", vendor_dir.join("include").display()))
        // We also need secp include path for binding generation if headers refer to it
        .clang_arg(format!("-I{}", secp_src.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
