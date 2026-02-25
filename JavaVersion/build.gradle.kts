plugins {
    java
}

val javaVersionInt = (property("java-version") as String).toInt()

group = property("group") as String
version = property("version") as String
description = "MMCLL"

repositories {
    mavenCentral()
}

subprojects {
    apply(plugin = "java-library")

    dependencies {
        implementation("org.jspecify:jspecify:1.0.0")
        implementation("jakarta.validation:jakarta.validation-api:3.1.1")
    }
}
