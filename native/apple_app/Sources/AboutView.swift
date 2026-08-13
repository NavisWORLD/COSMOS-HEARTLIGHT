import SwiftUI

struct AboutView: View {
    var body: some View {
        NavigationStack {
            List {
                Section("Privacy by architecture") {
                    Label("No advertising SDK", systemImage: "checkmark.shield")
                    Label("No cloud account required", systemImage: "checkmark.shield")
                    Label("No facial emotion recognition", systemImage: "checkmark.shield")
                    Label("No hidden child score", systemImage: "checkmark.shield")
                    Label("Camera frames are not saved", systemImage: "checkmark.shield")
                }
                Section("Purpose") {
                    Text("HEARTLIGHT is an educational and accessibility support tool. It is not a medical device, diagnostic test, psychotherapy replacement, crisis service, behavior-risk predictor, or automated discipline/IEP system.")
                }
                Section("The HEARTLIGHT rule") {
                    Text("Ask: What is the learner communicating, what is making access harder, and what safe support can we try together?")
                        .font(.headline)
                }
            }
            .navigationTitle("About HEARTLIGHT")
        }
    }
}
