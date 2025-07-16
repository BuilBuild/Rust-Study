###
 # @Author: LeiJiulong
 # @Date: 2025-07-16 14:45:12
 # @LastEditors: LeiJiulong && lei15557570906@outlook.com
 # @LastEditTime: 2025-07-16 15:02:18
 # @Description: 
### 

#!/bin/bash

echo a
curl -X POST http://localhost:3000/courses/ \
-H "Content-Type: application/json" \
-d '{"tutor_id": 1, "course_name": "Hello, my first course !"}' 