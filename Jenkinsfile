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
    post{
        always{
            echo "========always========"
        }
        success{
            echo "========pipeline executed successfully ========"
        }
        failure{
            echo "========pipeline execution failed========"
        }
    }
}