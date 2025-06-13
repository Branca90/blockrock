#!/bin/bash
npm run build
sudo rm -rf /var/www/html/nexus/*
sudo cp -r dist/* /var/www/html/nexus
sudo chown -R www-data:www-data /var/www/html/nexus
sudo chmod -R 755 /var/www/html/nexus
sudo systemctl reload nginx
