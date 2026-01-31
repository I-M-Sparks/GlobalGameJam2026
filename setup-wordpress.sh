#!/bin/bash
# WordPress Setup Script for WSL
# This script sets up WordPress on WSL with MySQL and PHP

set -e

echo "=== Themathar Game WordPress Setup ==="

# Update package manager
echo "Updating package manager..."
sudo apt-get update
sudo apt-get upgrade -y

# Install MySQL Server
echo "Installing MySQL Server..."
sudo apt-get install -y mysql-server

# Install PHP and required extensions
echo "Installing PHP and extensions..."
sudo apt-get install -y php php-cli php-fpm php-mysql php-curl php-gd php-json php-mbstring php-xml php-xmlrpc php-zip

# Install Nginx
echo "Installing Nginx..."
sudo apt-get install -y nginx

# Install WordPress dependencies
echo "Installing WordPress dependencies..."
sudo apt-get install -y wget

# Create WordPress directory
echo "Setting up WordPress directory..."
WORDPRESS_DIR="/var/www/themathar"
sudo mkdir -p $WORDPRESS_DIR
sudo chown -R $USER:$USER $WORDPRESS_DIR

# Download WordPress
echo "Downloading WordPress..."
cd $WORDPRESS_DIR
wget -q https://wordpress.org/latest.tar.gz -O - | tar xz
mv wordpress/* .
rmdir wordpress

# Create WordPress configuration
echo "Creating WordPress configuration..."
cp wp-config-sample.php wp-config.php

# Generate WordPress salts and keys
SALTS=$(curl -s https://api.wordpress.org/secret-key/1.1/salt/)
php << 'EOF'
$config = file_get_contents('/var/www/themathar/wp-config.php');
$salts = file_get_contents('php://stdin');

$config = preg_replace(
    '/define\(\s*["\']AUTH_KEY["\']\s*,\s*["\'](put your unique phrase here)["\'][^\n]*\n/',
    '',
    $config
);
$config = preg_replace(
    '/^define\(\s*["\']([A-Z_]+)["\']\s*,\s*["\']put your unique phrase here["\']\s*\);$/m',
    '',
    $config
);

// Insert salts before the comment asking for them
$insertPoint = strpos($config, "/**#@+");
if ($insertPoint !== false) {
    $config = substr_replace($config, $salts . "\n", $insertPoint, 0);
}

file_put_contents('/var/www/themathar/wp-config.php', $config);
EOF

# Start MySQL and create database
echo "Starting MySQL service..."
sudo service mysql start

# Create database and user
echo "Creating WordPress database..."
sudo mysql -u root -e "CREATE DATABASE IF NOT EXISTS wordpress_themathar;"
sudo mysql -u root -e "CREATE USER IF NOT EXISTS 'wordpress'@'localhost' IDENTIFIED BY 'wordpress_password';"
sudo mysql -u root -e "GRANT ALL PRIVILEGES ON wordpress_themathar.* TO 'wordpress'@'localhost';"
sudo mysql -u root -e "FLUSH PRIVILEGES;"

# Update wp-config.php with database details
sed -i "s/database_name_here/wordpress_themathar/g" $WORDPRESS_DIR/wp-config.php
sed -i "s/username_here/wordpress/g" $WORDPRESS_DIR/wp-config.php
sed -i "s/password_here/wordpress_password/g" $WORDPRESS_DIR/wp-config.php

# Configure Nginx
echo "Configuring Nginx..."
sudo tee /etc/nginx/sites-available/themathar > /dev/null <<EOF
server {
    listen 80;
    server_name _;

    root /var/www/themathar;
    index index.php;

    client_max_body_size 100M;

    location / {
        try_files \$uri \$uri/ /index.php?\$query_string;
    }

    location ~ \.php$ {
        fastcgi_pass unix:/run/php/php-fpm.sock;
        fastcgi_param SCRIPT_FILENAME \$document_root\$fastcgi_script_name;
        include fastcgi_params;
        fastcgi_read_timeout 60s;
    }

    location ~ /\.ht {
        deny all;
    }
}
EOF

# Enable the site
sudo ln -sf /etc/nginx/sites-available/themathar /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default

# Test Nginx configuration
echo "Testing Nginx configuration..."
sudo nginx -t

# Start Nginx
echo "Starting Nginx..."
sudo service nginx start
sudo service nginx reload

# Start PHP-FPM
echo "Starting PHP-FPM..."
sudo service php-fpm start

# Copy plugin to WordPress plugins directory
echo "Installing Themathar plugin..."
PLUGIN_SOURCE="../wordpress-plugin/themathar-game"
if [ -d "$PLUGIN_SOURCE" ]; then
    sudo cp -r "$PLUGIN_SOURCE" $WORDPRESS_DIR/wp-content/plugins/
    sudo chown -R www-data:www-data $WORDPRESS_DIR/wp-content/plugins/themathar-game
else
    echo "Warning: Plugin source not found at $PLUGIN_SOURCE"
fi

# Set proper permissions
echo "Setting file permissions..."
sudo chown -R www-data:www-data $WORDPRESS_DIR
sudo find $WORDPRESS_DIR -type f -exec chmod 644 {} \;
sudo find $WORDPRESS_DIR -type d -exec chmod 755 {} \;
sudo chmod 600 $WORDPRESS_DIR/wp-config.php

echo ""
echo "=== WordPress Setup Complete ==="
echo "WordPress URL: http://localhost"
echo "WordPress Admin: http://localhost/wp-admin"
echo ""
echo "Next steps:"
echo "1. Open http://localhost in your browser"
echo "2. Complete the WordPress installation"
echo "3. Activate the Themathar Game plugin"
echo ""
echo "To access from Windows, use your WSL IP address:"
hostname -I | awk '{print "http://" $1}'
