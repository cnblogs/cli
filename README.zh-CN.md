# Cnblogs 命令行工具

[![Build / Release](https://github.com/cnblogs/cli/actions/workflows/build-release.yml/badge.svg)](https://github.com/cnblogs/cli/actions/workflows/build-release.yml)
[![Build / Development](https://github.com/cnblogs/cli/actions/workflows/build-dev.yml/badge.svg)](https://github.com/cnblogs/cli/actions/workflows/build-dev.yml)

从 CLI 访问 cnblogs。

## Cnbogs Cli 设计

从Cnblogs的[OpenAPI](https://api.cnblogs.com/help)来说，API主要有以下几类:

1. Token: 认证
2. Users: 仅提供当前登录用户信息
3. Blogs: 博客的CURD及其评论的查看和增加，
4. Marks: 收藏的CURD
5. News: 新闻的查询，新闻评论的CURD
6. Statuses: 闪存CURD。
7. Questions: 问题相关操作
8. Edu: 班级相关
9. Articles: 知识库的查找。
10. Zzk: 找找看

### cli的使用

目前cli的使用如下：

```shell
# Check your post list
cnb post --list
# Check your post 
cnb --id 114514 post --show
# Create and publish post 
cnb post create --title 'Hello' --body 'world!' --publish
# Change your post body
cnb --id 114514 post update --body 'niconiconiconi'

# Show ing list
cnb ing list
# Publish ing 
cnb ing --publish 'Hello world!'
# Comment to ing 
cnb --id 114514 ing --comment 'Awesome!'

# Check your user infomation
cnb user --info
```

大体上使用如上的设计，支持子命令，相关操作的设计按照RESTFUL的思路设计实现，博客的相关操作设计如下：

```shell
cnb posts [comment] [list,create,query,delete,update] --[id/file/quertset] --[pagesize,pagecount] 
```

## 闪存cli

闪存cli设计如下：

```sh
cnb ing query   # 默认10条
cnb ing query --id 123456
cnb ing query --page 1 --count 10
cnb ing query --type All --page 1 --count 10 --tag Linux
cnb ing create --conent hello --private false --lucky false
cnb ing delete --id 123456
```
