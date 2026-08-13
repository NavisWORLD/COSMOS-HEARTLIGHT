use heartlight_core::{evaluate, Input, Need, SupportState};

#[repr(C)]
pub struct HeartlightState12 {
    pub values: [f64; 12],
}

fn state_from_array(v: [f64; 12]) -> SupportState {
    SupportState {
        sensory_load: v[0], visual_load: v[1], movement_need: v[2], focus_access: v[3],
        transition_need: v[4], communication_load: v[5], social_space_need: v[6], body_comfort: v[7],
        predictability_need: v[8], recovery_need: v[9], engagement_access: v[10], regulation_confidence: v[11],
    }
}

const NEEDS: [Need; 12] = [
    Need::Quiet, Need::Movement, Need::DimLight, Need::Space, Need::Help, Need::Break,
    Need::Pressure, Need::SoundChoice, Need::Predictability, Need::AlternateCommunication,
    Need::Unknown, Need::Company,
];

#[no_mangle]
pub extern "C" fn heartlight_rust_evaluate_mask(state: HeartlightState12, need_mask: u32) -> u32 {
    let needs = NEEDS.iter().enumerate().filter_map(|(i, n)| if need_mask & (1u32 << i) != 0 { Some(*n) } else { None }).collect();
    let result = evaluate(&Input { state: state_from_array(state.values), needs, ..Input::default() });
    let mut out = 0u32;
    for s in result.suggestions {
        let bit = match s.id {
            "quiet-option" => 0, "movement-option" => 1, "light-option" => 2, "space-option" => 3,
            "next-step" => 4, "break-option" => 5, "pressure-option" => 6, "sound-choice" => 7,
            "predictability" => 8, "communication-choice" => 9, "unknown-valid" => 10, "stay-near" => 11,
            _ => continue,
        };
        out |= 1u32 << bit;
    }
    out
}
