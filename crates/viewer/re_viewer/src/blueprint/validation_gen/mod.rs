// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/blueprint_validation.rs
use super::validation::validate_component;
use re_entity_db::EntityDb;
pub use re_types::blueprint::components::ActiveTab;
pub use re_types::blueprint::components::BackgroundKind;
pub use re_types::blueprint::components::ColumnShare;
pub use re_types::blueprint::components::Corner2D;
pub use re_types::blueprint::components::IncludedContent;
pub use re_types::blueprint::components::Interactive;
pub use re_types::blueprint::components::LockRangeDuringZoom;
pub use re_types::blueprint::components::PanelState;
pub use re_types::blueprint::components::QueryExpression;
pub use re_types::blueprint::components::RowShare;
pub use re_types::blueprint::components::SpaceViewClass;
pub use re_types::blueprint::components::SpaceViewOrigin;
pub use re_types::blueprint::components::TensorDimensionIndexSlider;
pub use re_types::blueprint::components::ViewFit;
pub use re_types::blueprint::components::ViewerRecommendationHash;
pub use re_types::blueprint::components::Visible;
pub use re_types::blueprint::components::VisibleTimeRange;
pub use re_types::blueprint::components::VisualBounds2D;
pub use re_types_blueprint::blueprint::components::AutoLayout;
pub use re_types_blueprint::blueprint::components::AutoSpaceViews;
pub use re_types_blueprint::blueprint::components::ContainerKind;
pub use re_types_blueprint::blueprint::components::GridColumns;
pub use re_types_blueprint::blueprint::components::IncludedSpaceView;
pub use re_types_blueprint::blueprint::components::RootContainer;
pub use re_types_blueprint::blueprint::components::SpaceViewMaximized;
pub use re_types_blueprint::blueprint::components::VisualizerOverrides;

/// Because blueprints are both read and written the schema must match what
/// we expect to find or else we will run into all kinds of problems.

pub fn is_valid_blueprint(blueprint: &EntityDb) -> bool {
    validate_component::<ActiveTab>(blueprint)
        && validate_component::<AutoLayout>(blueprint)
        && validate_component::<AutoSpaceViews>(blueprint)
        && validate_component::<BackgroundKind>(blueprint)
        && validate_component::<ColumnShare>(blueprint)
        && validate_component::<ContainerKind>(blueprint)
        && validate_component::<Corner2D>(blueprint)
        && validate_component::<GridColumns>(blueprint)
        && validate_component::<IncludedContent>(blueprint)
        && validate_component::<IncludedSpaceView>(blueprint)
        && validate_component::<Interactive>(blueprint)
        && validate_component::<LockRangeDuringZoom>(blueprint)
        && validate_component::<PanelState>(blueprint)
        && validate_component::<QueryExpression>(blueprint)
        && validate_component::<RootContainer>(blueprint)
        && validate_component::<RowShare>(blueprint)
        && validate_component::<SpaceViewClass>(blueprint)
        && validate_component::<SpaceViewMaximized>(blueprint)
        && validate_component::<SpaceViewOrigin>(blueprint)
        && validate_component::<TensorDimensionIndexSlider>(blueprint)
        && validate_component::<ViewFit>(blueprint)
        && validate_component::<ViewerRecommendationHash>(blueprint)
        && validate_component::<Visible>(blueprint)
        && validate_component::<VisibleTimeRange>(blueprint)
        && validate_component::<VisualBounds2D>(blueprint)
        && validate_component::<VisualizerOverrides>(blueprint)
}