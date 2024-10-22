use rerun::{
    external::{anyhow, re_build_info, re_data_loader, re_log},
    log::{Chunk, RowId},
    EntityPath, TimePoint,
};

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
        _settings: &rerun::external::re_data_loader::DataLoaderSettings,
        path: std::path::PathBuf,
        tx: std::sync::mpsc::Sender<re_data_loader::LoadedData>,
    ) -> Result<(), re_data_loader::DataLoaderError> {
        let contents = std::fs::read(&path)?;
        if path.is_dir() {
            return Err(re_data_loader::DataLoaderError::Incompatible(path)); // simply not interested
        }
        load_collada(&tx, &path, &contents)
    }

    fn load_from_file_contents(
        &self,
        _settings: &rerun::DataLoaderSettings,
        filepath: std::path::PathBuf,
        contents: std::borrow::Cow<'_, [u8]>,
        tx: std::sync::mpsc::Sender<rerun::LoadedData>,
    ) -> Result<(), rerun::DataLoaderError> {
        load_collada(&tx, &filepath, &contents)
    }
}

fn load_collada(
    tx: &std::sync::mpsc::Sender<re_data_loader::LoadedData>,
    filepath: &std::path::Path,
    contents: &[u8],
) -> Result<(), re_data_loader::DataLoaderError> {
    use mesh_loader::collada;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    println!("----------------------------------------------");
    let mut h = DefaultHasher::new();
    contents.hash(&mut h);

    let scene = collada::from_slice(contents);

    if let Ok(scene) = scene {
        for mesh in scene.meshes {
            // println!("verticce : {:?} | \nnormals : {:?}", mesh.vertices, mesh.normals);
            let mut mesh3d = rerun::Mesh3D::new(mesh.vertices);

            if !mesh.normals.is_empty() && !mesh.normals[0].is_empty() {
                mesh3d = mesh3d.with_vertex_normals(mesh.normals);
            }

            let entity_path = EntityPath::from_file_path(filepath);
            let chunk = Chunk::builder(entity_path)
                .with_archetype(RowId::new(), TimePoint::default(), &mesh3d)
                .build()?;

            tx.send(chunk.into()).ok();
        }
    }

    Ok(())
}
