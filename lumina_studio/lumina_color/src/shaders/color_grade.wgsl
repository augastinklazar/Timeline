struct GradingParameters {
    lift: vec4<f32>,
    gamma: vec4<f32>,
    gain: vec4<f32>,
    offset: vec4<f32>,
    contrast_pivot_sat: vec4<f32>, // x: Contrast, y: Pivot, z: Saturation, w: Unused
};

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var output_texture: texture_storage_2d<rgba32float, write>;
@group(0) @binding(2) var<uniform> params: GradingParameters;

@compute @workgroup_size(16, 16)
fn color_grade_cs(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(input_texture);
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }

    let coord = vec2<i32>(global_id.xy);
    var color = textureLoad(input_texture, coord, 0);

    // Step 1: Apply Master Offset
    var rgb = color.rgb + params.offset.rgb;

    // Step 2: ASC-CDL / Lift, Gain Model
    let one = vec3<f32>(1.0);
    rgb = rgb + params.lift.rgb * (one - clamp(rgb, vec3<f32>(0.0), one));
    rgb = rgb * params.gain.rgb;

    // Step 3: Gamma Curve (Safe Exponentiation)
    let safe_gamma = max(params.gamma.rgb, vec3<f32>(0.001));
    let sign_rgb = sign(rgb);
    rgb = sign_rgb * pow(max(abs(rgb), vec3<f32>(0.0)), one / safe_gamma);

    // Step 4: S-Curve / Linear Contrast Around Pivot
    let contrast = params.contrast_pivot_sat.x;
    let pivot = params.contrast_pivot_sat.y;
    rgb = pivot + (rgb - pivot) * contrast;

    // Step 5: Rec.709 Luma Saturation
    let luma = dot(rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let sat = params.contrast_pivot_sat.z;
    rgb = mix(vec3<f32>(luma), rgb, sat);

    textureStore(output_texture, coord, vec4<f32>(rgb, color.a));
}
