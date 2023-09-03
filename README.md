# nanogltf

![helmet](https://github.com/rust-gamedev/rust-gamedev.github.io/assets/910977/9edf5369-755b-415f-aa57-ca88f414615d)
*miniquad(gl2+, metal) [viewer](https://github.com/not-fl3/nanogltf/tree/master/examples/viewer) example*

`nanoserde` based gltf 2.0 loader.

It can load most of the [Sample Models](https://github.com/KhronosGroup/glTF-Sample-Models) and it can load fairly complex blender exported scenes.

With very little code introduced into the source tree:
```
nanogltf v0.1.0
└── nanoserde v0.1.33
    └── nanoserde-derive v0.1.20 (proc-macro)
```

For a feature complete gltf loader, check [gltf-rs](https://github.com/gltf-rs/gltf).
