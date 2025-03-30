#!/bin/bash

# UniversalPath.org Deployment Script
# This script helps set up and deploy the application to a production server

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Configuration
DB_NAME=${DB_NAME:-"catman"}
DB_USER=${DB_USER:-"catman"}
DB_PASS=${DB_PASS:-"xB6udTcQaR2afdDX"}
DOMAIN=${DOMAIN:-"universalpath.org"}
APP_USER=${APP_USER:-"universalpath"}
APP_DIR=${APP_DIR:-"/home/$APP_USER/universalpath.org"}
USE_SSL=${USE_SSL:-"true"}

# Helper function for section headers
section() {
    echo -e "\n${YELLOW}=== $1 ===${NC}"
}

# Check if script is running as root
check_root() {
    if [ "$(id -u)" != "0" ]; then
        echo -e "${RED}This script must be run as root${NC}" 1>&2
        exit 1
    fi
}

# Update system packages
update_system() {
    section "Updating System Packages"
    apt update
    apt upgrade -y
}

# Install required packages
install_packages() {
    section "Installing Required Packages"
    apt install -y build-essential pkg-config libssl-dev mariadb-server nginx certbot python3-certbot-nginx curl
}

# Create application user
create_app_user() {
    section "Creating Application User"
    if id "$APP_USER" &>/dev/null; then
        echo -e "User $APP_USER already exists"
    else
        useradd -m -s /bin/bash "$APP_USER"
        echo -e "User $APP_USER created"
    fi
}

# Set up the database
setup_database() {
    section "Setting Up Database"
    
    # Check if database exists
    if mysql -e "use $DB_NAME" 2>/dev/null; then
        echo -e "Database $DB_NAME already exists"
    else
        # Create database and user
        mysql -e "CREATE DATABASE $DB_NAME CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
        mysql -e "CREATE USER '$DB_USER'@'localhost' IDENTIFIED BY '$DB_PASS';"
        mysql -e "GRANT ALL PRIVILEGES ON $DB_NAME.* TO '$DB_USER'@'localhost';"
        mysql -e "FLUSH PRIVILEGES;"
        
        echo -e "Database and user created successfully"
        
        # Import initial schema if migration file exists
        if [ -f "$APP_DIR/migrations/20250329_initial.sql" ]; then
            mysql "$DB_NAME" < "$APP_DIR/migrations/20250329_initial.sql"
            echo -e "Initial database schema imported"
        else
            echo -e "${YELLOW}Warning:${NC} Initial migration file not found. Database is empty."
        fi
    fi
}

# Set up Nginx configuration
setup_nginx() {
    section "Setting Up Nginx"
    
    # Create nginx configuration
    cat > /etc/nginx/sites-available/$DOMAIN <<EOF
server {
    listen 80;
    server_name $DOMAIN www.$DOMAIN;
    
    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
    }
    
    # Статические файлы
    location /static/ {
        alias $APP_DIR/static/;
        expires 30d;
        add_header Cache-Control "public, max-age=2592000";
    }
    
    # Для больших загрузок
    client_max_body_size 20M;
}
EOF

    # Enable site
    if [ -f /etc/nginx/sites-enabled/$DOMAIN ]; then
        rm /etc/nginx/sites-enabled/$DOMAIN
    fi
    
    ln -s /etc/nginx/sites-available/$DOMAIN /etc/nginx/sites-enabled/
    
    # Test nginx configuration
    nginx -t
    
    # Restart nginx
    systemctl restart nginx
    
    echo -e "Nginx configured successfully"
}

# Install SSL certificate using Let's Encrypt
install_ssl() {
    section "Installing SSL Certificate"
    
    if [ "$USE_SSL" = "true" ]; then
        certbot --nginx -d $DOMAIN -d www.$DOMAIN --non-interactive --agree-tos --email admin@$DOMAIN
        echo -e "SSL certificate installed successfully"
    else
        echo -e "Skipping SSL certificate installation"
    fi
}

