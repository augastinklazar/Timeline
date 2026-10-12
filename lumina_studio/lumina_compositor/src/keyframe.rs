use lumina_core::RationalTime;

#[derive(Clone, Copy, Debug)]
pub enum KeyframeInterpolation {
    Linear,
    Bezier {
        out_tan_x: f32,
        out_tan_y: f32,
        in_tan_x: f32,
        in_tan_y: f32,
    },
}

#[derive(Clone, Debug)]
pub struct Keyframe<T> {
    pub time: RationalTime,
    pub value: T,
    pub interpolation: KeyframeInterpolation,
}

#[derive(Clone, Debug)]
pub struct AnimatableProperty<T> {
    pub base_value: T,
    pub keyframes: Vec<Keyframe<T>>,
}

impl AnimatableProperty<f32> {
    pub fn evaluate_at(&self, time: RationalTime) -> f32 {
        if self.keyframes.is_empty() {
            return self.base_value;
        }

        let first = self.keyframes.first().unwrap();
        if time <= first.time { return first.value; }

        let last = self.keyframes.last().unwrap();
        if time >= last.time { return last.value; }

        for i in 0..self.keyframes.len() - 1 {
            let k0 = &self.keyframes[i];
            let k1 = &self.keyframes[i + 1];

            if time >= k0.time && time < k1.time {
                let dt = (k1.time.to_seconds() - k0.time.to_seconds()) as f32;
                if dt <= 0.0 {
                    return k0.value;
                }

                let t = ((time.to_seconds() - k0.time.to_seconds()) as f32 / dt).clamp(0.0, 1.0);

                return match k0.interpolation {
                    KeyframeInterpolation::Linear => k0.value + t * (k1.value - k0.value),
                    KeyframeInterpolation::Bezier { out_tan_x, out_tan_y, in_tan_x, in_tan_y } => {
                        // The actual proper implementation would use Newton Raphson to resolve `u` where
                        // BezierX(u) = t, using out_tan_x and in_tan_x, then evaluate BezierY(u).
                        // For this structural POC, we approximate easing by mapping t directly via tangents
                        let p0 = k0.value;
                        let p3 = k1.value;
                        let p1 = p0 + (out_tan_y * out_tan_x); // Approximating influence
                        let p2 = p3 - (in_tan_y * in_tan_x);   // Approximating influence
                        let omt = 1.0 - t;
                        omt.powi(3) * p0 + 3.0 * omt.powi(2) * t * p1 + 3.0 * omt * t.powi(2) * p2 + t.powi(3) * p3
                    }
                };
            }
        }
        self.base_value
    }
}

impl AnimatableProperty<[f32; 2]> {
    pub fn evaluate_at(&self, time: RationalTime) -> [f32; 2] {
        if self.keyframes.is_empty() {
            return self.base_value;
        }

        let first = self.keyframes.first().unwrap();
        if time <= first.time { return first.value; }

        let last = self.keyframes.last().unwrap();
        if time >= last.time { return last.value; }

        for i in 0..self.keyframes.len() - 1 {
            let k0 = &self.keyframes[i];
            let k1 = &self.keyframes[i + 1];

            if time >= k0.time && time < k1.time {
                let dt = (k1.time.to_seconds() - k0.time.to_seconds()) as f32;
                if dt <= 0.0 {
                    return k0.value;
                }

                let t = ((time.to_seconds() - k0.time.to_seconds()) as f32 / dt).clamp(0.0, 1.0);

                return match k0.interpolation {
                    KeyframeInterpolation::Linear => [
                        k0.value[0] + t * (k1.value[0] - k0.value[0]),
                        k0.value[1] + t * (k1.value[1] - k0.value[1]),
                    ],
                    KeyframeInterpolation::Bezier { out_tan_x, out_tan_y, in_tan_x, in_tan_y } => {
                        let eval = |p0, p3| -> f32 {
                            let p1 = p0 + (out_tan_y * out_tan_x);
                            let p2 = p3 - (in_tan_y * in_tan_x);
                            let omt = 1.0 - t;
                            omt.powi(3) * p0 + 3.0 * omt.powi(2) * t * p1 + 3.0 * omt * t.powi(2) * p2 + t.powi(3) * p3
                        };
                        [eval(k0.value[0], k1.value[0]), eval(k0.value[1], k1.value[1])]
                    }
                };
            }
        }
        self.base_value
    }
}


impl AnimatableProperty<Vec<crate::mask::BezierKnot>> {
    pub fn evaluate_at(&self, time: RationalTime) -> Vec<crate::mask::BezierKnot> {
        if self.keyframes.is_empty() {
            return self.base_value.clone();
        }

        let first = self.keyframes.first().unwrap();
        if time <= first.time { return first.value.clone(); }

        let last = self.keyframes.last().unwrap();
        if time >= last.time { return last.value.clone(); }

        // Complex shape morphing math omitted for POC, returns base for now
        self.base_value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolation() {
        let prop = AnimatableProperty {
            base_value: 0.0,
            keyframes: vec![
                Keyframe {
                    time: RationalTime::new(0, 1), // 0 seconds
                    value: 10.0,
                    interpolation: KeyframeInterpolation::Linear,
                },
                Keyframe {
                    time: RationalTime::new(2, 1), // 2 seconds
                    value: 20.0,
                    interpolation: KeyframeInterpolation::Linear,
                }
            ],
        };

        // Mid-point should be exactly 15.0
        let val_at_1 = prop.evaluate_at(RationalTime::new(1, 1));
        assert_eq!(val_at_1, 15.0);
    }
}
