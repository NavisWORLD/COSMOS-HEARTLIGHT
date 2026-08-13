import SwiftUI

struct SensorView: View {
    @StateObject private var sampler = CameraSampler()
    @State private var lessLightRequested = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 18) {
                    Text("Ambient color / brightness").font(.title.bold())
                    Text("Point the camera at the room or light source, not at a learner. Frames are sampled in memory and are not saved by HEARTLIGHT.")
                        .foregroundStyle(.secondary)
                    RoundedRectangle(cornerRadius: 18)
                        .fill(Color(red: sampler.red, green: sampler.green, blue: sampler.blue))
                        .frame(height: 170)
                        .overlay(
                            HStack(spacing: 3) {
                                Text(verbatim: "\(Int(sampler.brightness))%")
                                Text("brightness")
                            }
                            .padding(8)
                            .background(.ultraThinMaterial)
                            .clipShape(Capsule())
                        )
                    Text(verbatim: sampler.status).font(.callout)
                    Toggle("Learner requested less light", isOn: $lessLightRequested)
                    if lessLightRequested && sampler.brightness >= 75 {
                        Text("The room reading is bright and less-light was requested. Consider reducing glare, moving position, or offering a lower-light option.")
                            .padding().background(.green.opacity(0.12)).clipShape(RoundedRectangle(cornerRadius: 14))
                    }
                    HStack {
                        Button {
                            sampler.start()
                        } label: {
                            Text(sampler.running ? String(localized: "Sampling…") : String(localized: "Start sensor"))
                        }
                        .buttonStyle(.borderedProminent)
                        .disabled(sampler.running)
                        Button("Stop") { sampler.stop() }.buttonStyle(.bordered)
                    }
                    Text("The sensor never decides what color a learner ‘should’ prefer. Learner preference wins.")
                        .font(.footnote).foregroundStyle(.secondary)
                }.padding()
            }
            .navigationTitle("Room Sensor")
            .onDisappear { sampler.stop() }
        }
    }
}
