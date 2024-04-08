pipeline {
    agent any

    stages {
        stage("Verify cargo") {
            steps {
                script {
                    def command = IS_WINDOWS ? 'cargo --version' : 'cargo --version'
                    sh command
                }
            }
        }
        stage("Build") {
            steps {
                script {
                    def command = IS_WINDOWS ? 'cargo build' : 'cargo build'
                    sh command
                }
            }
        }
        stage("Test") {
            steps {
                script {
                    def command = IS_WINDOWS ? 'cargo test' : 'cargo test'
                    sh command
                }
            }
        }
    }
}
