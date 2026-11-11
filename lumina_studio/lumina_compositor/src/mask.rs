use crate::keyframe::AnimatableProperty;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackMatte {
    None,
    AlphaMatte { inverted: bool },
    LumaMatte { inverted: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub struct BezierKnot {
    pub vertex: [f32; 2],
    pub in_tangent: [f32; 2],
    pub out_tangent: [f32; 2],
}

#[derive(Clone, Debug)]
pub struct VectorMask {
    pub name: String,
    pub inverted: bool,
    pub path: AnimatableProperty<Vec<BezierKnot>>,
    pub opacity: AnimatableProperty<f32>,
    pub feather: AnimatableProperty<f32>,
    pub expansion: AnimatableProperty<f32>,
}
