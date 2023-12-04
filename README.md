```
docker build -t rust-img-proc-simply .
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  rust-img-proc-simply \
  cp -r /workspace/dist /
```

Or

```
docker build -t rust-img-proc-simply .
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  -v $(pwd)/src/main.rs:/workspace/test-img-proc/src/main.rs \
  -v $(pwd)/src/input.jpg:/workspace/test-img-proc/input.jpg \
  rust-img-proc-simply \
  /bin/bash -c \
    "cargo run && cp -r /workspace/dist /"
```
