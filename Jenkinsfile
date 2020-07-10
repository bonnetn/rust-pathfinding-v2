pipeline {
    environment {
        GITHUB_TOKEN = credentials('test_token')
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
