pipeline {
    agent any

    stages {
        stage("Verify cargo") {
            steps {
                echo "Running on ${os}"
                script {
                    if (os == "WINDOWS") {
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
                    if (os == "WINDOWS") {
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
                    if (os == "WINDOWS") {
                        bat 'cargo test'
                    } else {
                        sh 'cargo test'
                    }
                }
            }
        }
    }
}
