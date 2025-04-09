document.addEventListener('DOMContentLoaded', function() {
    // スムーズスクロール機能
    const smoothScrollLinks = document.querySelectorAll('a[href^="#"]');
    
    for (const link of smoothScrollLinks) {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            const targetId = this.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                const offsetTop = targetElement.getBoundingClientRect().top + window.pageYOffset;
                
                window.scrollTo({
                    top: offsetTop,
                    behavior: 'smooth'
                });
            }
        });
    }
    
    // ナビゲーションのアクティブクラス制御
    const sections = document.querySelectorAll('section[id]');
    
    window.addEventListener('scroll', function() {
        const scrollPosition = window.scrollY + 200;
        
        for (const section of sections) {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            const sectionId = section.getAttribute('id');
            
            if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
                const navLinks = document.querySelectorAll('nav ul li a');
                
                for (const link of navLinks) {
                    if (link.getAttribute('href').includes(sectionId)) {
                        document.querySelectorAll('nav ul li a').forEach(el => el.classList.remove('active'));
                        link.classList.add('active');
                    }
                }
            }
        } });
    });
    
    // メンバー作品データ
    const memberWorks = [
        {
            title: "夜の夜まで",
            youtubeId: "JojdedXfldE",
            creator: "五月一日",
            description: "前々から作ってみたかったエレクトロスウィングです。ベースソロがお気に入りです。"
        },
        {
            title: "この感情には後で名前を付けます。",
            youtubeId: "MLtIQKxG0fU",
            creator: "五月一日",
            description: "感情が孤独で大切だからこそ、誰か大切な人やモノが失われることは、「その人やモノの感情や考えている唯一無二のモノが失われること」と同義で、とても辛いことだと思っています。亡くなった愛犬を想って、冷たい秋の風の中に書いた歌です"
        },
        {
            title: "大人に",
            youtubeId: "JmPd8G_9n2o",
            creator: "Auauo",
            description: "苦しいよ♪苦しいよ♪苦しい♪"
        },
        {
            title: "バレてたらどうしよう",
            youtubeId: "BAr50BKIQ8Y",
            creator: "Auauo",
            description: "なんか知らないけど一番伸びてるやつ！なんで？"
        },
        {
            title: "深海",
            youtubeId: "etUgny3nVrI",
            creator: "変曲点",
            description: "初音ミクオリジナル曲です。海の中をイメージして曲を作りました。"
        },
        {
            title: "神の嗜好品",
            youtubeId: "tEVnFeV-vwo",
            creator: "変曲点",
            description: "鏡音リンオリジナル曲です。シンセサイザーの電子音などを使って曲を作りました。"
        },
        {
            title: "退化 / 知声",
            youtubeId: "oBVR_R-_XTE",
            creator: "半防隊",
            description: "想像が退化！！"
        },
        {
            title: "3y3 / 知声",
            youtubeId: "nKL9FC0sVuo",
            creator: "半防隊",
            description: "👁️👁️"
        },
        {
            title: "三森 - 箱庭の空 (feat.可不)",
            youtubeId: "iQdH-3h6oXY",
            creator: "三森",
            description: "決意と共に歩いていくような冒険ソングです。"
        },
        {
            title: "三森 - 盲目少女と錬金術師 (feat.初音ミク)",
            youtubeId: "UCpbXytJIA4",
            creator: "三森",
            description: "楽しくも切ないおとぎ話のようなサウンドを目指して作りました。"
        }
    ];
    
    // YouTube APIの読み込み
    loadYouTubeAPI();
    
    function loadYouTubeAPI() {
        if (document.getElementById('featured-work-container')) {
            const tag = document.createElement('script');
            tag.src = "https://www.youtube.com/iframe_api";
            const firstScriptTag = document.getElementsByTagName('script')[0];
            firstScriptTag.parentNode.insertBefore(tag, firstScriptTag);
            
            // YouTube APIが読み込まれたら実行される関数をグローバルに定義
            window.onYouTubeIframeAPIReady = function() {
                displayRandomWork();
            };
        }
    }
    
    function displayRandomWork() {
        // ランダムな作品を選択
        const randomIndex = Math.floor(Math.random() * memberWorks.length);
        const selectedWork = memberWorks[randomIndex];
        
        // 情報を表示
        const titleElement = document.getElementById('featured-work-title');
        const creatorElement = document.getElementById('featured-work-creator');
        const descriptionElement = document.getElementById('featured-work-description');
        
        if (titleElement && creatorElement && descriptionElement) {
            titleElement.textContent = selectedWork.title;
            creatorElement.textContent = selectedWork.creator;
            descriptionElement.textContent = selectedWork.description;
            
            // YouTube動画を埋め込み
            new YT.Player('youtube-player', {
                height: '360',
                width: '640',
                videoId: selectedWork.youtubeId,
                playerVars: {
                    'autoplay': 1,  // 自動再生を有効化
                    'mute': 1,      // ミュートを有効化（自動再生に必要）
                    'controls': 1,
                    'rel': 0,
                    'fs': 1
                }
            });
        }
    }
    
    // プレースホルダー画像のテキスト表示
    const imgPlaceholders = document.querySelectorAll('.img-placeholder');
    
    for (const placeholder of imgPlaceholders) {
        if (!placeholder.textContent) {
            placeholder.textContent = '画像準備中';
        }
    }
;
