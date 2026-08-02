# 🎙️ Podcast Summaries Index

> 需要 Dataview 插件。打开 Obsidian 后如果没生效，等几秒或重启 Obsidian。

---

## 📊 统计概览

```dataviewjs
let episodes = dv.pages()
  .where(p => p.file.folder != "" && p.show && !p.file.name.startsWith("_"))
  .sort(p => p.date, "desc");

let dailyReports = dv.pages()
  .where(p => p.file.name.startsWith("_daily_report"));

let shows = new Set(episodes.map(p => p.show));
let guests = new Set(episodes.map(p => p.guest).filter(g => g));

let oldest = episodes.length > 0 ? episodes[episodes.length-1].date : "—";
let newest = episodes.length > 0 ? episodes[0].date : "—";

dv.paragraph(`- 🎧 **播客笔记：** ${episodes.length} 条`);
dv.paragraph(`- 📻 **播客节目：** ${shows.size} 个`);
dv.paragraph(`- 🎙️ **嘉宾：** ${guests.size} 位`);
dv.paragraph(`- 📅 **时间跨度：** ${oldest} ~ ${newest}`);
dv.paragraph(`- 📋 **每日报告：** ${dailyReports.length} 份`);
```

---

## 🎧 最新笔记

```dataview
TABLE show as "节目", guest as "嘉宾", duration as "时长", date as "日期"
FROM ""
WHERE show AND !contains(file.name, "_daily_report")
SORT date DESC
LIMIT 30
```

---

## 📻 按节目浏览

```dataview
TABLE rows.file.link as "单集", rows.guest as "嘉宾"
FROM ""
WHERE show AND !contains(file.name, "_daily_report")
GROUP BY show as "节目"
SORT rows.file.name ASC
```

---

## 🎙️ 按嘉宾搜索

```dataview
TABLE show as "节目", date as "日期", duration as "时长"
FROM ""
WHERE guest AND !contains(file.name, "_daily_report")
SORT guest ASC
```

---

## 🏷️ 热门话题

```dataview
TABLE length(rows) as "提及次数"
FROM ""
WHERE topics AND !contains(file.name, "_daily_report")
FLATTEN topics as topic
GROUP BY topic
SORT length(rows) DESC
LIMIT 20
```

---

## 📅 每日报告

```dataview
TABLE dateformat(file.mtime, "yyyy-MM-dd HH:mm") as "生成时间"
FROM ""
WHERE contains(file.name, "_daily_report")
SORT file.name DESC
```

---

## ⏱️ 时长分布

```dataview
TABLE rows.file.link as "单集"
FROM ""
WHERE duration AND !contains(file.name, "_daily_report")
GROUP BY duration as "时长"
SORT length(rows) DESC
```
