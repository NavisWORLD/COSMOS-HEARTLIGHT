import Foundation

struct NeedItem: Identifiable, Hashable {
    let id: String
    let label: String
    let support: String
}

let heartlightNeeds: [NeedItem] = [
    .init(id: "quiet", label: "🤫 Quiet", support: "Offer a quieter place or reduce competing sound."),
    .init(id: "movement", label: "🌀 Move", support: "Offer safe movement such as walking, stretching, rocking, or a movement break."),
    .init(id: "dim", label: "🕯 Less light", support: "Reduce glare or offer a lower-light option where safe."),
    .init(id: "space", label: "↔ More space", support: "Offer more physical space without using isolation as punishment."),
    .init(id: "help", label: "🧩 Help me", support: "Break the task into one visible next step and offer a choice of how to begin."),
    .init(id: "break", label: "🌿 Break", support: "Offer a predictable break and a clear path back when ready."),
    .init(id: "pressure", label: "🧸 Pressure", support: "Offer only familiar, consented proprioceptive options; never impose touch."),
    .init(id: "sound", label: "🎧 Sound choice", support: "Offer a preferred sound level or approved headphones."),
    .init(id: "predict", label: "🗓 What next?", support: "Show a first-then card, visual schedule, transition warning, or countdown."),
    .init(id: "communicate", label: "💬 Another way", support: "Offer AAC, typing, pointing, drawing, gesture, or yes/no choices."),
    .init(id: "unknown", label: "❔ I don't know", support: "Reduce demands briefly and offer two simple options without forcing an answer."),
    .init(id: "company", label: "💚 Stay with me", support: "Stay nearby, reduce language, and offer calm predictable presence.")
]

func supportSuggestions(selected: Set<String>, sensory: Double, brightness: Double?) -> [String] {
    var result = heartlightNeeds.filter { selected.contains($0.id) }.map(\.support)
    if sensory >= 7 && !selected.contains("quiet") {
        result.append("Sensory load is high: check whether reducing sound, visual clutter, or demand would help.")
    }
    if let brightness, brightness >= 75 && selected.contains("dim") {
        result.insert("The room reading is bright and less-light was requested: consider reducing glare or changing position.", at: 0)
    }
    return Array(result.prefix(6))
}
