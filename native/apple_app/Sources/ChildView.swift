import SwiftUI

struct ChildView: View {
    @State private var selected: Set<String> = []
    @State private var sensory = 5.0
    @State private var movement = 5.0
    @State private var focus = 5.0
    @State private var confidence = 5.0
    @State private var seconds = 120
    @State private var timerRunning = false

    private let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    private let columns = [GridItem(.adaptive(minimum: 150), spacing: 10)]

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("What do you need?").font(.title.bold())
                        Text("Choose as many as you want. ‘I don't know’ is a complete answer.").foregroundStyle(.secondary)
                        LazyVGrid(columns: columns, spacing: 10) {
                            ForEach(heartlightNeeds) { need in
                                Button {
                                    if selected.contains(need.id) { selected.remove(need.id) }
                                    else { selected.insert(need.id) }
                                } label: {
                                    Text(verbatim: need.label).frame(maxWidth: .infinity, minHeight: 48)
                                }
                                .buttonStyle(.borderedProminent)
                                .tint(selected.contains(need.id) ? .green : .gray.opacity(0.55))
                            }
                        }
                    }

                    GroupBox("My body right now") {
                        VStack(alignment: .leading) {
                            SliderRow(title: "Sensory load", value: $sensory)
                            SliderRow(title: "Movement need", value: $movement)
                            SliderRow(title: "Focus feels possible", value: $focus)
                            SliderRow(title: "I know what would help", value: $confidence)
                        }.padding(.vertical, 6)
                    }

                    GroupBox("Gentle support options") {
                        let suggestions = supportSuggestions(selected: selected, sensory: sensory, brightness: nil)
                        VStack(alignment: .leading, spacing: 10) {
                            if suggestions.isEmpty { Text("Choose a need above to see options.").foregroundStyle(.secondary) }
                            ForEach(suggestions, id: \.self) { Text(verbatim: "• \($0)") }
                        }.frame(maxWidth: .infinity, alignment: .leading).padding(.vertical, 6)
                    }

                    GroupBox("Visual timer") {
                        VStack(spacing: 12) {
                            Text(String(format: "%02d:%02d", seconds / 60, seconds % 60))
                                .font(.system(size: 48, weight: .bold, design: .rounded))
                                .monospacedDigit()
                            HStack {
                                Button("1 min") { seconds = 60; timerRunning = false }
                                Button("2 min") { seconds = 120; timerRunning = false }
                                Button("5 min") { seconds = 300; timerRunning = false }
                                Button {
                                    timerRunning.toggle()
                                } label: {
                                    Text(timerRunning ? String(localized: "Pause") : String(localized: "Start"))
                                }
                                .buttonStyle(.borderedProminent)
                            }
                            Text("A timer is an option, not a punishment.").font(.footnote).foregroundStyle(.secondary)
                        }.frame(maxWidth: .infinity)
                    }
                }
                .padding()
            }
            .navigationTitle("COSMOS HEARTLIGHT")
            .onReceive(timer) { _ in if timerRunning && seconds > 0 { seconds -= 1 }; if seconds == 0 { timerRunning = false } }
        }
    }
}

private struct SliderRow: View {
    let title: LocalizedStringKey
    @Binding var value: Double
    var body: some View {
        VStack(alignment: .leading) {
            HStack(spacing: 0) {
                Text(title)
                Text(verbatim: ": \(Int(value))/10")
            }
            Slider(value: $value, in: 0...10, step: 1)
        }
    }
}
