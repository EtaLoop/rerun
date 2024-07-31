---
title: "LeafTransformMat3x3"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

A 3x3 transformation matrix Matrix that doesn't propagate in the transform hierarchy.

3x3 matrixes are able to represent any affine transformation in 3D space,
i.e. rotation, scaling, shearing, reflection etc.

Matrices in Rerun are stored as flat list of coefficients in column-major order:
```text
            column 0       column 1       column 2
       -------------------------------------------------
row 0 | flat_columns[0] flat_columns[3] flat_columns[6]
row 1 | flat_columns[1] flat_columns[4] flat_columns[7]
row 2 | flat_columns[2] flat_columns[5] flat_columns[8]
```

## Fields

* matrix: [`Mat3x3`](../datatypes/mat3x3.md)

## API reference links
 * 🌊 [C++ API docs for `LeafTransformMat3x3`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1components_1_1LeafTransformMat3x3.html?speculative-link)
 * 🐍 [Python API docs for `LeafTransformMat3x3`](https://ref.rerun.io/docs/python/stable/common/components?speculative-link#rerun.components.LeafTransformMat3x3)
 * 🦀 [Rust API docs for `LeafTransformMat3x3`](https://docs.rs/rerun/latest/rerun/components/struct.LeafTransformMat3x3.html?speculative-link)


## Used by

* [`LeafTransforms3D`](../archetypes/leaf_transforms3d.md)