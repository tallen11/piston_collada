use std::path::Path;
use super::document::ColladaDocument;

#[test]
fn test_get_cameras() {
    let doc = ColladaDocument::from_path(Path::new("./test_assets/blender_default.dae")).unwrap();
    let cameras = doc.get_cameras();
    
    assert!(!cameras.is_none());
    assert_eq!(cameras.unwrap().len(), 1);
}
