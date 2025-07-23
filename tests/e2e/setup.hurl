# missing mac address
GET http://localhost:8888/api/setup
HTTP 404

# valid mac address
GET http://localhost:8888/api/setup
ID: 8C:1F:64:FB:68
HTTP 200
[Asserts]
jsonpath "$.api_key" matches "^[A-Za-z0-9]{20}$"
jsonpath "$.friendly_id" == ""

# duplicate mac address
GET http://localhost:8888/api/setup
ID: 8C:1F:64:FB:68
HTTP 500
{"message":"cannot setup new device","error":"unknown error"}
