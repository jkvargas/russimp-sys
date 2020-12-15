git clone https://github.com/microsoft/vcpkg
.\vcpkg\bootstrap-vcpkg.bat

.\vcpkg\vcpkg install assimp

cp .\vcpkg\installed\x86-windows\libassimp-vc142-mt.lib .\vcpkg\installed\x86-windows\assimp.lib

cd .\vcpkg
Get-ChildItem -Recurse