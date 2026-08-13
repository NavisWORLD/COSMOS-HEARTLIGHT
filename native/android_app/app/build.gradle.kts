plugins { id("com.android.application") }

android {
    namespace = "world.navis.heartlight"
    compileSdk = 35

    defaultConfig {
        applicationId = "world.navis.heartlight"
        minSdk = 24
        targetSdk = 35
        versionCode = 3
        versionName = "0.3.0"
    }

    buildTypes {
        release { isMinifyEnabled = false }
    }
}

dependencies {
    implementation("androidx.webkit:webkit:1.12.1")
}
