# 公式设计

## 0.输出

输出暂定只有 Error，Number，String，Bool，Date，TimeSpan 这些在输入完公式后就能知道的
然后针对 Number 可以让用户选择四舍五入，向上取整，向下取整，保留几位小数
针对 Date，TimeSpan 可以让用户选择格式化输出，否则默认输出为 ISO 格式

后续会考虑输出数组，暂时可以用 join 函数来实现

## 0.1 执行方法

针对单个 Issue，就可以直接 wasm 执行，然后返回结果，只需要传递一个 Issue 的信息。
但是考虑到以后可能需要对多个Issue、整个Issue表查询，所以可以考虑输出 SQL 语句，然后由后端执行。在前期就需要考虑 SQL 的兼容性问题。不过为了扩展性，感觉编译成 SQL 不是特别可取，那样掣肘太多,考虑编译成 wasm 或者 二进制库，用数据库的 UDF 功能去执行。

还有为了以后最大的运行时扩展性，要不要做成能使用 js 兼容的样子，那样可以用 quickjs 来跑，当然这样的话对语法的扩展性就不太友好了。例如 `$.xxx` 和 `1.day` 这种语法，就不能用了。

## 1.数字计算

```ts
1+ 1
2 - 1
1 + 2 + 3 + 4
1+2+3+4
1+2*3/4
(1+2)*3/4
(-1+2)*3/4
(1-2)*-3/4
(1+(-2)+(3 * 4)) + 5 
2 ^ 10
2 ^ -1
2 * -1
2 + 2!
```

## 2.函数

```ts
count(relationship)
count(where(relationship, $.issueType == 1848788))
count(where(relationship, $.issueType == 1848788 && $.status == 1))

// 下面这种也可以考虑加上，不过没前面看着优雅
countIf(relationship, $.issueType == 1848788)
countWhere(relationship, $.issueType == 1848788)

sum(subtask, $.estimatePoint)
sum(where(subtask, $.state == 2), $.estimatePoint)

avg(subtask, $.estimatePoint)

// 提取属性
subtask.pluck($.estimatePoint).join(',')
subtask.pluck($.name).map('subtask' + $.name).join(',')
subtask.take(10).pluck($.name).map(upper).join(',')
```

## 3.日期

```ts
days(now() - updateTime)
hours(now() - updateTime)

(now() - updateTime).days
(now() - updateTime).hours


```

```ts
// 日期计算
createTime + 1.day + 2.hour - 3.minute

// 上面借鉴自 swift 的语法，我不太确定上面的好不好实现，下面这种简单点

addDay(createTime, 1)
addHour(createTime, 2)

createTime.toString()
createTime.toString('YYYY-MM-DD HH:mm:ss')

max(subtask, $.updateTime).name

orderBy(subtask, $.updateTime, desc).take(10).pluck($.name).join(',')
orderBy(subtask, $.updateTime, asc)
```
