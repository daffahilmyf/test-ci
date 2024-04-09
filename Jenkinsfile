pipeline {
    agent any

    stages {
        stage("Verify cargo") {
            steps {
                echo "IS_WINDOWS=${IS_WINDOWS}"
                script {
                    if (IS_WINDOWS) {
                        bat 'cargo --version'
                    } else {
                        sh 'cargo --version'
                    }
                }
            }
        }
        stage("Build") {
            steps {
                script {
                    if (IS_WINDOWS) {
                        bat 'cargo build'
                    } else {
                        sh 'cargo build'
                    }
                }
            }
        }
        stage("Test") {
            steps {
                script {
                    if (IS_WINDOWS) {
                        bat 'cargo test'
                    } else {
                        sh 'cargo test'
                    }
                }
            }
        }
    }
}
