use itertools::Itertools;
use smallvec::smallvec;
use std::sync::Arc;

use std::result::Result::Ok;

use crate::{
    mesh::{self, GpuMesh},
    renderer::MeshInstance,
    RenderContext, Rgba32Unmul,
};

#[derive(thiserror::Error, Debug)]
pub enum DaeImportError {
    #[error("Error loading DAE mesh: {mesh_name:?}")]
    DaeLoading { mesh_name: String },

    #[error(transparent)]
    MeshError(#[from] mesh::MeshError),
}

pub fn load_dae_from_buffer(
    mesh_name: &str,
    buffer: &[u8],
    ctx: &RenderContext,
) -> Result<Vec<MeshInstance>, DaeImportError> {
    re_tracing::profile_function!();

    let scene = file_parser::collada::from_slice(buffer);
    let mut mesh_instances = Vec::<MeshInstance>::new();

    let mut i = 0;
    if let Ok(scene) = scene {
        for mesh in &scene.meshes {
            let num_vertices = mesh.vertices.len();

            let triangle_indices = (0..num_vertices as u32)
                .tuples::<(_, _, _)>()
                .map(glam::UVec3::from)
                .collect::<Vec<_>>();
            let vertex_positions: Vec<glam::Vec3> = mesh
                .vertices
                .clone()
                .into_iter()
                .map(glam::Vec3::from_array)
                .collect();

            let num_positions = vertex_positions.len();

            let vertex_normals = if !mesh.normals.is_empty() {
                re_tracing::profile_scope!("collect normals");
                mesh.normals
                    .clone()
                    .into_iter()
                    .map(glam::Vec3::from_array)
                    .collect()
            } else {
                vec![glam::Vec3::ZERO; num_positions]
            };

            let mut vertex_colors = Vec::new();

            if !mesh.colors.is_empty() && !mesh.colors[i].is_empty() {
                re_tracing::profile_scope!("copy_colors");
                vertex_colors.extend(mesh.colors.iter().flat_map(|vec_of_colors| {
                    vec_of_colors.iter().map(|color| {
                        let rgba_unmul = [
                            (color[0] * 255.0) as u8,
                            (color[1] * 255.0) as u8,
                            (color[2] * 255.0) as u8,
                            (color[3] * 255.0) as u8,
                        ];
                        Rgba32Unmul::from_rgba_unmul_array(rgba_unmul)
                    })
                }));
            } else {
                vertex_colors.resize(vertex_positions.len(), Rgba32Unmul::WHITE);
            };

            let mut vertex_texcoords = Vec::new();

            if !mesh.texcoords.is_empty() && !mesh.texcoords[0].is_empty() {
                vertex_texcoords.extend(mesh.texcoords.iter().flat_map(|textcoord_vec| {
                    textcoord_vec
                        .iter()
                        .map(|&textcoord| glam::Vec2::from(textcoord))
                }));
            } else {
                vertex_texcoords.resize(num_vertices, glam::Vec2::ZERO);
            }

            let albedo_factor = {
                if let Some(diffuse) = scene.materials[i].color.diffuse {
                    let [r, g, b, a] = diffuse;
                    crate::Rgba::from_rgba_unmultiplied(r, g, b, a)
                } else {
                    crate::Rgba::WHITE
                }
            };

            let material = mesh::Material {
                label: "default material".into(),
                index_range: 0..num_vertices as u32,
                albedo: ctx.texture_manager_2d.white_texture_unorm_handle().clone(),
                albedo_factor,
            };

            println!("{:?}", scene.materials[i]);
            let m = mesh::Mesh {
                label: mesh_name.into(),
                triangle_indices,
                vertex_positions,
                vertex_normals,
                vertex_colors,
                vertex_texcoords,
                materials: smallvec![material],
            };

            m.sanity_check()?;

            mesh_instances.push(MeshInstance::new_with_cpu_mesh(
                Arc::new(GpuMesh::new(ctx, &m)?),
                Some(Arc::new(m)),
            ));
            i += 1;
        }
    } else {
        return Err(DaeImportError::DaeLoading {
            mesh_name: mesh_name.to_owned(),
        });
    }
    Ok(mesh_instances)
}
