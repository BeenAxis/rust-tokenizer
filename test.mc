import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar

plugins {
    kotlin("jvm")
    id("com.google.devtools.ksp")
    id("com.github.johnrengelman.shadow") version "7.1.2"
    `maven-publish`
}

fun isRelease() = System.getenv("isRelease")?.toBoolean() == true

val mavenLocal: String by project

tasks.withType<ShadowJar> {
    exclude("kotlin/**")
    exclude("/kotlinx/**")
}

repositories {
    mavenCentral()
    if(mavenLocal.isNotBlank() && !isRelease()) {
        mavenLocal()
    } else {
        maven {
            credentials {
                username = project.findProperty("funtimeMavenRepoUser") as? String ?: System.getenv("funtimeMavenRepoUser")
                password = project.findProperty("funtimeMavenRepoPass") as? String ?: System.getenv("funtimeMavenRepoPass")
            }
            url = uri("https://maven.pkg.github.com/FunTimeMC/*") // FunTimeMC organization packages
        }
    }
}

val kotlinPlugVer: String by project
val nettySerialVer: String by project
val serverCoreVer: String by project
val advanceThingsVer: String by project
val fttpVer: String by project

dependencies {
    compileOnly("ru.beenaxis:kotlinplug:$kotlinPlugVer")
    compileOnly("org.mongodb:mongodb-driver-kotlin-sync:4.10.2")
    compileOnly("ru.beenaxis:servercore-libs:$serverCoreVer")
    compileOnly("ru.beenaxis:fttp-libs:$fttpVer")
    //compileOnly("ru.beenaxis:advance-things:$advThingsVer")

    compileOnly("ru.beenaxis:netty-serial:$nettySerialVer")
    ksp("ru.beenaxis:netty-serial:$nettySerialVer")
}

publishing {
    val mavenLocal: String by project
    val projectVer: String by project
    publications {
        create<MavenPublication>("mavenKotlin") {
            from(components["java"])
            groupId = "ru.beenaxis"
            artifactId = "funprofile-libs"
            version = projectVer
        }
    }
    repositories {
        if(mavenLocal.isNotBlank() && !isRelease()){
            mavenLocal()
        } else {
            maven {
                credentials {
                    username = project.findProperty("funtimeMavenRepoUser") as? String ?: System.getenv("funtimeMavenRepoUser")
                    password = project.findProperty("funtimeMavenRepoPass") as? String ?: System.getenv("funtimeMavenRepoPass")
                }
                url = uri("https://maven.pkg.github.com/FunTimeMC/FunProfileRepo") // FunTimeMC organization packages
            }
        }
    }
}

tasks {
    build {
        dependsOn(shadowJar)
    }
    shadowJar {
        archiveBaseName.set("libs")
        archiveVersion.set("")
        archiveClassifier.set("")
    }
}

tasks.test {
    useJUnitPlatform()
}