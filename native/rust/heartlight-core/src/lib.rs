#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    Quiet,
    Movement,
    DimLight,
    Space,
    Help,
    Break,
    Pressure,
    SoundChoice,
    Predictability,
    AlternateCommunication,
    Unknown,
    Company,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupportState {
    pub sensory_load: f64,
    pub visual_load: f64,
    pub movement_need: f64,
    pub focus_access: f64,
    pub transition_need: f64,
    pub communication_load: f64,
    pub social_space_need: f64,
    pub body_comfort: f64,
    pub predictability_need: f64,
    pub recovery_need: f64,
    pub engagement_access: f64,
    pub regulation_confidence: f64,
}

impl Default for SupportState {
    fn default() -> Self {
        Self {
            sensory_load: 5.0,
            visual_load: 5.0,
            movement_need: 5.0,
            focus_access: 5.0,
            transition_need: 5.0,
            communication_load: 5.0,
            social_space_need: 5.0,
            body_comfort: 5.0,
            predictability_need: 5.0,
            recovery_need: 5.0,
            engagement_access: 5.0,
            regulation_confidence: 5.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EnvironmentObservation {
    pub ambient_brightness_percent: Option<f64>,
    pub rgb: Option<(u8, u8, u8)>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BioObservation {
    /// Optional manually-entered or approved-sensor observation only.
    /// The reference engine does not infer diagnosis, emotion, danger,
    /// compliance, or behavior risk from this value.
    pub pulse_bpm: Option<u16>,
}

#[derive(Clone, Debug, Default)]
pub struct Input {
    pub state: SupportState,
    pub needs: Vec<Need>,
    pub environment: EnvironmentObservation,
    pub bio: BioObservation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Suggestion {
    pub id: &'static str,
    pub title: &'static str,
    pub rationale: &'static str,
    pub priority: u8,
    pub requires_consent: bool,
}

#[derive(Clone, Debug)]
pub struct ResultState {
    pub normalized_state: SupportState,
    pub suggestions: Vec<Suggestion>,
    pub notices: Vec<String>,
}

fn clamp10(v: f64) -> f64 {
    if !v.is_finite() { 5.0 } else { v.clamp(0.0, 10.0) }
}

pub fn normalize(s: SupportState) -> SupportState {
    SupportState {
        sensory_load: clamp10(s.sensory_load),
        visual_load: clamp10(s.visual_load),
        movement_need: clamp10(s.movement_need),
        focus_access: clamp10(s.focus_access),
        transition_need: clamp10(s.transition_need),
        communication_load: clamp10(s.communication_load),
        social_space_need: clamp10(s.social_space_need),
        body_comfort: clamp10(s.body_comfort),
        predictability_need: clamp10(s.predictability_need),
        recovery_need: clamp10(s.recovery_need),
        engagement_access: clamp10(s.engagement_access),
        regulation_confidence: clamp10(s.regulation_confidence),
    }
}

fn has(needs: &[Need], target: Need) -> bool { needs.contains(&target) }

fn push_unique(out: &mut Vec<Suggestion>, suggestion: Suggestion) {
    if !out.iter().any(|s| s.id == suggestion.id) { out.push(suggestion); }
}

pub fn evaluate(input: &Input) -> ResultState {
    let s = normalize(input.state);
    let mut suggestions = Vec::new();

    if has(&input.needs, Need::Quiet) || s.sensory_load >= 7.0 {
        push_unique(&mut suggestions, Suggestion { id: "quiet-option", title: "Offer a quieter option", rationale: "High sensory load or an explicit quiet request can justify reducing competing sound.", priority: 90, requires_consent: false });
    }
    if has(&input.needs, Need::Movement) || s.movement_need >= 7.0 {
        push_unique(&mut suggestions, Suggestion { id: "movement-option", title: "Offer safe movement", rationale: "Movement can support access and regulation. Offer a safe, non-punitive movement choice.", priority: 85, requires_consent: false });
    }
    let bright = input.environment.ambient_brightness_percent.map(|v| v >= 80.0).unwrap_or(false);
    if has(&input.needs, Need::DimLight) || s.visual_load >= 7.0 || bright {
        push_unique(&mut suggestions, Suggestion { id: "light-option", title: "Reduce glare or brightness", rationale: "The learner requested less light, visual load is high, or the room reading is very bright.", priority: 85, requires_consent: false });
    }
    if has(&input.needs, Need::Space) || s.social_space_need >= 7.0 {
        push_unique(&mut suggestions, Suggestion { id: "space-option", title: "Offer more space", rationale: "Provide physical space without using isolation as punishment.", priority: 80, requires_consent: false });
    }
    if has(&input.needs, Need::Help) || s.focus_access <= 3.0 || s.engagement_access <= 3.0 {
        push_unique(&mut suggestions, Suggestion { id: "next-step", title: "Show one visible next step", rationale: "Lowering task complexity can improve access when focus or engagement feels difficult.", priority: 80, requires_consent: false });
    }
    if has(&input.needs, Need::Break) || s.recovery_need >= 7.0 {
        push_unique(&mut suggestions, Suggestion { id: "break-option", title: "Offer a flexible break", rationale: "A predictable, non-punitive break may support recovery and return to learning.", priority: 90, requires_consent: false });
    }
    if has(&input.needs, Need::Pressure) {
        push_unique(&mut suggestions, Suggestion { id: "pressure-option", title: "Offer familiar proprioceptive choices", rationale: "Use only learner-preferred options such as wall pushes or carrying a light classroom item. Never impose touch.", priority: 70, requires_consent: true });
    }
    if has(&input.needs, Need::SoundChoice) {
        push_unique(&mut suggestions, Suggestion { id: "sound-choice", title: "Offer a sound-level choice", rationale: "The learner explicitly requested a different sound environment.", priority: 75, requires_consent: false });
    }
    if has(&input.needs, Need::Predictability) || s.predictability_need >= 7.0 || s.transition_need >= 7.0 {
        push_unique(&mut suggestions, Suggestion { id: "predictability", title: "Make the next transition visible", rationale: "A first-then card, visual schedule, or warning can reduce uncertainty.", priority: 85, requires_consent: false });
    }
    if has(&input.needs, Need::AlternateCommunication) || s.communication_load >= 7.0 {
        push_unique(&mut suggestions, Suggestion { id: "communication-choice", title: "Offer another way to communicate", rationale: "AAC, typing, pointing, drawing, gesture, and yes/no choices are valid communication paths.", priority: 95, requires_consent: false });
    }
    if has(&input.needs, Need::Unknown) || s.regulation_confidence <= 3.0 {
        push_unique(&mut suggestions, Suggestion { id: "unknown-valid", title: "Reduce demands and offer two simple choices", rationale: "Not knowing what would help is valid information; simplify the next decision.", priority: 95, requires_consent: false });
    }
    if has(&input.needs, Need::Company) {
        push_unique(&mut suggestions, Suggestion { id: "stay-near", title: "Stay nearby without crowding", rationale: "The learner requested supportive presence. Follow their preferred distance and communication style.", priority: 90, requires_consent: true });
    }
    if suggestions.is_empty() {
        push_unique(&mut suggestions, Suggestion { id: "ask-first", title: "Ask what would make learning easier", rationale: "No strong rule fired. The learner's preference is more important than forcing a recommendation.", priority: 50, requires_consent: false });
    }

    suggestions.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.id.cmp(b.id)));
    let mut notices = vec![
        "HEARTLIGHT suggestions are educational support options, not diagnoses or treatment instructions.".to_string(),
        "Human review and learner preference remain authoritative.".to_string(),
    ];
    if let Some(pulse) = input.bio.pulse_bpm {
        notices.push(format!("Pulse observation present ({pulse} bpm). Reference engine stores no medical or emotional interpretation."));
    }
    ResultState { normalized_state: s, suggestions, notices }
}

pub fn version() -> &'static str { "0.2.0" }
