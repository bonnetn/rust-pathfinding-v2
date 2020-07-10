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
                sh 'apk add --update bash && bash publish_github_release.sh "$GITHUB_TOKEN" /artifact/*'
            }
        }
    }
}
