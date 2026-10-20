use super::{
    ORGANIZATION_TILE_SIZE, RAIL_HEADER_HEIGHT, RAIL_WIDTH, organization_marker_height,
};

#[test]
fn organization_rail_matches_reference_dimensions() {
    assert_eq!(RAIL_WIDTH, 64.);
    assert_eq!(RAIL_HEADER_HEIGHT, 34.);
    assert_eq!(ORGANIZATION_TILE_SIZE, 36.);
}

#[test]
fn active_and_hovered_organizations_use_reference_marker_heights() {
    assert_eq!(organization_marker_height(true, false), 28.);
    assert_eq!(organization_marker_height(false, true), 12.);
    assert_eq!(organization_marker_height(false, false), 0.);
}
