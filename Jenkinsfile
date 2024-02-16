pipeline {
    agent any

    environment {
        DOCKER_IMAGE_NAME = 'rust_calc'
        GITHUB_REPO_URL = 'https://github.com/SiddharthVPillai/rust_calc.git'
    }

    stages {
        stage('Checkout') {
            steps {
                script {
                    // Checkout the code from the GitHub repository
                    git branch: 'main', url: "${GITHUB_REPO_URL}"
                }
            }
        }

        stage('Unit testing') {
            steps {
                script {
                    // Checkout the code from the GitHub repository
                    sh 'cd rust_calc'
                    sh 'cargo test'
                }
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    // Build Docker image
                    docker.build("${DOCKER_IMAGE_NAME}", '.')
                }
            }
        }

        stage('Push Docker Images') {
            steps {
                script{
                    docker.withRegistry('', 'DockerHubCred') {
                    sh 'docker tag rust_calc 0siddharth/calculator:latest'
                    sh 'docker push 0siddharth/calculator'
                    }
                 }
            }
        }

   stage('Run Ansible Playbook') {
            steps {
                script {
                    ansiblePlaybook(
                        playbook: 'deploy.yml',
                        inventory: 'inventory'
                     )
                }
            }
        }

    }
}