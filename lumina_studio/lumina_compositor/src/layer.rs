use lumina_core::RationalTime;
use crate::keyframe::AnimatableProperty;
use crate::mask::{BlendMode, TrackMatte, VectorMask};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Transform2D {
    pub anchor_point: AnimatableProperty<[f32; 2]>,
    pub position: AnimatableProperty<[f32; 2]>,
    pub scale: AnimatableProperty<[f32; 2]>,
    pub rotation: AnimatableProperty<f32>,
    pub opacity: AnimatableProperty<f32>,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            anchor_point: AnimatableProperty { base_value: [0.0, 0.0], keyframes: vec![] },
            position: AnimatableProperty { base_value: [0.0, 0.0], keyframes: vec![] },
            scale: AnimatableProperty { base_value: [100.0, 100.0], keyframes: vec![] },
            rotation: AnimatableProperty { base_value: 0.0, keyframes: vec![] },
            opacity: AnimatableProperty { base_value: 100.0, keyframes: vec![] },
        }
    }
}

pub struct Layer {
    pub id: Uuid,
    pub name: String,

    // Parenting hierarchy
    pub parent_id: Option<Uuid>,

    // Trim bounding
    pub in_point: RationalTime,
    pub out_point: RationalTime,

    // Core transforms
    pub transform: Transform2D,

    // Masking and Blending
    pub blend_mode: BlendMode,
    pub track_matte: TrackMatte,
    pub masks: Vec<VectorMask>,

    pub is_adjustment_layer: bool,
    pub is_3d: bool,
}

/// Helper function to multiply two 3x3 matrices in row-major order.
/// Result = a * b
fn multiply_matrix_3x3(a: &[f32; 9], b: &[f32; 9]) -> [f32; 9] {
    let mut out = [0.0; 9];
    for row in 0..3 {
        for col in 0..3 {
            out[row * 3 + col] =
                a[row * 3 + 0] * b[0 * 3 + col] +
                a[row * 3 + 1] * b[1 * 3 + col] +
                a[row * 3 + 2] * b[2 * 3 + col];
        }
    }
    out
}

impl Layer {
    pub fn local_matrix_at(&self, time: RationalTime) -> [f32; 9] {
        let pos = self.transform.position.evaluate_at(time);
        let scale = self.transform.scale.evaluate_at(time);
        let rot = self.transform.rotation.evaluate_at(time);
        let anchor = self.transform.anchor_point.evaluate_at(time);

        let r = rot.to_radians();
        let c = r.cos();
        let s = r.sin();

        let sx = scale[0] / 100.0;
        let sy = scale[1] / 100.0;

        // Proper Affine transformation:
        // Tx = pos_x - (c * sx * anchor_x - s * sy * anchor_y)
        // Ty = pos_y - (s * sx * anchor_x + c * sy * anchor_y)

        let tx = pos[0] - (c * sx * anchor[0] - s * sy * anchor[1]);
        let ty = pos[1] - (s * sx * anchor[0] + c * sy * anchor[1]);

        [
            c * sx, -s * sy, tx,
            s * sx,  c * sy, ty,
            0.0,     0.0,    1.0
        ]
    }
}

pub struct Composition {
    pub layers: Vec<Layer>
}

impl Composition {
    pub fn get_global_matrix_at(&self, layer_id: Uuid, time: RationalTime) -> Option<[f32; 9]> {
        let layer = self.layers.iter().find(|l| l.id == layer_id)?;
        let local_mat = layer.local_matrix_at(time);

        if let Some(parent_id) = layer.parent_id {
            if let Some(parent_global) = self.get_global_matrix_at(parent_id, time) {
                // Return matrix multiplication (ParentGlobal * LocalMat)
                return Some(multiply_matrix_3x3(&parent_global, &local_mat));
            }
        }

        Some(local_mat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyframe::{Keyframe, KeyframeInterpolation};

    #[test]
    fn test_layer_local_matrix_evaluation() {
        let mut transform = Transform2D::default();
        transform.scale.keyframes.push(Keyframe {
            time: RationalTime::new(0, 1),
            value: [200.0, 200.0],
            interpolation: KeyframeInterpolation::Linear,
        });

        transform.position.keyframes.push(Keyframe {
            time: RationalTime::new(0, 1),
            value: [150.0, 50.0],
            interpolation: KeyframeInterpolation::Linear,
        });

        let layer = Layer {
            id: Uuid::new_v4(),
            name: "Child Layer".into(),
            parent_id: None,
            in_point: RationalTime::new(0, 1),
            out_point: RationalTime::new(10, 1),
            transform,
            blend_mode: BlendMode::Normal,
            track_matte: TrackMatte::None,
            masks: vec![],
            is_adjustment_layer: false,
            is_3d: false,
        };

        let matrix = layer.local_matrix_at(RationalTime::new(5, 1));
        assert_eq!(matrix[0], 2.0); // sx
        assert_eq!(matrix[4], 2.0); // sy
        assert_eq!(matrix[2], 150.0); // tx (translation x)
        assert_eq!(matrix[5], 50.0); // ty (translation y)
    }

    #[test]
    fn test_composition_parent_resolution() {
        let parent_id = Uuid::new_v4();
        let child_id = Uuid::new_v4();

        let mut parent_transform = Transform2D::default();
        parent_transform.scale.base_value = [200.0, 200.0]; // Parent scales by 2x

        let parent = Layer {
            id: parent_id,
            name: "Parent Null".into(),
            parent_id: None,
            in_point: RationalTime::new(0, 1),
            out_point: RationalTime::new(10, 1),
            transform: parent_transform,
            blend_mode: BlendMode::Normal,
            track_matte: TrackMatte::None,
            masks: vec![],
            is_adjustment_layer: false,
            is_3d: false,
        };

        let mut child_transform = Transform2D::default();
        child_transform.scale.base_value = [200.0, 200.0]; // Child scales by 2x

        let child = Layer {
            id: child_id,
            name: "Child Layer".into(),
            parent_id: Some(parent_id),
            in_point: RationalTime::new(0, 1),
            out_point: RationalTime::new(10, 1),
            transform: child_transform,
            blend_mode: BlendMode::Normal,
            track_matte: TrackMatte::None,
            masks: vec![],
            is_adjustment_layer: false,
            is_3d: false,
        };

        let comp = Composition { layers: vec![parent, child] };

        // The global scale of the child should be 4x (2x parent * 2x child)
        let global_matrix = comp.get_global_matrix_at(child_id, RationalTime::new(0, 1)).unwrap();
        assert_eq!(global_matrix[0], 4.0); // global scale x
        assert_eq!(global_matrix[4], 4.0); // global scale y
    }
}
