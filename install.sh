#!/bin/bash

# Variables
REPO_URL="https://github.com/Estikno/oxsh"
REPO_ARCHIVE_URL="https://github.com/Estikno/oxsh/releases/download/0.1.0/oxsh"
HOME_DIR="$HOME"
OXSHRC_FILE="$HOME_DIR/oxshrc.sh"
OXSH_EXECUTABLE="$HOME_DIR/oxsh"

echo "Downloading from GitHub repository..."
wget -O "$HOME_DIR/oxsh" "$REPO_ARCHIVE_URL"

echo "Creating the oxshrc.sh file..."
cat <<EOL > "$OXSHRC_FILE"
echo 'Welcome to OXSH 0.1.0'
echo ''
echo ' * Repository:  https://github.com/Estikno/oxsh'
echo ''
echo ' * Strictly confined Kubernetes makes edge and IoT secure. Learn how MicroK8s'
echo '   just raised the bar for easy, resilient and secure K8s cluster deployment.'
echo ''
echo '   https://ubuntu.com/engage/secure-kubernetes-at-the-edge'
echo ''
echo 'This message is shown every time you log in. To change it please modify the ~/oxshrc.sh file.'
EOL

echo "Setting executable permissions for oxsh..."
chmod +x "$OXSH_EXECUTABLE"

echo "Setting permissions for oxshrc.sh..."
chmod 755 "$OXSHRC_FILE"

echo "Process completed."