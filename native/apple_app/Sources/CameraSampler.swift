import Foundation
import AVFoundation
import CoreVideo

final class CameraSampler: NSObject, ObservableObject, AVCaptureVideoDataOutputSampleBufferDelegate {
    @Published var red: Double = 0
    @Published var green: Double = 0
    @Published var blue: Double = 0
    @Published var brightness: Double = 0
    @Published var running = false
    @Published var status = NSLocalizedString("Sensor stopped", comment: "Camera sensor status")

    private let session = AVCaptureSession()
    private let queue = DispatchQueue(label: "heartlight.camera.sample", qos: .userInitiated)

    func start() {
        switch AVCaptureDevice.authorizationStatus(for: .video) {
        case .authorized: configureAndRun()
        case .notDetermined:
            AVCaptureDevice.requestAccess(for: .video) { [weak self] granted in
                if granted { self?.configureAndRun() }
                else { DispatchQueue.main.async { self?.status = NSLocalizedString("Camera permission was not granted.", comment: "Camera permission status") } }
            }
        default:
            status = NSLocalizedString("Camera permission is disabled. The rest of HEARTLIGHT works without it.", comment: "Camera permission status")
        }
    }

    private func configureAndRun() {
        queue.async { [weak self] in
            guard let self else { return }
            if self.session.inputs.isEmpty {
                self.session.beginConfiguration()
                self.session.sessionPreset = .low
                guard let device = AVCaptureDevice.default(for: .video),
                      let input = try? AVCaptureDeviceInput(device: device),
                      self.session.canAddInput(input) else {
                    self.session.commitConfiguration()
                    DispatchQueue.main.async { self.status = NSLocalizedString("No camera is available.", comment: "Camera status") }
                    return
                }
                self.session.addInput(input)
                let output = AVCaptureVideoDataOutput()
                output.alwaysDiscardsLateVideoFrames = true
                output.videoSettings = [kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_32BGRA]
                output.setSampleBufferDelegate(self, queue: self.queue)
                if self.session.canAddOutput(output) { self.session.addOutput(output) }
                self.session.commitConfiguration()
            }
            self.session.startRunning()
            DispatchQueue.main.async {
                self.running = true
                self.status = NSLocalizedString("Sampling ambient color locally", comment: "Camera sensor status")
            }
        }
    }

    func stop() {
        queue.async { [weak self] in
            self?.session.stopRunning()
            DispatchQueue.main.async {
                self?.running = false
                self?.status = NSLocalizedString("Sensor stopped", comment: "Camera sensor status")
            }
        }
    }

    func captureOutput(_ output: AVCaptureOutput, didOutput sampleBuffer: CMSampleBuffer, from connection: AVCaptureConnection) {
        guard let buffer = CMSampleBufferGetImageBuffer(sampleBuffer) else { return }
        CVPixelBufferLockBaseAddress(buffer, .readOnly)
        defer { CVPixelBufferUnlockBaseAddress(buffer, .readOnly) }
        guard let base = CVPixelBufferGetBaseAddress(buffer) else { return }
        let width = CVPixelBufferGetWidth(buffer)
        let height = CVPixelBufferGetHeight(buffer)
        let bytesPerRow = CVPixelBufferGetBytesPerRow(buffer)
        let ptr = base.assumingMemoryBound(to: UInt8.self)

        var rTotal: Double = 0, gTotal: Double = 0, bTotal: Double = 0, count: Double = 0
        let step = max(1, min(width, height) / 24)
        for y in stride(from: 0, to: height, by: step) {
            let row = ptr + y * bytesPerRow
            for x in stride(from: 0, to: width, by: step) {
                let p = row + x * 4
                bTotal += Double(p[0]); gTotal += Double(p[1]); rTotal += Double(p[2]); count += 1
            }
        }
        guard count > 0 else { return }
        let r = rTotal / count / 255.0, g = gTotal / count / 255.0, b = bTotal / count / 255.0
        let light = (0.2126 * r + 0.7152 * g + 0.0722 * b) * 100
        DispatchQueue.main.async { self.red = r; self.green = g; self.blue = b; self.brightness = light }
    }
}
