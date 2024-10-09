# Example of using etsi014-client library in C

### Building and installing

[Install the etsi014-client library](../../REAMDE.md)

```bash
cp src/config.example.h src/config.h
vim src/config.h # Configure client with your kms, certificates and keys
rm -rf build
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Debug ..
make
LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/usr/local/lib" ./etsi014-client-c-test
```