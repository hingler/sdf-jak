
try:
  Import('env')
except:
  env = Environment()

env.Append(CXXFLAGS=["-std=c++17"])
RUST_LIB_PATH = "target/debug/libsdf_jak" + env["SHLIBSUFFIX"];

RUST_CARGO = "Cargo.toml"
res = env.Command(
  target = RUST_LIB_PATH,
  source = "Cargo.toml",
  action = "cargo build",
  chdir  = env.Dir(".")
)

# need to set per platform :3

env.Append(CPPPATH = [[env.Dir(d) for d in [
  "include"
]]])

gtest = SConscript("./SConstruct_gtest", exports = 'env')

gtest_env = env.Clone();
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

print("constructed SDF!");

Default(gtest_env.Program(base_dir + "/build/GTEST_sdfjak", test_deps))
Return("res")

