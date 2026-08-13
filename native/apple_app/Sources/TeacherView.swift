import SwiftUI

struct TeacherObservation: Codable, Identifiable {
    var id = UUID()
    var created = Date()
    var studentCode: String
    var context: String
    var observable: String
    var support: String
    var result: String
}

struct TeacherView: View {
    @State private var studentCode = ""
    @State private var context = ""
    @State private var observable = ""
    @State private var support = ""
    @State private var result = ""
    @State private var observations: [TeacherObservation] = []

    var body: some View {
        NavigationStack {
            Form {
                Section("Neutral observation") {
                    TextField("Student code — avoid full name", text: $studentCode)
                    TextField("What was happening immediately before?", text: $context, axis: .vertical)
                    TextField("What could a camera have seen or microphone heard?", text: $observable, axis: .vertical)
                    TextField("What support was offered?", text: $support, axis: .vertical)
                    TextField("What changed or what did the learner communicate next?", text: $result, axis: .vertical)
                    Button("Save locally") { save() }.buttonStyle(.borderedProminent)
                }
                Section("Local records") {
                    Text("\(observations.count) observation(s) saved on this device.")
                    Button("Erase all local observations", role: .destructive) { observations = []; persist() }
                }
                Section("Human decision boundary") {
                    Text("Never automate diagnosis, discipline, restraint, seclusion, medication advice, eligibility, IEP decisions, risk labels, or denial of access.")
                }
            }
            .navigationTitle("Teacher / Aide")
            .onAppear(perform: load)
        }
    }

    private func save() {
        let item = TeacherObservation(studentCode: studentCode, context: context, observable: observable, support: support, result: result)
        observations.append(item)
        persist()
        studentCode = ""; context = ""; observable = ""; support = ""; result = ""
    }

    private func persist() {
        if let data = try? JSONEncoder().encode(observations) { UserDefaults.standard.set(data, forKey: "heartlight.teacher.observations") }
    }

    private func load() {
        guard let data = UserDefaults.standard.data(forKey: "heartlight.teacher.observations"),
              let decoded = try? JSONDecoder().decode([TeacherObservation].self, from: data) else { return }
        observations = decoded
    }
}
