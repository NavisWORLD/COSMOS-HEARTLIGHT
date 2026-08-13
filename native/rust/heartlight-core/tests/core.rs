use heartlight_core::{evaluate, normalize, BioObservation, Input, Need, SupportState};

#[test]
fn clamps_state() {
    let mut state = SupportState::default();
    state.sensory_load = 99.0;
    state.focus_access = -4.0;
    let clean = normalize(state);
    assert_eq!(clean.sensory_load, 10.0);
    assert_eq!(clean.focus_access, 0.0);
}

#[test]
fn returns_support_without_bio_inference() {
    let mut state = SupportState::default();
    state.sensory_load = 8.0;
    state.regulation_confidence = 2.0;
    let input = Input {
        state,
        needs: vec![Need::Quiet, Need::Unknown],
        bio: BioObservation { pulse_bpm: Some(100) },
        ..Input::default()
    };
    let result = evaluate(&input);
    assert!(result.suggestions.iter().any(|s| s.id == "quiet-option"));
    assert!(result.suggestions.iter().any(|s| s.id == "unknown-valid"));
    assert!(result.notices.iter().any(|n| n.contains("no medical or emotional interpretation")));
}
