###
 # @Author: LeiJiulong
 # @Date: 2025-07-18 16:36:27
 # @LastEditors: LeiJiulong && lei15557570906@outlook.com
 # @LastEditTime: 2025-07-20 14:20:20
 # @Description: 
### 
export DATABASE_USER=truuser
export PROJECT_ROOT=.
psql -U truuser -h localhost -d ezytutors < tutor-course.sql
psql -U truuser -h localhost -d ezytutors < course.sql