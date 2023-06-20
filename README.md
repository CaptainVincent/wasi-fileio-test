# Fileio test suite

Comparing the behavior of running filesystem I/O tests.

## Test Linux behavior
```bash
./initial.sh
cargo run -- 'Linux'

Re-run test for: Linux
Failed to '1. create a new sub folder': Permission denied (os error 13)
Failed to '2. rename the sub folder': Permission denied (os error 13)
Failed to '3. delete the sub folder': Permission denied (os error 13)
Failed to '5. write into folder': Permission denied (os error 13)
+---------------------------------------+-------+
| Test Cases \ Label                    | Linux |
+---------------------------------------+-------+
| 1. create a new sub folder            | x     |
+---------------------------------------+-------+
| 2. rename the sub folder              | x     |
+---------------------------------------+-------+
| 3. delete the sub folder              | x     |
+---------------------------------------+-------+
| 4. read folder                        | o     |
+---------------------------------------+-------+
| 5. write into folder                  | x     |
+---------------------------------------+-------+
| 6. readonly (metadata) before changed | true  |
+---------------------------------------+-------+
| 7. set permissions                    | o     |
+---------------------------------------+-------+
| 8. readonly (metadata) after changed  | false |
+---------------------------------------+-------+
```

## Test wasmedge from installer

Build wasm first
```bash
cargo build --target wasm32-wasi
```

Then
```bash
./initial.sh
~/.wasmedge/bin/wasmedge --dir ./readonlyFolder:./readonlyFolder:readonly --dir .:. target/wasm32-wasi/debug/wasi-file-test.wasm WasmEdge-0.12.1

Re-run test for: WasmEdge-0.12.1
Failed to '1. create a new sub folder': Capabilities insufficient (os error 76)
Failed to '2. rename the sub folder': Capabilities insufficient (os error 76)
Failed to '3. delete the sub folder': Capabilities insufficient (os error 76)
Failed to '4. read folder': Capabilities insufficient (os error 76)
Failed to '5. write into folder': Capabilities insufficient (os error 76)
Failed to '7. set permissions': operation not supported on this platform
+---------------------------------------+-------+-----------------+
| Test Cases \ Label                    | Linux | WasmEdge-0.12.1 |
+---------------------------------------+-------+-----------------+
| 1. create a new sub folder            | x     | x               |
+---------------------------------------+-------+-----------------+
| 2. rename the sub folder              | x     | x               |
+---------------------------------------+-------+-----------------+
| 3. delete the sub folder              | x     | x               |
+---------------------------------------+-------+-----------------+
| 4. read folder                        | o     | x               |
+---------------------------------------+-------+-----------------+
| 5. write into folder                  | x     | x               |
+---------------------------------------+-------+-----------------+
| 6. readonly (metadata) before changed | true  | false           |
+---------------------------------------+-------+-----------------+
| 7. set permissions                    | o     | x               |
+---------------------------------------+-------+-----------------+
| 8. readonly (metadata) after changed  | false | false           |
+---------------------------------------+-------+-----------------+

```

## Test wasmedge build from source
Make sure you export your source build wasmedge `PATH` first and build wasm
```bash
cargo build --target wasm32-wasi
```

Then
```bash
./initial.sh
wasmedge --dir ./readonlyFolder:./readonlyFolder:readonly --dir .:. target/wasm32-wasi/debug/wasi-file-test.wasm WasmEdge-PR2586

Re-run test for: WasmEdge-PR2586
Failed to '1. create a new sub folder': Capabilities insufficient (os error 76)
Failed to '2. rename the sub folder': Capabilities insufficient (os error 76)
Failed to '3. delete the sub folder': Capabilities insufficient (os error 76)
Failed to '5. write into folder': Capabilities insufficient (os error 76)
Failed to '7. set permissions': operation not supported on this platform
+---------------------------------------+-------+-----------------+-----------------+
| Test Cases \ Label                    | Linux | WasmEdge-0.12.1 | WasmEdge-PR2586 |
+---------------------------------------+-------+-----------------+-----------------+
| 1. create a new sub folder            | x     | x               | x               |
+---------------------------------------+-------+-----------------+-----------------+
| 2. rename the sub folder              | x     | x               | x               |
+---------------------------------------+-------+-----------------+-----------------+
| 3. delete the sub folder              | x     | x               | x               |
+---------------------------------------+-------+-----------------+-----------------+
| 4. read folder                        | o     | x               | o               |
+---------------------------------------+-------+-----------------+-----------------+
| 5. write into folder                  | x     | x               | x               |
+---------------------------------------+-------+-----------------+-----------------+
| 6. readonly (metadata) before changed | true  | false           | false           |
+---------------------------------------+-------+-----------------+-----------------+
| 7. set permissions                    | o     | x               | x               |
+---------------------------------------+-------+-----------------+-----------------+
| 8. readonly (metadata) after changed  | false | false           | false           |
+---------------------------------------+-------+-----------------+-----------------+
```

## Reset report
```
rm report.json
```