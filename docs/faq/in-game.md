# 游戏内加载相关

[[toc]]

## <Badge type="info">皮肤 Mod</Badge> <Badge type="tip" text="常见" /> 这不是我自己设置的皮肤 {#not-my-skin}

这个问题通常出现在使用皮肤 Mod（如 _CustomSkinLoader_）加载皮肤的用户当中。

以 CustomSkinLoader 为例，它的默认加载顺序为 _Mojang_ > _BYS_。  
所以，<mark>如果你的角色名与某位正版用户相同，那么 CustomSkinLoader 会优先加载那位正版用户的皮肤</mark>。

🛠️ 要解决此问题，可以参照下拉框中的内容进行操作。
::: details 手动修改配置文件 {#edit-csl-config}

<!-- markdownlint-disable MD051 -->

> [!NOTE] 什么情况下需要手动修改 CustomSkinLoader 配置文件？
> 一般来说，你只需要简单地安装 CustomSkinLoader Mod 即可，无需进行任何额外的配置。
>
> 然而有时事与愿违，如果你在使用过程中遇到了 [同名冲突](/faq/in-game#not-my-skin) 这样的情况，那就是时候照着下面的步骤来做了。

<!-- markdownlint-restore -->

---

配置文件: `CustomSkinLoader.json` <a href="/CustomSkinLoader.json" download><BSButton style="background-color: var(--vp-c-success-3)"><FA :icon="faFileArrowDown" /> 下载此文件 </BSButton></a>

> [!IMPORTANT] ✅ 建议直接下载 CustomSkinLoader 配置文件
> 你可以下载到为 BYS 量身定制的配置文件，此文件的内容与下方代码块的内容一致。
>
> 👉 你只需将下载到的文件<mark>覆盖（替换）掉原有的文件</mark>即可，而无需对原文件进行编辑。

> [!TIP] 📍 配置文件的位置
> 配置文件默认存放于 `.minecraft/CustomSkinLoader/` 目录中，仅有一个配置文件，文件名为 `CustomSkinLoader.json`。
>
> 在大多数情况下，安装皮肤 Mod 后需要启动一次游戏并进入存档，配置文件才会被自动生成。

> [!TIP] 🔁 需要重启游戏
> 完成配置文件的修改后，你需要重启游戏才能使其生效。

> [!NOTE] 🥰 有关版本隔离
> 如果你在启动器中启用了版本隔离，配置文件的路径还可能与上述有少许不同。
>
> 善用启动器的 <BSSection>打开模组文件夹</BSSection> 等类似功能，结合下方图示，再给予自己一些信心，你能够找到它的。

::: details 🤔 找不到配置文件的具体位置？来看看图示
![CustomSkinLoader 文件夹所处位置](/csl-folder.webp)

![CustomSkinLoader 的配置文件和日志文件](/csl-files.webp)
:::


## <Badge type="info">皮肤 Mod</Badge> <Badge type="tip" text="常见" /> 别人看不到我的材质 {#no-skin-by-other-players}

通常，只有在多人游戏中使用皮肤 Mod 时才会遇到此问题。

在此情境下，只有 **正确安装并配置** 皮肤 Mod 后才能加载来自 LittleSkin 的材质。

🛠️ 如果你想让别人也看见你的材质，请<mark>让他们也正确安装并配置皮肤 Mod</mark>。

## <Badge type="info">皮肤 Mod</Badge> <Badge type="tip" text="常见" /> 为什么我在网站上设置好了材质，但是在游戏中不显示 / 没更新？ {#no-skin}

> [!IMPORTANT] 仅皮肤 Mod（如 CustomSkinLoader）
> 以下内容仅针对使用皮肤 Mod 加载材质的用户。
>
> 如果你使用外置登录加载材质，并遇到了如标题所说的问题，请阅读 [下一条目👇](#no-skin-in-server)。

> [!NOTE] 需要重新进入游戏
> 当你在 LittleSkin 中更新了角色的材质，大多数的皮肤 Mod 都需要重新进入游戏。
>
> 只有这样，Mod 才会重新从 LittleSkin 获取角色的材质信息。
>
> 除此之外，皮肤 Mod 还可能会将你的材质信息进行缓存，这种情况请参考下方的解决方案。

> [!NOTE] 与正版玩家重名的情况
> 如果有正版玩家的玩家名与你在 LittleSkin 上的角色名相同，在没有对 CustomSkinLoader 进行调整的情况下，你就会看到正版玩家的皮肤。
>
> 若正版玩家没有设置皮肤，你将会看到 Steve 或 Alex 的皮肤。这非常容易让人产生困惑和误解。
>
> 这种情况下，你应该参考上方的 [# <Badge type="info">皮肤 Mod</Badge> <Badge type="tip" text="常见" /> 这不是我自己设置的皮肤](#not-my-skin)。

对于使用皮肤 Mod 加载材质的用户，这是个很常见的问题。大多数情况下，这个问题是以下四个原因引起的：

::: details 1. 你没有正确地安装和配置皮肤 Mod

🛠️ 解决方案：正确安装并配置皮肤 Mod。

你可以在 [注册指南](/faq/reg#%E5%90%AF%E5%8A%A8%E5%99%A8%E9%85%8D%E7%BD%AE%E6%95%99%E7%A8%8B) 中学习如何正确配置皮肤 Mod；
:::

::: details 2. 你安装的其它 Mod（如 _NonUpdate / 不再有更新_）干扰了皮肤 Mod 与 LittleSkin 之间的连接 {#skinport}

🛠️ 解决方案：删除这些 Mod，或让它们绕过皮肤 Mod 与 LittleSkin 之间的连接。

可参考下拉框中的解决方案。
::: details SkinPort

 如果你想要在 Minecraft 1.7.10 中加载 Alex 模型的皮肤，你需要使用 SkinPort。

 你可以在以下网站获取到关于 SkinPort 的更多信息。

 - [MC 百科 (mcmod.cn)](https://www.mcmod.cn/class/2700.html)
 - [CurseForge](https://www.curseforge.com/minecraft/mc-mods/skinport)
 - [GitHub Release](https://github.com/zlainsama/SkinPort/releases/latest)

 > [!TIP] 提示
 >LittleSkin 仅支持 `1.7.10-v10a` 或更高版本的 SkinPort。
 >如果你使用 Yggdrasil 外置登录加载 Alex 材质，则只需要安装 SkinPort 即可，无需修改配置文件。

 > [!danger] 谨记
 >SkinPort 仅适用于 Minecraft 1.7.10。对于更低版本，目前没有方法加载 Alex 模型的皮肤。
 >请勿将 SkinPort 安装在其它版本的 Minecraft 上，否则可能导致游戏崩溃。

 配置文件位于 `.minecraft/config/skinport.cfg`。请使用记事本或者任意代码编辑器将其打开，将原有的所有内容替换成以下内容：

 ```java:line-numbers
 client {
    S:hostCustomServer=http://example.com
    S:hostCustomServer2Cape=https://littleskin.cn/cape/%name%.png  // [!code focus]
    S:hostCustomServer2Skin=https://littleskin.cn/skin/%name%.png  // [!code focus]
    B:useCrafatar=false
    B:useCustomServer=false
    B:useCustomServer2=true
    B:useMojang=false
 }
 ```

 保存退出，再次打开 Minecraft 之后，你应该就能看到你在 LittleSkin 中设置的材质了。
 :::

::: details 3. 你的材质被皮肤 Mod 缓存了 <Badge type="tip" text="🎯 最为常见" />
🛠️ 解决方案：等待几分钟后再试。

如果还是没有更新，尝试清除皮肤 Mod 的缓存，即**直接删除**对应的文件夹：

- CustomSkinLoader 的缓存文件夹为 `.minecraft/CustomSkinLoader/caches`
- SkinPort 的缓存文件夹为 `.minecraft/cachedImages`
:::

::: details 4. 你使用了 Alex 模型的皮肤，并且你的 Minecraft 版本低于 1.8
🛠️ 解决方案：

- 对于 1.7.10，请使用 SkinPort 加载材质。SkinPort 在 1.7.10 上提供了对 Alex 模型的支持。  
  参考 [配置SkinPort](#skinport) 进行配置。
- 对于更低版本，目前无解，你只能更换为 Steve 模型的皮肤。
:::

<Helpme afdian readBeforeAsk>

<p>如果你确定你的问题不是以上原因引起的，或者你按照以上的解决方案做了之后你的角色的材质依然没有显示或更新，请在认真仔细阅读下面这个文章后，<mark>带上你的皮肤 Mod 的日志</mark>，向我们寻求帮助。</p>
<p>当然，你也可以直接购买一对一远程技术支持服务，将由高质量的专家手把手协助你解决问题。</p>

</Helpme>

## <Badge type="info">外置登录</Badge> <Badge type="tip" text="常见" /> 单人游戏中可以正常显示皮肤，但在多人游戏中就不行 {#no-skin-in-server}

检查你有没有安装皮肤 Mod，安装之后你就可以在服务器中查看你的皮肤了。

🛠️ 推荐你<mark>安装 CustomSkinLoader</mark>。此外，你可能还需要<mark>「手动修改配置文件」</mark>。

幸运的是，这份手册中详细介绍了 [CustomSkinLoader 的配置方法](#edit-csl-config)。

## <Badge type="info">外置登录</Badge> 登入失败：身份验证正在停机维护  {#authentication-server-is-down}

请检查你是否安装了会阻止网络请求的 Mod，这会影响到外置登录的正常工作。


比较常见的 Mod 有：

- [NonUpdate](https://www.curseforge.com/minecraft/mc-mods/non-update) （NU，不再有更新）
- [NonUpdate Reloaded](https://modrinth.com/mod/non-update-reloaded) （NUR，不再有更新重制版）

🛠️ 通常来说，**直接删除这类 Mod** 是最简单快速的解决方案。但这违背了你安装此类 Mod 的初衷。

👇 此处特别列出上述 Mod 的 **配置方法**，以便在禁用更新的同时不影响 BYS 外置登录的正常工作。

::: details 🛠️ NonUpdate
将 LittleSkin 的域名添加至 `.minecraft/nu-whitelist.txt`：

```txt{5}
minecraft.net
mojang.com
// ...

littleskin.cn
```

:::

::: details 🛠️ NonUpdate Reloaded
将 LittleSkin 的域名添加至 `.minecraft/config/nonupdate_reloaded.json`:

```jsonc
{
  // ...
  "whitelist": [
    "$eminecraftservices.com",
    "$emojang.com",
    "$eminecraft.net"
    "$elittleskin.cn" // [!code ++]
  ],
  // ...
}
```

:::

## <Badge type="info">外置登录</Badge> 外置登录进入服务器时提示「无效的会话」/ accessToken 无效  {#invalid-session}

🛠️ 请先尝试退出游戏并在启动器中删除账号，然后重新登录。

<Helpme afdian readBeforeAsk>
如果以上操作均无法解决问题，请在认真仔细阅读下面这个文章后，向我们寻求帮助。
</Helpme>