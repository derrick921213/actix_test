# Test API CALL
## 概述 (Overview)
測試前端人員是否會使用API調用
## API路徑與方法
### Readme
- HTTP方法: `GET`
- URL: `/readme`
- 成功響應: `string`

### Hello, World
- HTTP方法: `POST`
- URL: `/`
- Header: `Content-Type: application/json`
- 請求參數:
    - username: string, 必填, 用戶名稱
- 成功響應: `string`

### Add Node
- HTTP方法: `POST`
- URL: `/add_node`
- Header: `Content-Type: application/json`
- 請求參數:
    - id: String,
    - name: String,
    - status: String,
    - resource: {
        cpu_usage: f64,
        memory_usage: u32,
        disk_usage: u64,
    },
- 成功響應: `string`, 所有Node節點

### Remove Node
- HTTP方法: `POST`
- URL: `/del_node`
- Header: `Content-Type: application/json`
- 請求參數:
    - id: String,
- 成功響應: `string`, 所有Node節點

### Get Node
- HTTP方法: `POST`
- URL: `/get_node`
- Header: `Content-Type: application/json`
- 請求參數:
    - id: String,
- 成功響應: `string`, 單獨Node節點