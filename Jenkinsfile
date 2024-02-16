pipeline {
    agent any

    environment {
        DOCKER_IMAGE_NAME = 'rust_calc'
        GITHUB_REPO_URL = 'https://github.com/SiddharthVPillai/rust_calc.git'
        RUST_VERSION = '1.56.0'
    
        PATH = "${env.HOME}/.cargo/bin:${env.PATH}"
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
        stage('Unit test') {
            steps {
                // Install Rust, configure Rust version, etc.
                // This step might be unnecessary if Rust is already installed on the Jenkins agent
                
                // sh 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
                // echo "verion"
                
                dir('rust_calc') {
                    // Run cargo commands
                    sh 'cargo build'
                    sh 'cargo test'
                }
                // sh 'cargo --version'
                // sh 'cargo build'
            }
        }
        
        // stage('Build') {
        //     steps {
        //         // Build the Rust project
                
        //     }
        // }

        // stage('Unit testing') {
        //     steps {
        //         script {
        //             // Checkout the code from the GitHub repository
        //             // sh 'cargo build'
                    
        //         }
        //     }
        // }

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