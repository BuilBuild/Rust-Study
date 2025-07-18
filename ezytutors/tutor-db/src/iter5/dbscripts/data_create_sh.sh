###
 # @Author: LeiJiulong
 # @Date: 2025-07-18 16:36:27
 # @LastEditors: LeiJiulong && lei15557570906@outlook.com
 # @LastEditTime: 2025-07-18 16:37:31
 # @Description: 
### 
export DATABASE_USER=truuser
export PROJECT_ROOT=.
psql -U truuser -h localhost -d ezytutors < tutor-course.sql