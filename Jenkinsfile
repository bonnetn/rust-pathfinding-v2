pipeline {
    environment {
        GITHUB_TOKEN = credentials('github_token')
    }
    agent { 
      dockerfile true
    }
    stages {
        stage('Build') {
            steps {
                sh 'env'
            }
        }
    }
}
