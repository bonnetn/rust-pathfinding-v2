pipeline {
    environment {
        GITHUB_TOKEN = credentials('github_token_release')
    }
    agent none
    stages {
        stage('Build') {
            agent { 
              dockerfile true
            }
            steps {
                sh 'apk add --update bash curl git && bash scripts/publish_github_release.sh "$GITHUB_TOKEN" /artifact/*'
            }
        }
        stage('Test') {
            agent { 
              dockerfile {
                filename 'Dockerfile_test'
              }
            }
            steps {
                sh 'cargo test'
            }
        }
    }
}
