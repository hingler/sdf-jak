
try:
  Import('env')
except:
  env = Environment()

env.Append(CXXFLAGS=["-std=c++17"])

compile_rust = Builder(action = "~/.cargo/bin/cargo build", src_suffix = ".toml")
env.Append(BUILDERS = { 'Rust': compile_rust });
RUST_CARGO = "Cargo.toml"
res = env.Rust(env.Dir("target/debug/libsdf_jak"), RUST_CARGO)

# need to set per platform :3
RUST_LIB_PATH = "target/debug/libsdf_jak" + env["SHLIBSUFFIX"];

env.Append(CPPPATH = [[env.Dir(d) for d in [
  "include"
]]])

gtest = SConscript("./SConstruct_gtest", exports = 'env')

gtest_env = env;
gtest_env.Append(LIBS = [gtest])

base_dir = Dir('#').abspath;

test_dir = "src/test/c_test/"

test_src = [
  "bundle_test"
]

test_deps = [
  [test_dir + test + ".cpp" for test in test_src],
  RUST_LIB_PATH
]

print("dasd")
print(res)

Default(res)
Default(gtest_env.Program(base_dir + "/build/GTEST_sdfjak", test_deps))

