<script setup>
import { faUser, faTrash } from '@fortawesome/free-solid-svg-icons'
</script>
> [!CAUTION] 本页面修改自Littleskin开源官方文档
> [<BSSection>点击前往</BSSection>](https://github.com/LittleSkinChina/manual-ng)

# 站点使用相关

[[toc]]

## <Badge type="tip" text="常见" /> 收不到验证邮件？ {#no-email}

请优先检查你的账号绑定的邮箱是否正确。如果不正确，请在[<BSSection><FA :icon="faUser" /> 个人资料</BSSection>](https://skin.yealqp.fun/user/profile)页面中将其更改为正确的邮箱。

再看看你的邮箱的<BSSection><FA :icon="faTrash" /> 垃圾箱</BSSection>。如果垃圾箱中也没有验证邮件，可以尝试再发送一次验证邮件。

如果你短时间内反复发送验证邮件，你的邮箱可能会被我们的邮件服务提供商封禁，这种情况下请向我们寻求帮助。

<Helpme>如果你确定你绑定的邮箱正确，并且验证邮件不在你的邮箱的垃圾箱里，请联系我们排查问题。</Helpme>

> [!IMPORTANT] 常见的错误邮箱
> 我们经常遇到有用户填写了错误的邮箱，<mark>检查一下你是否犯了同样的错误</mark>。
>
> - ❌ 使用 QQ 邮箱时将 QQ 号输入错误
>
>     ✅ 认真地、仔细地、一个一个数字地：比对检查你的 QQ 号
> - ❌ 使用 QQ 邮箱时将手机号填写在 `@qq.com` 前
>
>     ✅ 正确的做法应该是 `你的QQ号@qq.com`
> - ❌ 使用 QQ 邮箱时拼写错误，例如 `123456789@qq.coom` / `qq@qq.com123456789`
>
>     ✅ 正确做法如上，并检查是否你的输入是否有错误
> - ❌ 使用 QQ 邮箱时经过调查发现其并没有开通 QQ 邮箱服务
>
>     ✅ 正确做法是前往 <https://mail.qq.com/> 检查你是否真的开通了 QQ 邮箱
> - ❌ 使用 163 邮箱时将手机号填写在 `@163.com` 前
>
>     ✅ 正确做法是填写实际的邮箱地址
> - ...

## <Badge type="tip" text="常见" /> 我的角色被别人占用了，我可以要回来吗？ {#player-already-exists}

<mark>不可以</mark>。重名的几率很大，随意更改角色所有者对双方都不公平，所以不管你是名声多大的主播还是某个影响力巨大的人物，我们都不会提供更改角色所有者的服务。

如果你发现你的角色名被别人占用了，你可以先联系当前该角色的所有者进行协商，没准这事儿就成了，同时你还收获了一个朋友呢。如果你好好地和当前该角色的所有者商量（而不是一上来就用一种强硬的口气要求释放角色——这种情况大多不会有好结果），在大多数情况下，他们都可以是很好说话的。至于怎么联系？缘分到了就自然联系上了咯。

> [!NOTE] 例外情况
>
> - 你拥有正版 Minecraft
> - 你的正版 ID 就是那个被他人占用的角色名
>
> 这种情况下，你可以通过向管理员证明你来强制获取这个角色的所有权。

## <Badge type="warning" text="关键" /> 无法访问 BYS 网站 {#server-down}

你可能在尝试访问 BYS 网站时遇到了类似于以下报错：

- 503 Service Temporarily Unavailable
- 502 Bad Gateway
- Error 525, SSL handshake failed
- Service Unavailable

当你遇到这类情况时，请稍安勿躁。

我们已在第一时间通过自动监测程序即时发现了问题，并正在全力以赴解决中。

我们的紧迫感与你同样强烈，都在期待着尽快恢复正常服务。

<Helpme>若在等待一段时间后依旧无法访问 BYS 网站，你可以通过以下方式获取最新的进展情况和相互支持。</Helpme>

## 注册账号时失败 {#ip-limit}

若在注册账号时出现含有以下关键词的报错，则说明你的 IP 地址上注册的用户数已达上限：

- **【IPL】**
- 注册 IP 达到上限
- 网络环境异常，请使用移动网络注册

导致此问题的原因较为复杂，但解决方案很简单。

🛠️ 使用移动通讯网络（如流量）进行注册即可。

## 无法正常显示某些页面 / 图片？ {#broken-webpage}

大多数情况下，这是由你的浏览器导致的。

请先尝试<mark>按下键盘上的 <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>R</kbd></mark>（即清除本地缓存并刷新页面），如果问题仍然存在，则说明你的浏览器过时了。

很多过时的浏览器不支持 BYS 使用的一些新技术，而你可能就正在使用这些过时的浏览器。你需要<mark>更新你的浏览器至最新版本</mark>。然而，有些浏览器即使更新到最新版本，其使用的浏览器内核仍然是过时的。

经测试，BYS 在以下浏览器的最新版本上都能正常工作：

- [Microsoft Edge](https://aka.ms/msedge) <Badge type="info" text="仅新版" />
- [Google Chrome](https://www.google.cn/chrome)
- [Mozilla Firefox](https://www.mozilla.org/firefox/new)

在极少数情况下，你使用的设备的显卡不支持 BYS 页面中的使用的技术（如 WebGL）也会导致此问题。你可能需要更新显卡驱动或更换至新的显卡才能彻底解决。

<Helpme>如果你还是没有什么头绪，请向管理员反馈。</Helpme>

## 不是有效的披风文件 {#invalid-cape}

BYS 对材质格式有着严格的要求，请按照 Minecraft 官方的标准来绘制披风。

此外，BYS 不支持 22\*17 格式的披风，请自行将披风转换为 64\*32 格式后再上传。

## 发生严重错误 {#server-error}

如果你在使用 BYS 时遇到了红色的 <BSButton style="background-color: #dc3545;">严重错误</BSButton> 弹窗，别着急。

错误可能是临时性的，你可以稍等一会儿再试试看。

<Helpme>如果长时间等待后错误仍未解决，请将其截图，联系站点管理员处理。</Helpme>

## 我被封禁了，可以解封吗？ {#banned}

不可以。封禁是永久性的，<mark>除非是误封</mark>，就算你大额捐助(可能？)并且手写书面保证书也不能给你解封。

<Helpme>如果你确定是误封，请联系站点管理员处理。</Helpme>
