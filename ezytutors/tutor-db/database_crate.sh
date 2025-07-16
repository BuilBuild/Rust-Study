###
 # @Author: LeiJiulong
 # @Date: 2025-07-16 21:07:27
 # @LastEditors: LeiJiulong && lei15557570906@outlook.com
 # @LastEditTime: 2025-07-16 21:08:59
 # @Description: 
### 
export DATABASE_USER=truuser
export PROJECT_ROOT=.
psql -U truuser -h localhost -d ezytutors < $PROJECT_ROOT/src/database.sql 