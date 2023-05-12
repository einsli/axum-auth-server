# Axum User Auth Server

## 服务描述
> 该服务基于axum框架，数据库采用mysql存储，基于jwt token验证，以及redis作为缓存来实现的一个用户登录以及注册的服务


## 实现功能
- [X] 用户创建
- [X] 用户登录
- [X] 用户信息获取
- [X] 注销

## API 

- 用户注册
```shell
curl --location '127.0.0.1:8088/user/register' \
--header 'Content-Type: application/json' \
--data '{
    "user_name": "zhangsan",
    "password": "123456"
}'
# output
{
    "code": 0,
    "msg": "success",
    "data": null
}
```

- 用户登录
```shell
curl --location '127.0.0.1:8088/user/login' \
--header 'Content-Type: application/json' \
--data '{
    "user_name": "zhangsan",
    "password": "zhangsan123"
}'
# output
{
    "code": 0,
    "msg": "success",
    "data": {
        "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZDFjOWQ5Mi1iMzFiLTQwNmMtOGZlOC1kNWZlMmY5ODEwMWMiLCJleHAiOjE2ODM4MjI0OTIsImlhdCI6MTY4MzgxNTI5Mn0.PkNtbhMhVti9c2oX5euC2fgUu0nEymxEvsp8P43EPoA",
        "token_type": "Bearer"
    }
}
```

- 用户信息获取
```shell
curl --location '127.0.0.1:8088/api/get_me' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZDFjOWQ5Mi1iMzFiLTQwNmMtOGZlOC1kNWZlMmY5ODEwMWMiLCJleHAiOjE2ODM4MjI0OTIsImlhdCI6MTY4MzgxNTI5Mn0.PkNtbhMhVti9c2oX5euC2fgUu0nEymxEvsp8P43EPoA'
#output
{
    "code": 0,
    "msg": "success",
    "data": {
        "user_id": "3d1c9d92-b31b-406c-8fe8-d5fe2f98101c",
        "user_name": "zhangsan"
    }
}

# unauthorized status
{
    "code": 403,
    "msg": "Authorization Error",
    "data": null
}
```

- 用户注销
```shell
curl --location '127.0.0.1:8088/api/logout' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZDFjOWQ5Mi1iMzFiLTQwNmMtOGZlOC1kNWZlMmY5ODEwMWMiLCJleHAiOjE2ODM4MjI0OTIsImlhdCI6MTY4MzgxNTI5Mn0.PkNtbhMhVti9c2oX5euC2fgUu0nEymxEvsp8P43EPoA'

# output
{
    "code": 0,
    "msg": "success",
    "data": null
}
```

## 特别提醒
> 该程序仅用于学习交流，不能上生产！！
> 
> 如有疑问，请联系 415401944(WX)