# Install Rust
install_rust() {
    section "Installing Rust"
    
    if ! command -v rustc &> /dev/null; then
        # Install rustup as the app user
        su - $APP_USER -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
        su - $APP_USER -c "source ~/.cargo/env"
        echo -e "Rust installed successfully"
    else
        echo -e "Rust is already installed"
    fi
}

# Clone the repository
clone_repository() {
    section "Cloning Repository"
    
    if [ -d "$APP_DIR" ]; then
        echo -e "Repository directory already exists"
    else
        # Clone the repository as the app user
        su - $APP_USER -c "git clone https://github.com/your-org/universalpath.org.git"
        echo -e "Repository cloned successfully"
    fi
}

# Build the application
build_app() {
    section "Building Application"
    
    # Create .env file
    cat > $APP_DIR/.env <<EOF
DATABASE_URL=mysql://$DB_USER:$DB_PASS@localhost:3306/$DB_NAME
JWT_SECRET=$(openssl rand -hex 32)
SITE_URL=$DOMAIN
EOF
    
    chown $APP_USER:$APP_USER $APP_DIR/.env
    
    # Build the application as the app user
    su - $APP_USER -c "cd $APP_DIR && source ~/.cargo/env && cargo build --release"
    
    echo -e "Application built successfully"
}

# Create systemd service
create_service() {
    section "Creating Systemd Service"
    
    cat > /etc/systemd/system/universalpath.service <<EOF
[Unit]
Description=UniversalPath.org Web Server
After=network.target mysql.service

[Service]
User=$APP_USER
WorkingDirectory=$APP_DIR
Environment="ROCKET_ENV=production"
EnvironmentFile=$APP_DIR/.env
ExecStart=$APP_DIR/target/release/universal_path
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
EOF
    
    # Reload systemd configuration
    systemctl daemon-reload
    
    # Enable and start the service
    systemctl enable universalpath.service
    systemctl start universalpath.service
    
    echo -e "Systemd service created and started"
}

# Check service status
check_status() {
    section "Checking Service Status"
    
    systemctl status universalpath.service
    
    echo -e "Deployment complete! Visit: https://$DOMAIN"
}

# Set up automatic backups
setup_backups() {
    section "Setting Up Automatic Backups"
    
    # Create backup directory
    mkdir -p /home/$APP_USER/backups
    chown $APP_USER:$APP_USER /home/$APP_USER/backups
    
    # Create backup script
    cat > /home/$APP_USER/backup.sh <<EOF
#!/bin/bash
BACKUP_DIR="/home/$APP_USER/backups"
DATE=\$(date +"%Y%m%d")
mysqldump -u $DB_USER -p$DB_PASS $DB_NAME > \$BACKUP_DIR/db_backup_\$DATE.sql
tar -czf \$BACKUP_DIR/files_backup_\$DATE.tar.gz $APP_DIR
find \$BACKUP_DIR -name "db_backup_*.sql" -mtime +30 -delete
find \$BACKUP_DIR -name "files_backup_*.tar.gz" -mtime +30 -delete
EOF
    
    chmod +x /home/$APP_USER/backup.sh
    chown $APP_USER:$APP_USER /home/$APP_USER/backup.sh
    
    # Set up cron job
    (crontab -u $APP_USER -l 2>/dev/null || echo "") | { cat; echo "0 3 * * * /home/$APP_USER/backup.sh > /dev/null 2>&1"; } | crontab -u $APP_USER -
    
    echo -e "Automatic daily backups set up with 30-day retention"
}

# Main function to run the deployment
main() {
    echo -e "${GREEN}Starting UniversalPath.org Deployment${NC}"
    
    check_root
    update_system
    install_packages
    create_app_user
    setup_database
    setup_nginx
    install_rust
    clone_repository
    build_app
    create_service
    if [ "$USE_SSL" = "true" ]; then
        install_ssl
    fi
    setup_backups
    check_status
    
    echo -e "\n${GREEN}Deployment completed successfully!${NC}"
    echo -e "Your website is now running at: ${USE_SSL:+https://}${USE_SSL:-http://}$DOMAIN"
}

# Check if a specific function was requested
if [ $# -gt 0 ]; then
    $1
else
    # Run the full deployment
    main
fi