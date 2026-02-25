plugins {
    id("java")
}

group = "mmcll"
version = "0.1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    val lombokVersion = property("lombokVersion") as String
    compileOnly("org.projectlombok:lombok:$lombokVersion")
    annotationProcessor("org.projectlombok:lombok:$lombokVersion")

    testCompileOnly("org.projectlombok:lombok:$lombokVersion")
    testAnnotationProcessor("org.projectlombok:lombok:$lombokVersion")
}
