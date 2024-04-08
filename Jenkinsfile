pipeline {
    agent any

    stages{
        stage("Verify cargo"){
            steps{
                sh 'cargo --version'
            }
        }
        stage("Build"){
            steps{
                sh 'cargo build'
            }
        }
        stage("Test"){
            steps{
                sh 'cargo test'
            }
        }
    }
}