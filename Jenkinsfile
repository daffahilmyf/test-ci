pipeline {
    agent any

    stages {
        stage("Verify cargo") {
            steps {
                echo "Verify cargo"
                cargo --version
            }
        }
        stage("Build") {
            steps {
                echo "Build"
            }

        }
        stage("Test") {
            steps {
                echo "Test"
            }
        }
    }
}
