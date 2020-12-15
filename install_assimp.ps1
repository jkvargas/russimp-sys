$path_assimp_repo = "$GITHUB_WORKSPACE\assimp"
$path_assimp_repo_build = "$GITHUB_WORKSPACE\assimp\build"
$path_assimp_out = "$GITHUB_WORKSPACE\out"

git clone --depth 1 --branch v5.0.1 https://github.com/assimp/assimp.git ${path_assimp_repo}

mkdir $path_assimp_repo_build

cd $path_assimp_repo_build

cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_C_COMPILER=clang -DCMAKE_INSTALL_PREFIX=path_assimp_out -G Ninja ..
ninja
ninja install