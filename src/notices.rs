use rocket::serde::json::{Value, Json, json};
use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    chatNotices: Vec<String>,
    popupNotices: Vec<String>,
    topicsNotices: Vec<Notice>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notice { 
    bannerUrl: String,
    enabledEnglish: bool,
    enabledFrench: bool,
    enabledGerman: bool,
    enabledJapanese: bool,
    enabledKorean: bool,
    enabledSimplifiedChinese: bool,
    enabledTraditionalChinese: bool,
    msgEnglish: String,
    msgFrench: String,
    msgGerman: String,
    msgJapanese: Value,
    msgKorean: String,
    msgSimplifiedChinese: String,
    msgTraditionalChinese: String,
    noticeCode: String,
    priority: usize,
    redirectUI: Value,
    subtitleEnglish: String,
    subtitleFrench: String,
    subtitleGerman: String,
    subtitleJapanese: Value,
    subtitleKorean: String,
    subtitleSimplifiedChinese: String,
    subtitleTraditionalChinese: String,
    target: usize,
    titleEnglish: String,
    titleFrench: String,
    titleGerman: String,
    titleJapanese: Value,
    titleKorean: String,
    titleSimplifiedChinese: String,
    titleTraditionalChinese: String,
    topicsUrl: String
}

#[post("/OpsNotice/GetNotices", format = "json")]
pub fn notices() -> Json<Ressa> {
    Json(Ressa {
        chatNotices: vec![],
        popupNotices: vec![],
        topicsNotices: vec![
            Notice {
                bannerUrl: "https://media.gundamevogame.com/topics/n4XaHOTMV78WnZIlaNcrOg.jpg".to_string(),
                enabledEnglish: true,
                enabledFrench: true,
                enabledGerman: true,
                enabledJapanese: false,
                enabledKorean: true,
                enabledSimplifiedChinese: true,
                enabledTraditionalChinese: true,
                msgEnglish: "GUNDAM EVOLUTION's service will end on November 29, 2023. \r\nView the official website for details.".to_string(),                
                msgFrench: "Le service de GUNDAM EVOLUTION prendra fin le 30 novembre 2023. \r\nVeuillez consulter le site officiel    pour plus de détails.".to_string(),
                msgGerman: "Der GUNDAM EVOLUTION-Service wird am 30. November 2023 eingestellt. \r\nWeitere Details findet ihr auf der offiziellen Website.".to_string(),
                msgJapanese: json!(null),
                msgKorean: "GUNDAM EVOLUTION」 서비스가 2023년 11월 30일에 종료됩니다.\r\n 자세한 내용은 공식 사이트를 확인하세요.".to_string(),             
                msgSimplifiedChinese: "《GUNDAM EVOLUTION》将在2023年11月30日停止运营。\r\n详情请在官方网站进行确认。".to_string(),
                msgTraditionalChinese: "《GUNDAM EVOLUTION》將於2023年11月30日結束營運。\r\n詳情請至官方網站確認。".to_string(),
                noticeCode: "257_3".to_string(),
                priority: 1,
                redirectUI: json!(null),
                subtitleEnglish: "Important Announcement for GUNDAM EVOLUTION Players".to_string(),
                subtitleFrench: "Annonce importante à destination des joueurs de GUNDAM EVOLUTION".to_string(),
                subtitleGerman: "Wichtige Ankündigung für Spieler von GUNDAM EVOLUTION".to_string(),
                subtitleJapanese: json!(null),
                subtitleKorean: "(중요) 「GUNDAM EVOLUTION」 플레이어 여러분께 알려드립니다".to_string(),
                subtitleSimplifiedChinese: "致《GUNDAM EVOLUTION》各位玩家的重要公告".to_string(),
                subtitleTraditionalChinese: "給《GUNDAM EVOLUTION》各位玩家的重要通知".to_string(),
                target: 1,
                titleEnglish: "Important Announcement".to_string(),
                titleFrench: "Annonce importante".to_string(),
                titleGerman: "Wichtige Ankündigung".to_string(),
                titleJapanese: json!(null),
                titleKorean: "알려드립니다".to_string(),
                titleSimplifiedChinese: "重要公告".to_string(),
                titleTraditionalChinese: "重要通知".to_string(),
                topicsUrl: "https://media.gundamevogame.com/topics/kiT1hNivd9OeI8Cd1BsaAg.jpg".to_string()
            },

            Notice {
                bannerUrl: "https://media.gundamevogame.com/topics/-mXNyPUi-gtTlDT7jbgMjw.png".to_string(),
                enabledEnglish: true,
                enabledFrench: true,
                enabledGerman: true,
                enabledJapanese: false,
                enabledKorean: true,
                enabledSimplifiedChinese: true,
                enabledTraditionalChinese: true,
                msgEnglish: "See patch notes for details on event period and changes for content.".to_string(),                
                msgFrench: "Pour plus de détails sur la période d'événement et les changements apportés, \r\nlisez les notes de version.".to_string(),
                msgGerman: "Genauere Informationen zum Eventzeitraum und \r\nzu den Inhaltsänderungen sind in den Versionshinweisen zu finden.".to_string(),
                msgJapanese: json!(null),
                msgKorean: "각 콘텐츠의 자세한 개최 기간 및 갱신 내용은 패치 노트를 확인하세요.".to_string(),             
                msgSimplifiedChinese: "各项内容的活动时间及更新内容的详情，请查看补丁说明。".to_string(),
                msgTraditionalChinese: "關於各內容的活動期間與更新內容，可於補丁說明中查看。".to_string(),
                noticeCode: "261_1".to_string(),
                priority: 2,
                redirectUI: json!(null),
                subtitleEnglish: "Season 5 STORM: July Update Released".to_string(),
                subtitleFrench: "Saison 5 STORM : mise à jour juillet disponible".to_string(),
                subtitleGerman: "Saison 5 „STORM“: Juli-Update veröffentlicht".to_string(),
                subtitleJapanese: json!(null),
                subtitleKorean: "「Season 5 STORM」 7월 업데이트 공개 중".to_string(),
                subtitleSimplifiedChinese: "“Season 5 STORM”7月更新现已公开".to_string(),
                subtitleTraditionalChinese: "「Season 5 STORM」7月更新公開中".to_string(),
                target: 1,
                titleEnglish: "\"Season 5 STORM\" Patch Notes".to_string(),
                titleFrench: "Saison 5 STORM : notes de version".to_string(),
                titleGerman: "Saison 5 „STORM“: Versionshinweise".to_string(),
                titleJapanese: json!(null),
                titleKorean: "「Season 5 STORM」 패치 노트".to_string(),
                titleSimplifiedChinese: "“Season 5 STORM”补丁说明".to_string(),
                titleTraditionalChinese: "「Season 5 STORM」補丁說明".to_string(),
                topicsUrl: "https://media.gundamevogame.com/topics/oGt7gFzEhdzIwija5ZKDrQ.png".to_string()
            },
        ]
    })
}