import SwiftUI

@main
struct HeartlightApp: App {
    var body: some Scene {
        WindowGroup {
            RootView()
                .tint(.green)
        }
    }
}

struct RootView: View {
    var body: some View {
        TabView {
            ChildView()
                .tabItem { Label("Child", systemImage: "heart.fill") }
            TeacherView()
                .tabItem { Label("Teacher", systemImage: "person.crop.rectangle") }
            SensorView()
                .tabItem { Label("Sensor", systemImage: "camera.metering.center.weighted") }
            AboutView()
                .tabItem { Label("About", systemImage: "shield.lefthalf.filled") }
        }
        #if os(macOS)
        .frame(minWidth: 760, minHeight: 620)
        #endif
    }
}
