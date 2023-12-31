# DiskGameSaver
学习版的单机游戏存档往往不能云存档，这里提供一个将游戏存档集中到磁盘上的功能，能够将存档实时同步到用户指定的文件夹中（比如OneDrive）。

## 用户需求
1. 启动游戏，显示游戏封面，排序游戏，分类游戏，并设置游戏存档位置。
   1. 游戏名、游戏存档对应关系应该单独保存，因为正常来说游戏存档都在特定的目录下。一款游戏可以有多个不同的名字（中文、英文、游戏exe名字等）。也可以通过检查游戏exe路径来实现。
   2. 我们可以要求用户将游戏都放到和该软件相同的目录下，以方便使用相对路径。
2. 点击 `上传` 按钮后，将电脑上指定游戏的存档同步到 `指定目录/GameSaves/Username/` 文件夹里。
3. 点击 `备份` 按钮后，备份当前电脑上所有游戏存档到`指定目录/GameSaves/Username/Backup/date-time`文件夹下。
4. 点击 `下载` 按钮后，
   1. 备份游戏存档。
   2. 将`指定目录/GameSaves/Username/`里的存档覆盖到本地。
5. 未来需求：
   1. 开机启动，自动同步存档（类似OneDrive）



# 笔记

1. 区分user_settings和common settings。

2. Common settings可能包含

   

```mermaid
classDiagram
    class Games {
    
    }
```

# 资源

游戏封面：https://gvcover.top/?s=%E5%8F%AA%E7%8B%BC

游戏名匹配：https://www.igdb.com 在这个网站的搜索框输入的时候会有get请求，帮你匹配。这个接口是好调用的。
