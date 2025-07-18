###
 # @Author: LeiJiulong
 # @Date: 2025-07-17 12:07:04
 # @LastEditors: LeiJiulong && lei15557570906@outlook.com
 # @LastEditTime: 2025-07-17 12:07:05
 # @Description: 
### 
export DATABASE_USER=truuser
export PROJECT_ROOT=.
psql -U truuser -h localhost -d ezytutors < $PROJECT_ROOT/src/database_iter4.sql 