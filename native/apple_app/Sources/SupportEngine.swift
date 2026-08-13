import Foundation

struct NeedItem: Identifiable, Hashable {
    let id: String
    let label: String
    let support: String
}

private func hl(_ key: String) -> String {
    NSLocalizedString(key, comment: "HEARTLIGHT localized interface string")
}

let heartlightNeeds: [NeedItem] = [
    .init(id: "quiet", label: hl("🤫 Quiet"), support: hl("Offer a quieter place or reduce competing sound.")),
    .init(id: "movement", label: hl("🌀 Move"), support: hl("Offer safe movement such as walking, stretching, rocking, or a movement break.")),
    .init(id: "dim", label: hl("🕯 Less light"), support: hl("Reduce glare or offer a lower-light option where safe.")),
    .init(id: "space", label: hl("↔ More space"), support: hl("Offer more physical space without using isolation as punishment.")),
    .init(id: "help", label: hl("🧩 Help me"), support: hl("Break the task into one visible next step and offer a choice of how to begin.")),
    .init(id: "break", label: hl("🌿 Break"), support: hl("Offer a predictable break and a clear path back when ready.")),
    .init(id: "pressure", label: hl("🧸 Pressure"), support: hl("Offer only familiar, consented proprioceptive options; never impose touch.")),
    .init(id: "sound", label: hl("🎧 Sound choice"), support: hl("Offer a preferred sound level or approved headphones.")),
    .init(id: "predict", label: hl("🗓 What next?"), support: hl("Show a first-then card, visual schedule, transition warning, or countdown.")),
    .init(id: "communicate", label: hl("💬 Another way"), support: hl("Offer AAC, typing, pointing, drawing, gesture, or yes/no choices.")),
    .init(id: "unknown", label: hl("❔ I don't know"), support: hl("Reduce demands briefly and offer two simple options without forcing an answer.")),
    .init(id: "company", label: hl("💚 Stay with me"), support: hl("Stay nearby, reduce language, and offer calm predictable presence."))
]

func supportSuggestions(selected: Set<String>, sensory: Double, brightness: Double?) -> [String] {
    var result = heartlightNeeds.filter { selected.contains($0.id) }.map(\.support)
    if sensory >= 7 && !selected.contains("quiet") {
        result.append(hl("Sensory load is high: check whether reducing sound, visual clutter, or demand would help."))
    }
    if let brightness, brightness >= 75 && selected.contains("dim") {
        result.insert(hl("The room reading is bright and less-light was requested: consider reducing glare or changing position."), at: 0)
    }
    return Array(result.prefix(6))
}
