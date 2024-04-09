pipeline {
    agent any

    stages {
        stage("Verify cargo") {
            steps {
                echo "IS_WINDOWS=${IS_WINDOWS}"
                script {
                    try {
                        if (IS_WINDOWS) {
                            bat 'cargo --version'
                        } else {
                            sh 'cargo --version'
                        }
                    } catch (Exception e) {
                        echo "Failed to verify cargo: ${e}"
                        // You can also use error() to mark the stage as failed
                        error("Failed to verify cargo")
                    }
                }
            }
        }
        stage("Build") {
            steps {
                script {
                    try {
                        if (IS_WINDOWS) {
                            bat 'cargo build'
                        } else {
                            sh 'cargo build'
                        }
                    } catch (Exception e) {
                        echo "Build failed: ${e}"
                        error("Build failed")
                    }
                }
            }
        }
        stage("Test") {
            steps {
                script {
                    try {
                        if (IS_WINDOWS) {
                            bat 'cargo test'
                        } else {
                            sh 'cargo test'
                        }
                    } catch (Exception e) {
                        echo "Tests failed: ${e}"
                        error("Tests failed")
                    }
                }
            }
        }
    }
}
