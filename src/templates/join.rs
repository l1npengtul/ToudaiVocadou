use crate::{
    SiteData, image, lnk,
    metadata::Metadata,
    templates::{base::base, partials::navbar::Sections},
};
use hauchiwa::{Context, RuntimeError};
use maud::{Markup, html};

pub fn join(sack: &Context<SiteData>) -> Result<Markup, RuntimeError> {
    let meta = Metadata {
        page_title: "入会希望者へ - Joining Vocaloid Producer Club".to_string(),
        page_image: Some(lnk(image(sack, "images/circle-photo.jpg")?)),
        canonical_link: lnk(format!("join.html")),
        section: Sections::Join,
        author: None,
        date: None,
        description: None,
    };

    let content = html! {
        section #hero {
            .container {
                h2 { "ボカロP同好会、入会しよう。" }
                p { "ボーカロイド楽曲の制作を通じて交流するサークルです。" }
                a href="#join" .btn { "入会案内" }
            }
        }

        section #join {
            .container {
                h2 { "入会案内" }
                .join-info {
                    p { "東京大学の学生であれば、学部・学年を問わず入会できます。音楽制作の経験がなくても大歓迎です！" }
                    p { "入会を希望される方は、下記のXアカウントまでご連絡ください。" }
                    p .contact-email {
                        a href="https://twitter.com/toudaivocadou/" {
                            "@toudaivocadou"
                        }
                    }
                    p { "または、新歓期間中の説明会にお越しください。" }
                    .join-details {
                        h3 { "説明会情報" }
                        p { "日時: 4月12日 18:00〜18:30" }
                        p { "説明会の参加方法に関しましては、公式Xアカウントで随時お知らせいたします。" }
                        p { "また、日時に関しても変更される場合がありますので、公式Xアカウントからの情報を随時ご確認ください。" }
                    }
                }
            }
        }
    };
    base(sack, &meta, None, content)
}
