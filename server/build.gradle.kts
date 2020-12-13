import com.google.protobuf.gradle.*
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
	id("org.springframework.boot") version "2.4.1"
	id("io.spring.dependency-management") version "1.0.10.RELEASE"
	kotlin("jvm") version "1.4.21"
	kotlin("plugin.spring") version "1.4.21"
	id("com.google.protobuf") version "0.8.14"
	id("com.google.cloud.tools.jib") version "2.7.0"
}

group = "mountains"
version = "0.1.0"
java.sourceCompatibility = JavaVersion.VERSION_11

repositories {
	mavenCentral()
}

dependencies {
	implementation("io.github.lognet:grpc-spring-boot-starter:4.2.3")
	implementation("org.springframework.boot:spring-boot-starter")

	implementation("io.grpc:grpc-kotlin-stub:1.0.0")
	implementation("org.jetbrains.kotlin:kotlin-stdlib")
	implementation("org.jetbrains.kotlin:kotlin-reflect")
	implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
	implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.3.8")

	// Grpc and Protobuf
	api("io.grpc:grpc-protobuf:1.34.0")
	api("io.grpc:grpc-stub:1.34.0")

	testImplementation("org.springframework.boot:spring-boot-starter-test")
}

tasks.withType<KotlinCompile> {
	kotlinOptions {
		freeCompilerArgs = listOf("-Xjsr305=strict")
		jvmTarget = "11"
	}
}

tasks.withType<Test> {
	useJUnitPlatform()
}

tasks.withType<Wrapper> {
	gradleVersion = "6.7.1"
}

sourceSets {
	main {
		java {
			setSrcDirs(
				listOf(
					"build/generated/source/proto/main/grpc",
					"build/generated/source/proto/main/grpckt",
					"build/generated/source/proto/main/java"
				)
			)
		}
	}
}

val protobufVersion = "3.14.0"
val grpcVersion = "1.34.0"
val grpcKotlinVersion = "1.0.0"

protobuf {
	protoc {
		artifact = "com.google.protobuf:protoc:$protobufVersion"
	}
	plugins {
		id("grpc") {
			artifact = "io.grpc:protoc-gen-grpc-java:$grpcVersion"
		}
		id("grpckt") {
			artifact = "io.grpc:protoc-gen-grpc-kotlin:$grpcKotlinVersion:jdk7@jar"
		}
	}
	generateProtoTasks {
		all().forEach {
			it.plugins {
				id("grpc")
				id("grpckt")
			}
		}

	}
}

val registry = System.getenv("REGISTRY")

jib {
	to {
		image = "$registry/mountains-server:$version"
	}
}
