###
 # @Author: LeiJiulong
 # @Date: 2025-07-16 20:48:55
 # @LastEditors: LeiJiulong && lei15557570906@outlook.com
 # @LastEditTime: 2025-07-16 20:53:26
 # @Description: 
### 
sudo -i -u postgres
psql
create database ezytutors;
create user truuser with password 'mypassword';
grant all privileges on database ezytutors to truuser;
\q
export DATABASE_USER=truuser
psql -U $DATABASE_USER -d ezytutors --password
psql -U truuser -d ezytutors -h localhost --password