use std::path::Path;
use super::document::ColladaDocument;

#[test]
fn test_get_cameras() {
    let doc = ColladaDocument::from_path(Path::new("./test_assets/blender_default.dae")).unwrap();
    let cameras = doc.get_cameras();
    
    assert!(!cameras.is_none());
    assert_eq!(cameras.unwrap().len(), 1);
}

#[test]
fn test_get_lights() {
    let doc = ColladaDocument::from_path(Path::new("./test_assets/blender_default.dae")).unwrap();
    let lights = doc.get_lights();

    assert!(!lights.is_none());
    assert_eq!(lights.unwrap().len(), 1);
}
