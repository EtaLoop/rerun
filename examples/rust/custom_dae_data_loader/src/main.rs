use rerun::{
    external::{anyhow, re_build_info, re_data_loader, re_log},
    log::{Chunk, RowId},
    EntityPath, Rgba32, TimePoint,
};
use rerun::LoadedData;
// use rerun::dae

fn main() -> anyhow::Result<std::process::ExitCode> {
    re_log::setup_logging();

    re_data_loader::register_custom_data_loader(DaeLoader);

    let build_info = re_build_info::build_info!();
    rerun::run(build_info, rerun::CallSource::Cli, std::env::args())
        .map(std::process::ExitCode::from)
}

struct DaeLoader;

impl re_data_loader::DataLoader for DaeLoader {
    fn name(&self) -> String {
        "rerun.data_loaders.DaeLoader".into()
    }

    fn load_from_path(
        &self,
        settings: &rerun::external::re_data_loader::DataLoaderSettings,
        path: std::path::PathBuf,
        tx: std::sync::mpsc::Sender<re_data_loader::LoadedData>,
    ) -> Result<(), re_data_loader::DataLoaderError> {
        let contents = std::fs::read(&path)?;
        if path.is_dir() {
            return Err(re_data_loader::DataLoaderError::Incompatible(path)); // simply not interested
        }
        load_collada(settings, &tx, &path, &contents)
    }

    fn load_from_file_contents(
        &self,
        settings: &rerun::DataLoaderSettings,
        filepath: std::path::PathBuf,
        contents: std::borrow::Cow<'_, [u8]>,
        tx: std::sync::mpsc::Sender<rerun::LoadedData>,
    ) -> Result<(), rerun::DataLoaderError> {
        load_collada(settings, &tx, &filepath, &contents)
    }
}

fn load_collada(
        settings: &rerun::DataLoaderSettings,
        tx: &std::sync::mpsc::Sender<re_data_loader::LoadedData>,
    filepath: &std::path::Path,
    contents: &[u8],
) -> Result<(), re_data_loader::DataLoaderError> {
    use mesh_loader::collada;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    let mut h = DefaultHasher::new();
    contents.hash(&mut h);

    let scene = collada::from_slice(contents);

    if let Ok(scene) = scene {
        for (mesh, mat) in scene.meshes.iter().zip(scene.materials.iter()) {
            // println!("verticce : {:?} | \nnormals : {:?}", mesh.vertices, mesh.normals);
            let mut mesh3d = rerun::Mesh3D::new(&mesh.vertices);

            if !mesh.normals.is_empty() && !mesh.normals[0].is_empty() {
                mesh3d = mesh3d.with_vertex_normals(&mesh.normals);
            }

            if let Some(diffuse) = &mat.color.diffuse {
                mesh3d = mesh3d.with_albedo_factor(Rgba32::from_linear_unmultiplied_rgba_f32(
                    diffuse[0], diffuse[1], diffuse[2], diffuse[3],
                ));
            }

            let entity_path = EntityPath::from_file_path(filepath);
            let chunk = Chunk::builder(entity_path)
                .with_archetype(RowId::new(), TimePoint::default(), &mesh3d)
                .build()?;

            let store_id = settings
                .opened_store_id
                .clone()
                .unwrap_or_else(|| settings.store_id.clone());
            let data = LoadedData::Chunk(store_id, chunk);

            tx.send(data).ok();
        }
    }

    Ok(())
}
