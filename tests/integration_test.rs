#![cfg_attr(test, allow(missing_docs))]

use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;
use aetherus_remesh::Inventory;
use colored::Colorize;

use anyhow::Result;

use aetherus_remesh::utils::parse_obj_file;
use aetherus_remesh::mesh::remesh;
use aetherus_remesh::Save;

#[test]
fn test_water_tank() -> Result<()> {
    env_logger::init();

    let obj_filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/objs/WaterTankEmpty.obj");

    let Inventory{meshes, verts, norms, faces} = parse_obj_file(Path::new(&obj_filepath))?;
    //let (meshes, ..) = parse_obj(obj);

    let now = Instant::now();

    assert_eq!(verts.borrow().len(), 20);
    assert_eq!(norms.borrow().len(), 17);
    assert_eq!(faces.borrow().len(), 40);
    for mesh in &meshes {
        println!(" > Mesh: {}", mesh.name.green());
        //println!("Mesh: {:?}", mesh.polygons);
    }

    let resolved_meshes = remesh(meshes)?;

    assert_eq!(verts.borrow().len(), 29);

    println!("Remeshed in {} seconds", now.elapsed().as_secs_f64());

    println!("Re-export the mesh to remeshed.obj");
    let tmp_obj_path = tempfile::NamedTempFile::new()?.into_temp_path().keep()?;
    resolved_meshes.save(&tmp_obj_path)?;

    // Check the written re-export is the same as expected, by just comparing files content
    let expected_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/objs/WaterTankEmpty_remeshed.obj");
    let re_exported_content = std::fs::read_to_string(&tmp_obj_path)
        .expect("Failed to read re-exported mesh file");
    let expected_content = std::fs::read_to_string(&expected_path)
        .expect("Failed to read expected mesh file");
    assert_eq!(re_exported_content, expected_content, "Re-exported mesh file content does not match expected content");


    Ok(())
}
