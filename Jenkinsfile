pipeline {
    environment {
        GITHUB_TOKEN = credentials('github_token_release')
    }
    agent { 
      dockerfile true
    }
    stages {
        stage('Build') {
            steps {
                sh 'bash publish_github_release.sh "$GITHUB_TOKEN" *'
            }
        }
    }
}
