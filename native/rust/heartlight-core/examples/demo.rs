use heartlight_core::{evaluate, EnvironmentObservation, Input, Need, SupportState};

fn main() {
    let mut state = SupportState::default();
    state.sensory_load = 8.0;
    state.visual_load = 8.0;
    state.regulation_confidence = 2.0;

    let input = Input {
        state,
        needs: vec![Need::Quiet, Need::DimLight, Need::Unknown],
        environment: EnvironmentObservation { ambient_brightness_percent: Some(91.0), rgb: Some((240, 240, 255)) },
        ..Input::default()
    };

    let result = evaluate(&input);
    for suggestion in result.suggestions {
        println!("[{}] {} — {}", suggestion.priority, suggestion.title, suggestion.rationale);
    }
}
